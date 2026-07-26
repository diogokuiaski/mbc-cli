use crate::api::client::MercadoBitcoinClient;
use crate::api::models::TickerResponse;
use crate::error::Result;

impl MercadoBitcoinClient {
    /// Obtém informações de ticker de uma criptomoeda
    /// 
    /// # Exemplos
    /// ```
    /// let ticker = client.get_ticker("BTC").await?;
    /// println!("Preço BTC: {}", ticker.ticker.last);
    /// ```
    pub async fn get_ticker(&self, coin: &str) -> Result<TickerResponse> {
        let endpoint = format!("/v4/{}/ticker", coin.to_lowercase());
        log::info!("Buscando ticker para: {}", coin);
        self.get(&endpoint).await
    }

    /// Obtém ticker para Bitcoin (BTC)
    pub async fn get_btc_ticker(&self) -> Result<TickerResponse> {
        self.get_ticker("BTC").await
    }

    /// Obtém ticker para Ethereum (ETH)
    pub async fn get_eth_ticker(&self) -> Result<TickerResponse> {
        self.get_ticker("ETH").await
    }
}
