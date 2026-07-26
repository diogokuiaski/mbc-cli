use serde::{Deserialize, Serialize};

/// Modelo para dados de ticker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    pub high: String,
    pub low: String,
    pub vol: String,
    pub last: String,
    pub buy: String,
    pub sell: String,
    pub open: String,
    pub date: u64,
}

/// Modelo para resposta de ticker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerResponse {
    pub ticker: Ticker,
}

/// Modelo para informações de ordebook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    pub timestamp: String,
    pub bids: Vec<[String; 2]>,
    pub asks: Vec<[String; 2]>,
}

/// Modelo para transações
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub tid: String,
    pub date: u64,
    pub time: String,
    pub type_str: String,
    pub price: String,
    pub amount: String,
}

/// Modelo para resposta de transações
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradesResponse {
    pub trades: Vec<Trade>,
}

/// Modelo genérico de erro da API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    pub error: String,
    pub code: Option<String>,
}
