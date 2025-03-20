use crate::error::{AppError, Result};
use homedir::my_home;
use serde::Deserialize;
use std::str::FromStr;

/// encapsulation of the application configuration
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct FormatterConfig {
    pub decimal_separator: char,
    pub thousands_separator: char,
    pub currencies: Vec<AmountCommodityPairConfig>,
}

/// configuration options of an amount-commodity pair
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct AmountCommodityPairConfig {
    pub symbol: String,
    pub decimals: u8,
    pub suppress_thousands_separator: Option<bool>,
}

impl FormatterConfig {
    pub fn path() -> Result<std::path::PathBuf> {
        let env_path = std::env::var("CURRENCYFMT_CONFIG");
        match env_path {
            Ok(env) => match std::path::PathBuf::from_str(&env) {
                Ok(path) => Ok(path),
                Err(_) => Err(AppError::ConfigPath),
            },
            Err(_) => match my_home() {
                Ok(home) => match home {
                    Some(home) => {
                        let mut path = home.into_os_string();
                        path.push("/.config/currencyfmt/config.toml");
                        Ok(path.into())
                    }
                    None => Err(AppError::ConfigPath),
                },
                Err(_) => Err(AppError::ConfigPath),
            },
        }
    }

    pub fn load(preferred_path: Option<std::path::PathBuf>) -> Result<Self> {
        let path = preferred_path.unwrap_or(Self::path()?);
        let config_str = std::fs::read_to_string(&path)?;
        toml::from_str::<FormatterConfig>(&config_str).map_err(AppError::Parse)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn simple_config_from_toml_str() {
        let config_str = "decimal_separator = \",\"
        thousands_separator = \".\"
        currencies = []
        "
        .to_owned();
        let expected = FormatterConfig {
            decimal_separator: ',',
            thousands_separator: '.',
            currencies: vec![],
        };
        let result = toml::from_str::<FormatterConfig>(&config_str).expect("TOML parsing failed");
        assert_eq!(result, expected);
    }

    #[test]
    fn full_config_from_toml_str() {
        let config_str = "decimal_separator = \",\"
        thousands_separator = \".\"
        
        [[currencies]]
        symbol = \"EUR\"
        decimals = 2

        [[currencies]]
        symbol = \"BTC\"
        decimals = 8
        suppress_thousands_separator = true
        "
        .to_owned();
        let expected = FormatterConfig {
            decimal_separator: ',',
            thousands_separator: '.',
            currencies: vec![
                AmountCommodityPairConfig {
                    symbol: "EUR".to_owned(),
                    decimals: 2,
                    suppress_thousands_separator: None,
                },
                AmountCommodityPairConfig {
                    symbol: "BTC".to_owned(),
                    decimals: 8,
                    suppress_thousands_separator: Some(true),
                },
            ],
        };
        let result = toml::from_str::<FormatterConfig>(&config_str).expect("TOML parsing failed");
        assert_eq!(result, expected);
    }

    #[test]
    fn invalid_config_from_toml_str() {
        let config_str = "decimal_separator = false".to_owned();
        let result = toml::from_str::<FormatterConfig>(&config_str);
        assert!(result.is_err());
    }
}
