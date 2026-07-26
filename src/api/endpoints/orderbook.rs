use crate::api::client::MercadoBitcoinClient;
use crate::api::models::OrderBook;
use crate::error::Result;

impl MercadoBitcoinClient {
    /// Obtém o orderbook (livro de ofertas) de uma criptomoeda
    /// 
    /// # Exemplos
    /// ```
    /// let orderbook = client.get_orderbook("BTC").await?;
    /// println!("Maiores ofertas de compra: {:?}", orderbook.bids);
    /// ```
    pub async fn get_orderbook(&self, coin: &str) -> Result<OrderBook> {
        let endpoint = format!("/v4/{}/orderbook", coin.to_lowercase());
        log::info!("Buscando orderbook para: {}", coin);
        self.get(&endpoint).await
    }

    /// Obtém orderbook para Bitcoin (BTC)
    pub async fn get_btc_orderbook(&self) -> Result<OrderBook> {
        self.get_orderbook("BTC").await
    }

    /// Obtém orderbook para Ethereum (ETH)
    pub async fn get_eth_orderbook(&self) -> Result<OrderBook> {
        self.get_orderbook("ETH").await
    }
}
