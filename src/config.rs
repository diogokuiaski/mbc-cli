use crate::error::{MbcError, Result};

#[derive(Clone, Debug)]
pub struct Config {
    pub base_url: String,
    pub timeout_seconds: u64,
    pub api_key: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        let base_url = std::env::var("MBC_BASE_URL")
            .unwrap_or_else(|_| "https://www.mercadobitcoin.com.br/api".to_string());

        let timeout_seconds = std::env::var("MBC_TIMEOUT")
            .unwrap_or_else(|_| "30".to_string())
            .parse()
            .unwrap_or(30);

        let api_key = std::env::var("MBC_API_KEY").ok();

        Config {
            base_url,
            timeout_seconds,
            api_key,
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.base_url.is_empty() {
            return Err(MbcError::ConfigError(
                "base_url não pode estar vazio".to_string(),
            ));
        }

        if self.timeout_seconds == 0 {
            return Err(MbcError::ConfigError(
                "timeout_seconds deve ser maior que 0".to_string(),
            ));
        }

        Ok(())
    }
}
