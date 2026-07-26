use clap::{Parser, Subcommand};
use anyhow::Result;
use tracing::info;

mod api;
mod models;
mod error;
mod strategy;

use api::MercadoBitcoinClient;
use strategy::{TradingStrategy, StrategyParams, Signal};

#[derive(Parser)]
#[command(name = "mbc-cli")]
#[command(about = "CLI tool for Mercado Bitcoin API access with trading strategy", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get ticker information for a cryptocurrency
    Ticker {
        /// Cryptocurrency code (e.g., BTC, ETH, LTC)
        #[arg(value_name = "COIN")]
        coin: String,
    },
    /// Get order book for a cryptocurrency
    OrderBook {
        /// Cryptocurrency code (e.g., BTC, ETH, LTC)
        #[arg(value_name = "COIN")]
        coin: String,
    },
    /// Get recent trades for a cryptocurrency
    Trades {
        /// Cryptocurrency code (e.g., BTC, ETH, LTC)
        #[arg(value_name = "COIN")]
        coin: String,
    },
    /// Analyze cryptocurrency using trading strategy
    Analyze {
        /// Cryptocurrency code (e.g., BTC, ETH, LTC)
        #[arg(value_name = "COIN")]
        coin: String,
        /// Number of ticks to analyze
        #[arg(short, long, default_value = "5")]
        ticks: u32,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let cli = Cli::parse();
    let client = MercadoBitcoinClient::new();

    match cli.command {
        Commands::Ticker { coin } => {
            info!("Fetching ticker for {}", coin);
            let ticker = client.get_ticker(&coin).await?;
            println!("{}", serde_json::to_string_pretty(&ticker)?);
        }
        Commands::OrderBook { coin } => {
            info!("Fetching order book for {}", coin);
            let order_book = client.get_order_book(&coin).await?;
            println!("{}", serde_json::to_string_pretty(&order_book)?);
        }
        Commands::Trades { coin } => {
            info!("Fetching trades for {}", coin);
            let trades = client.get_trades(&coin).await?;
            println!("{}", serde_json::to_string_pretty(&trades)?);
        }
        Commands::Analyze { coin, ticks } => {
            info!("Analyzing {} with trading strategy", coin);
            analyze_with_strategy(&client, &coin, ticks).await?;
        }
    }

    Ok(())
}

async fn analyze_with_strategy(
    client: &MercadoBitcoinClient,
    coin: &str,
    num_ticks: u32,
) -> Result<()> {
    let mut strategy = TradingStrategy::new(StrategyParams::default());
    let mut signals = Vec::new();

    println!("\n📊 Analyzing {} with trading strategy...\n", coin);
    println!("Fetching {} ticks for analysis...", num_ticks);

    for i in 0..num_ticks {
        let ticker = client.get_ticker(coin).await?;
        let ticker_data = ticker
            .get("ticker")
            .ok_or_else(|| anyhow::anyhow!("Ticker data not found"))?
            .clone();
        let ticker_obj: models::TickerData = serde_json::from_value(ticker_data)?;

        let signal = strategy.analyze(&ticker_obj);
        signals.push(signal);

        let price = ticker_obj.last.parse::<f64>().unwrap_or(0.0);
        let rsi = strategy.get_rsi();

        println!(
            "  Tick {}: Price: R$ {:.2} | RSI: {:.2} | Signal: {:?}",
            i + 1,
            price,
            rsi,
            signal
        );

        if i < num_ticks - 1 {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    }

    // Print analysis summary
    println!("\n📈 Analysis Summary:");
    println!("===================");

    let buy_count = signals.iter().filter(|s| **s == Signal::Buy).count();
    let sell_count = signals.iter().filter(|s| **s == Signal::Sell).count();
    let hold_count = signals.iter().filter(|s| **s == Signal::Hold).count();

    println!("Buy Signals:  {}", buy_count);
    println!("Sell Signals: {}", sell_count);
    println!("Hold Signals: {}", hold_count);

    // Final recommendation
    if buy_count > sell_count {
        println!("\n🟢 RECOMMENDATION: BULLISH - Consider buying");
    } else if sell_count > buy_count {
        println!("\n🔴 RECOMMENDATION: BEARISH - Consider selling");
    } else {
        println!("\n⚪ RECOMMENDATION: NEUTRAL - Hold current position");
    }

    println!("\nDisclaimer: This is for educational purposes only. Not financial advice!");

    Ok(())
}
