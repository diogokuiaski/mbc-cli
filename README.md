# mbc-cli

CLI tool for Mercado Bitcoin API access with advanced trading strategy

## Features

- 📊 Get cryptocurrency ticker information
- 📈 Fetch order book data
- 💹 Retrieve recent trades
- 🤖 **NEW:** Automated trading strategy with buy/sell signals
- 🔧 RSI, EMA, and Price Action indicators
- ⚡ Built with Tokio for async operations
- 🔄 Uses Reqwest for HTTP requests
- 📝 Serialization with Serde JSON

## Prerequisites

- Rust 1.70 or later
- Cargo

## Installation

```bash
git clone https://github.com/diogokuiaski/mbc-cli.git
cd mbc-cli
cargo build --release
```

## Usage

### Get Ticker Information

```bash
cargo run -- ticker BTC
cargo run -- ticker ETH
```

### Get Order Book

```bash
cargo run -- orderbook BTC
cargo run -- orderbook ETH
```

### Get Recent Trades

```bash
cargo run -- trades BTC
cargo run -- trades ETH
```

### 🆕 Analyze with Trading Strategy

```bash
# Analyze BTC with 5 ticks (default)
cargo run -- analyze BTC

# Analyze with custom number of ticks
cargo run -- analyze BTC --ticks 20

# Analyze other cryptocurrencies
cargo run -- analyze ETH --ticks 10
cargo run -- analyze LTC --ticks 15
```

### Example Output

```
📊 Analyzing BTC with trading strategy...

Fetching 5 ticks for analysis...
  Tick 1: Price: R$ 180,000.00 | RSI: 45.32 | Signal: Hold
  Tick 2: Price: R$ 181,500.00 | RSI: 52.15 | Signal: Hold
  Tick 3: Price: R$ 179,200.00 | RSI: 28.64 | Signal: Buy
  Tick 4: Price: R$ 182,100.00 | RSI: 65.78 | Signal: Hold
  Tick 5: Price: R$ 184,300.00 | RSI: 72.45 | Signal: Sell

📈 Analysis Summary:
===================
Buy Signals:  1
Sell Signals: 1
Hold Signals: 3

🟢 RECOMMENDATION: BULLISH - Consider buying

Disclaimer: This is for educational purposes only. Not financial advice!
```

## Supported Cryptocurrencies

- BTC (Bitcoin)
- ETH (Ethereum)
- LTC (Litecoin)
- XRP (Ripple)
- BCH (Bitcoin Cash)
- And other cryptocurrencies supported by Mercado Bitcoin

## Trading Strategy

The CLI includes a sophisticated trading strategy that analyzes market conditions using:

1. **RSI (Relative Strength Index)**
   - Oversold threshold: < 30
   - Overbought threshold: > 70

2. **Price Action**
   - Buy when price bounces 2% from low
   - Sell when price pulls back 2% from high

3. **EMA (Exponential Moving Average)**
   - Short EMA: 12 periods
   - Long EMA: 26 periods

For detailed strategy documentation, see [TRADING_STRATEGY.md](./TRADING_STRATEGY.md)

## API Documentation

This CLI uses the Mercado Bitcoin public API. For more information, visit:
https://www.mercadobitcoin.com.br/api/

## Dependencies

- **tokio**: Asynchronous runtime
- **reqwest**: HTTP client
- **serde**: Serialization framework
- **serde_json**: JSON support
- **clap**: Command-line argument parsing
- **tracing**: Logging and diagnostics
- **anyhow**: Error handling
- **thiserror**: Error types
- **chrono**: Date and time handling

## Configuration

Create a `.env` file based on `.env.example`:

```bash
cp .env.example .env
```

Available settings:
```env
# Logging level (DEBUG, INFO, WARN, ERROR)
RUST_LOG=info
```

## Project Structure

```
mbc-cli/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── api.rs            # Mercado Bitcoin API client
│   ├── models.rs         # Data models
│   ├── strategy.rs       # Trading strategy implementation
│   └── error.rs          # Error types
├── Cargo.toml            # Project manifest
├── README.md             # This file
└── TRADING_STRATEGY.md   # Detailed strategy documentation
```

## Examples

### Scalping Strategy (2-5% daily gains)
```bash
# Monitor every hour
watch -n 3600 'cargo run -- analyze BTC --ticks 10'
```

### Swing Trading (1-3 day holds)
```bash
# Daily analysis with more data
cargo run -- analyze BTC --ticks 50
```

### Position Trading (long-term)
```bash
# Weekly analysis
cargo run -- analyze BTC --ticks 100
```

## Performance Tips

1. **Efficient Analysis**: Use `--ticks 10-20` for quick decisions
2. **Detailed Analysis**: Use `--ticks 50+` for comprehensive analysis
3. **Continuous Monitoring**: Set up a cron job for regular analysis
4. **Combine with Manual Review**: Always verify signals before trading

## Risk Management

⚠️ **IMPORTANT:** Before using the trading strategy:

1. **Start Small**: Begin with minimal capital (0.001 BTC)
2. **Test First**: Paper trade before using real money
3. **Risk Per Trade**: Never risk more than 2% of your capital
4. **Stop Loss**: Always set stop losses
5. **Take Profit**: Lock in gains at reasonable levels
6. **Diversify**: Don't put all capital in one strategy

## Disclaimer

**EDUCATIONAL PURPOSE ONLY**

- This tool is provided for educational and research purposes
- It is **NOT financial advice**
- Trading cryptocurrencies carries significant risk of loss
- You may lose some or all of your investment
- Past performance does not guarantee future results
- Always conduct your own research (DYOR)
- Only risk money you can afford to lose
- Consider consulting a financial advisor before trading

## Contributing

To improve this project:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Future Enhancements

- [ ] Machine Learning signal optimization
- [ ] Sentiment analysis integration
- [ ] Volume-based analysis
- [ ] Advanced order types
- [ ] Multi-cryptocurrency correlation
- [ ] Real-time Discord/Telegram notifications
- [ ] Database for historical analysis
- [ ] Web dashboard
- [ ] Automated trading execution
- [ ] Backtesting framework

## License

MIT License - See LICENSE file for details

## Author

**Diogo Kuiaski**
- GitHub: [@diogokuiaski](https://github.com/diogokuiaski)

## Support

For issues, questions, or suggestions:
1. Check existing issues
2. Create a new issue with detailed description
3. Join our community discussions

---

**Last Updated**: 2024
**Version**: 0.2.0
