use crate::config::Config;
use crate::error::{MbcError, Result};
use reqwest::Client;
use std::time::Duration;

pub struct MercadoBitcoinClient {
    http_client: Client,
    config: Config,
}

impl MercadoBitcoinClient {
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;

        let http_client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(|e| MbcError::RequestError(e))?;

        Ok(Self {
            http_client,
            config,
        })
    }

    /// Faz uma requisição GET para a API
    pub async fn get<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> Result<T> {
        let url = format!("{}{}", self.config.base_url, endpoint);
        
        log::debug!("GET request to: {}", url);

        let response = self.http_client
            .get(&url)
            .send()
            .await
            .map_err(MbcError::RequestError)?;

        let status = response.status();

        if !status.is_success() {
            let message = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(MbcError::ApiError {
                status_code: status.as_u16(),
                message,
            });
        }

        let data = response
            .json::<T>()
            .await
            .map_err(MbcError::RequestError)?;

        Ok(data)
    }

    /// Faz uma requisição POST para a API
    pub async fn post<T: serde::de::DeserializeOwned, B: serde::Serialize>(
        &self,
        endpoint: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}{}", self.config.base_url, endpoint);
        
        log::debug!("POST request to: {}", url);

        let response = self.http_client
            .post(&url)
            .json(body)
            .send()
            .await
            .map_err(MbcError::RequestError)?;

        let status = response.status();

        if !status.is_success() {
            let message = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(MbcError::ApiError {
                status_code: status.as_u16(),
                message,
            });
        }

        let data = response
            .json::<T>()
            .await
            .map_err(MbcError::RequestError)?;

        Ok(data)
    }

    /// Retorna a URL base da API
    pub fn base_url(&self) -> &str {
        &self.config.base_url
    }
}
