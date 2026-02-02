# Web Dashboard Setup

The web dashboard requires an HTTP server to function (WebSockets require HTTP protocol).

## Quick Start

### Option 1: Python HTTP Server (Easiest)
```bash
# From the examples directory
cd examples
python -m http.server 8000

# Then open http://localhost:8000/dashboard.html in your browser
```

### Option 2: Using Node.js http-server
```bash
npm install -g http-server
cd examples
http-server
```

### Option 3: Using VS Code Live Server
Install "Live Server" extension, then right-click dashboard.html and select "Open with Live Server"

## Architecture

The dashboard connects to the WebSocket server (`ws://127.0.0.1:9001`) and displays:

- **Real-time Market Data**: BTC-USD and ETH-USD prices from the server
- **Agent Performance**: Capital, trades, win rates for each trading agent
- **Live Trade Feed**: Real-time updates as agents execute trades
- **Charts**: Price history and agent capital distribution

## Server Status

When the server is running:
- ✅ **WebSocket Server**: `ws://127.0.0.1:9001`
- ✅ **Metrics HTTP Endpoint**: `http://127.0.0.1:9184/metrics`
- ✅ **Health Check**: `http://127.0.0.1:9184/health`

Start the WebSocket server:
```bash
cargo run --example ws_broadcast_v2 --release
```

## Dashboard Features

### Top Metrics
- BTC-USD price with % change
- ETH-USD price with % change
- Total trades executed across all agents
- Average win rate across all agents

### Price Charts
Real-time candlestick/line chart showing:
- BTC-USD (Bitcoin price)
- ETH-USD (Ethereum price)
- Updates every 30 seconds from market ticker

### Agent Capital Chart
Bar chart showing current capital for each active agent:
- ⚡ Momentum
- 🛡️ Conservative
- ⚖️ Balanced

### Agents Table
Detailed table with:
- Agent name and emoji icon
- Current capital balance
- Number of trades executed
- Win rate percentage
- Accumulated scars from losses
- Status (Active/Resting)

### Live Trade Feed
Real-time stream of trades as they execute:
- Green highlights for winning trades
- Red highlights for losing trades
- Timestamps for each event
- Trade PnL (profit/loss) amount

## Metrics Endpoint

The metrics server exports Prometheus-compatible metrics:

```bash
curl http://127.0.0.1:9184/metrics
```

Available metrics include:
- `agent_capital{agent}` - Current capital for each agent
- `agent_trades_total{agent}` - Total trades executed
- `agent_win_rate{agent}` - Win rate percentage
- `agent_scars{agent}` - Number of scars
- `market_price{symbol}` - Current price for each asset
- `trades_executed_total` - Total trades across all agents
- `arena_rounds_total` - Number of rounds completed

## WebSocket API

The dashboard communicates with the server via WebSocket. Messages are JSON-formatted:

### Market Update
```json
{
  "type": "market",
  "symbol": "BTC-USD",
  "price": 42000.50,
  "timestamp": 1738512000
}
```

### Agent Update
```json
{
  "type": "agent",
  "agent_name": "⚡ Momentum",
  "capital": 50000.00,
  "trades": 5,
  "win_rate": 60.0,
  "scars": 1,
  "action": "⚡ Momentum WON trade (+$450)",
  "timestamp": 1738512000
}
```

## Troubleshooting

### Dashboard shows "Connecting..."
- Ensure `cargo run --example ws_broadcast_v2 --release` is running
- Check firewall isn't blocking port 9001
- Check browser console for WebSocket errors

### No price updates
- Wait 30 seconds for first market ticker update
- Check terminal output shows market prices being generated

### No agent updates
- Wait 15 seconds for first agent simulator update
- Check terminal output shows agent state updates

## Next Steps

See [PHASE_3_WEBSOCKET_METRICS_GUIDE.md](../PHASE_3_WEBSOCKET_METRICS_GUIDE.md) for:
- Full API documentation
- PromQL query examples
- Integration patterns
- Deployment guide
