use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(
        "Failed to provide the path to the configruation file. Please provide the path to the configuration file in the environment variable \"CURRENCYFMT_CONFIG\" to fix this error."
    )]
    ConfigPath,
    #[error("Failed to read the configuration file: {0}")]
    ConfigRead(#[from] std::io::Error),
    #[error("Failed to parse the configuration file: {0}")]
    Parse(#[from] toml::de::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
