use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    Buy,
    Sell,
    Hold,
}

#[derive(Debug, Clone)]
pub struct StrategyParams {
    pub rsi_period: usize,
    pub rsi_overbought: f64,
    pub rsi_oversold: f64,
}

impl Default for StrategyParams {
    fn default() -> Self {
        Self {
            rsi_period: 14,
            rsi_overbought: 70.0,
            rsi_oversold: 30.0,
        }
    }
}

pub struct TradingStrategy {
    params: StrategyParams,
    prices: VecDeque<f64>,
    rsi: f64,
}

impl TradingStrategy {
    pub fn new(params: StrategyParams) -> Self {
        Self {
            params,
            prices: VecDeque::new(),
            rsi: 50.0,
        }
    }

    pub fn analyze(&mut self, price: f64) -> Signal {
        self.prices.push_back(price);
        
        if self.prices.len() > self.params.rsi_period {
            self.prices.pop_front();
        }

        self.calculate_rsi();

        if self.rsi > self.params.rsi_overbought {
            Signal::Sell
        } else if self.rsi < self.params.rsi_oversold {
            Signal::Buy
        } else {
            Signal::Hold
        }
    }

    fn calculate_rsi(&mut self) {
        if self.prices.len() < 2 {
            return;
        }

        let mut gains = 0.0;
        let mut losses = 0.0;

        for i in 1..self.prices.len() {
            let change = self.prices[i] - self.prices[i - 1];
            if change > 0.0 {
                gains += change;
            } else {
                losses += -change;
            }
        }

        let avg_gain = gains / self.prices.len() as f64;
        let avg_loss = losses / self.prices.len() as f64;

        if avg_loss == 0.0 {
            self.rsi = 100.0;
        } else {
            let rs = avg_gain / avg_loss;
            self.rsi = 100.0 - (100.0 / (1.0 + rs));
        }
    }

    pub fn get_rsi(&self) -> f64 {
        self.rsi
    }
}
