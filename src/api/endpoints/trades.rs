use crate::api::client::MercadoBitcoinClient;
use crate::api::models::TradesResponse;
use crate::error::Result;

impl MercadoBitcoinClient {
    /// Obtém transações recentes de uma criptomoeda
    /// 
    /// # Argumentos
    /// * `coin` - Criptomoeda (ex: BTC, ETH)
    /// * `tid` - ID da transação para filtro (opcional)
    /// 
    /// # Exemplos
    /// ```
    /// let trades = client.get_trades("BTC", None).await?;
    /// for trade in trades.trades {
    ///     println!("Trade: {} {} a {}", trade.amount, trade.type_str, trade.price);
    /// }
    /// ```
    pub async fn get_trades(&self, coin: &str, tid: Option<&str>) -> Result<TradesResponse> {
        let mut endpoint = format!("/v4/{}/trades", coin.to_lowercase());
        
        if let Some(tid_value) = tid {
            endpoint.push_str(&format!("/{}", tid_value));
        }
        
        log::info!("Buscando transações para: {}", coin);
        self.get(&endpoint).await
    }

    /// Obtém transações recentes para Bitcoin (BTC)
    pub async fn get_btc_trades(&self) -> Result<TradesResponse> {
        self.get_trades("BTC", None).await
    }

    /// Obtém transações recentes para Ethereum (ETH)
    pub async fn get_eth_trades(&self) -> Result<TradesResponse> {
        self.get_trades("ETH", None).await
    }
}
