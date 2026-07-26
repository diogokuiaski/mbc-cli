use crate::models::TickerData;
use serde::{Deserialize, Serialize};

/// Trading strategy signals
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Signal {
    Buy,
    Sell,
    Hold,
}

/// Strategy parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyParams {
    /// RSI period (default: 14)
    pub rsi_period: usize,
    /// RSI oversold threshold (default: 30)
    pub rsi_oversold: f64,
    /// RSI overbought threshold (default: 70)
    pub rsi_overbought: f64,
    /// EMA short period (default: 12)
    pub ema_short: usize,
    /// EMA long period (default: 26)
    pub ema_long: usize,
    /// Price change threshold for buy signal (default: 0.02 = 2%)
    pub price_change_threshold: f64,
}

impl Default for StrategyParams {
    fn default() -> Self {
        Self {
            rsi_period: 14,
            rsi_oversold: 30.0,
            rsi_overbought: 70.0,
            ema_short: 12,
            ema_long: 26,
            price_change_threshold: 0.02,
        }
    }
}

/// Trading strategy analyzer
pub struct TradingStrategy {
    params: StrategyParams,
    price_history: Vec<f64>,
}

impl TradingStrategy {
    pub fn new(params: StrategyParams) -> Self {
        Self {
            params,
            price_history: Vec::new(),
        }
    }

    /// Add price to history
    pub fn add_price(&mut self, price: f64) {
        self.price_history.push(price);
    }

    /// Analyze ticker and generate trading signal
    pub fn analyze(&mut self, ticker: &TickerData) -> Signal {
        let last_price = ticker.last.parse::<f64>().unwrap_or(0.0);
        let high_price = ticker.high.parse::<f64>().unwrap_or(0.0);
        let low_price = ticker.low.parse::<f64>().unwrap_or(0.0);
        let buy_price = ticker.buy.parse::<f64>().unwrap_or(0.0);
        let sell_price = ticker.sell.parse::<f64>().unwrap_or(0.0);

        self.add_price(last_price);

        // Check for buy signal
        if self.should_buy(last_price, high_price, low_price, buy_price) {
            return Signal::Buy;
        }

        // Check for sell signal
        if self.should_sell(last_price, high_price, low_price, sell_price) {
            return Signal::Sell;
        }

        Signal::Hold
    }

    /// Buy signal logic using RSI and price action
    fn should_buy(&self, last: f64, high: f64, low: f64, buy: f64) -> bool {
        if self.price_history.len() < self.params.rsi_period {
            return false;
        }

        let rsi = self.calculate_rsi();
        let price_from_low = (last - low) / low;
        let price_bounce = price_from_low > self.params.price_change_threshold;

        // Buy when RSI is oversold AND price bounces from low
        rsi < self.params.rsi_oversold && price_bounce && last < buy
    }

    /// Sell signal logic using RSI and price action
    fn should_sell(&self, last: f64, high: f64, low: f64, sell: f64) -> bool {
        if self.price_history.len() < self.params.rsi_period {
            return false;
        }

        let rsi = self.calculate_rsi();
        let price_from_high = (high - last) / high;
        let price_pullback = price_from_high > self.params.price_change_threshold;

        // Sell when RSI is overbought AND price pulls back from high
        rsi > self.params.rsi_overbought && price_pullback && last > sell
    }

    /// Calculate RSI (Relative Strength Index)
    fn calculate_rsi(&self) -> f64 {
        if self.price_history.len() < self.params.rsi_period {
            return 50.0; // Neutral
        }

        let period = self.params.rsi_period;
        let prices = &self.price_history[self.price_history.len() - period..];

        let mut gains = 0.0;
        let mut losses = 0.0;

        for i in 1..prices.len() {
            let change = prices[i] - prices[i - 1];
            if change > 0.0 {
                gains += change;
            } else {
                losses += change.abs();
            }
        }

        let avg_gain = gains / period as f64;
        let avg_loss = losses / period as f64;

        if avg_loss == 0.0 {
            return 100.0;
        }

        let rs = avg_gain / avg_loss;
        100.0 - (100.0 / (1.0 + rs))
    }

    /// Calculate EMA (Exponential Moving Average)
    pub fn calculate_ema(&self, period: usize) -> f64 {
        if self.price_history.len() < period {
            return 0.0;
        }

        let prices = &self.price_history[self.price_history.len() - period..];
        let multiplier = 2.0 / (period as f64 + 1.0);
        let mut ema = prices[0];

        for i in 1..prices.len() {
            ema = (prices[i] * multiplier) + (ema * (1.0 - multiplier));
        }

        ema
    }

    /// Get current RSI value
    pub fn get_rsi(&self) -> f64 {
        self.calculate_rsi()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_creation() {
        let strategy = TradingStrategy::new(StrategyParams::default());
        assert_eq!(strategy.price_history.len(), 0);
    }

    #[test]
    fn test_price_history() {
        let mut strategy = TradingStrategy::new(StrategyParams::default());
        strategy.add_price(50000.0);
        strategy.add_price(51000.0);
        assert_eq!(strategy.price_history.len(), 2);
    }
}
