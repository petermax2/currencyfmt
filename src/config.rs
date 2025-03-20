use serde::Deserialize;

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
