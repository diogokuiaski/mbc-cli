use thiserror::Error;

#[derive(Error, Debug)]
pub enum MbcError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("JSON serialization failed: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("API error: {status_code} - {message}")]
    ApiError { status_code: u16, message: String },

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("Unexpected error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, MbcError>;
