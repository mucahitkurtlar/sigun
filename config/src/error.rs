use thiserror::Error;

#[derive(Error, Clone, Debug, PartialEq)]
pub enum ConfigError {
    #[error("failed to read the configuration file: {0}")]
    ReadFailed(String),
    #[error("failed to parse the configuration file")]
    ParseFailed,
}
