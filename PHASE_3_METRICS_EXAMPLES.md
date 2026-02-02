# Example Prometheus Metrics Output

Below is an example of the metrics output from `cargo run --example metrics_server_v2 --release`:

```
# HELP lineage_cache_hit_rate Cache hit rate percentage
# TYPE lineage_cache_hit_rate gauge
lineage_cache_hit_rate 96.32

# HELP lineage_request_success_rate Request success rate percentage
# TYPE lineage_request_success_rate gauge
lineage_request_success_rate 99.84

# HELP lineage_api_errors_total Total API errors
# TYPE lineage_api_errors_total counter
lineage_api_errors_total 0

# HELP lineage_agent_capital Agent current capital in USD
# TYPE lineage_agent_capital gauge
lineage_agent_capital{agent="Momentum"} 128960.00
lineage_agent_capital{agent="Conservative"} 113666.00
lineage_agent_capital{agent="Balanced"} 108135.00

# HELP lineage_agent_trades_total Total agent trades executed
# TYPE lineage_agent_trades_total counter
lineage_agent_trades_total{agent="Momentum"} 35
lineage_agent_trades_total{agent="Conservative"} 12
lineage_agent_trades_total{agent="Balanced"} 28

# HELP lineage_agent_win_rate Agent win rate as fraction [0-1]
# TYPE lineage_agent_win_rate gauge
lineage_agent_win_rate{agent="Momentum"} 0.5800
lineage_agent_win_rate{agent="Conservative"} 0.5200
lineage_agent_win_rate{agent="Balanced"} 0.6200

# HELP lineage_agent_scars_total Total agent losses/scars
# TYPE lineage_agent_scars_total counter
lineage_agent_scars_total{agent="Momentum"} 8
lineage_agent_scars_total{agent="Conservative"} 6
lineage_agent_scars_total{agent="Balanced"} 3

# HELP lineage_arena_rounds_total Total number of arena competition rounds
# TYPE lineage_arena_rounds_total counter
lineage_arena_rounds_total 20

# HELP lineage_arena_agent_wins_total Total agent arena round wins
# TYPE lineage_arena_agent_wins_total counter
lineage_arena_agent_wins_total{agent="Balanced"} 12
lineage_arena_agent_wins_total{agent="Momentum"} 8

# HELP lineage_arena_agent_final_capital Agent capital after arena round
# TYPE lineage_arena_agent_final_capital gauge
lineage_arena_agent_final_capital{agent="Balanced"} 128960.00
lineage_arena_agent_final_capital{agent="Momentum"} 120500.00
lineage_arena_agent_final_capital{agent="Conservative"} 113200.00

# HELP lineage_active_connections Active WebSocket connections
# TYPE lineage_active_connections gauge
lineage_active_connections 3

# HELP lineage_messages_broadcast_total Total messages broadcast to clients
# TYPE lineage_messages_broadcast_total counter
lineage_messages_broadcast_total 250

# HELP lineage_uptime_seconds Server uptime in seconds
# TYPE lineage_uptime_seconds gauge
lineage_uptime_seconds 45
```

## Example PromQL Queries

### Show agent capital over time
```promql
lineage_agent_capital
```

**Result:**
```
lineage_agent_capital{agent="Balanced"} 108135
lineage_agent_capital{agent="Conservative"} 113666
lineage_agent_capital{agent="Momentum"} 128960
```

### Find the agent with highest capital
```promql
topk(1, lineage_agent_capital)
```

**Result:**
```
{agent="Momentum"} 128960
```

### Compare win rates
```promql
lineage_agent_win_rate
```

**Result:**
```
{agent="Balanced"} 0.62
{agent="Conservative"} 0.52
{agent="Momentum"} 0.58
```

### Calculate average capital
```promql
avg(lineage_agent_capital)
```

**Result:**
```
114587.0
```

### Find agents below capital threshold
```promql
lineage_agent_capital < 120000
```

**Result:**
```
{agent="Balanced"} 108135
{agent="Conservative"} 113666
```

### Win rate vs capital correlation
```promql
lineage_agent_win_rate / (lineage_agent_capital / 100000)
```

This shows capital efficiency per unit of win rate.

### Sum trades by agent
```promql
sum(lineage_agent_trades_total) by (agent)
```

**Result:**
```
{agent="Balanced"} 28
{agent="Conservative"} 12
{agent="Momentum"} 35
```

### Cache efficiency alert (drop below 90%)
```promql
lineage_cache_hit_rate < 90
```

### API error rate
```promql
rate(lineage_api_errors_total[5m])
```

Shows errors per second over last 5 minutes.

### Uptime alert (reset detection)
```promql
increase(lineage_uptime_seconds[1m]) < 0
```

Triggers if uptime decreases (server restart).

## WebSocket Event Examples

### Market Event
```json
{
  "event_type": "market",
  "sequence": 145,
  "market": {
    "symbol": "BTC-USD",
    "price": 45123.45,
    "timestamp": "2026-02-03T14:30:45.123456Z",
    "change_percent": 0.32
  }
}
```

### Agent Event
```json
{
  "event_type": "agent",
  "sequence": 146,
  "agent": {
    "agent_id": "Balanced",
    "capital": 115234.56,
    "trades_executed": 28,
    "win_rate": 0.62,
    "status": "active",
    "timestamp": "2026-02-03T14:30:45.654321Z"
  }
}
```

### Welcome Message
```json
{
  "type": "welcome",
  "client_id": 1,
  "timestamp": "2026-02-03T14:30:45Z",
  "message": "Connected to Lineage WebSocket broadcast server"
}
```

## Interpreting the Metrics

| Metric | Healthy | Warning | Critical |
|--------|---------|---------|----------|
| `cache_hit_rate` | > 95% | 90-95% | < 90% |
| `request_success_rate` | > 99% | 95-99% | < 95% |
| `agent_capital` | Growing | Stable | < 50K initial |
| `win_rate` | > 0.5 | 0.4-0.5 | < 0.4 |
| `agent_scars_total` | < 5 | 5-10 | > 20 |
| `active_connections` | > 0 | 1-3 | 0 (disconnect) |

## Grafana Dashboard Variables

Define these variables in Grafana for dashboard flexibility:

**Agent Selector**
```promql
label_values(lineage_agent_capital, agent)
```

**Metric Selectors**
```promql
label_names(lineage_)
```

**Time Range**
- Default: Last 1 hour
- Min interval: 5 seconds
- Step: Auto

## Prometheus Retention Policy

Recommended `prometheus.yml` settings:

```yaml
global:
  scrape_interval: 15s        # Scrape every 15 seconds
  evaluation_interval: 15s    # Evaluate rules every 15 seconds
  external_labels:
    monitor: 'lineage-monitor'

# Keep metrics for 30 days
storage:
  tsdb:
    retention:
      time: 720h              # 30 days

scrape_configs:
  - job_name: 'lineage'
    static_configs:
      - targets: ['127.0.0.1:9184']
    scrape_interval: 15s
    scrape_timeout: 5s
```

## Alerting Rules Example

Create `lineage-alerts.yml`:

```yaml
groups:
  - name: lineage.rules
    interval: 30s
    rules:
      - alert: AgentCapitalLow
        expr: lineage_agent_capital < 50000
        for: 5m
        annotations:
          summary: "Agent {{ $labels.agent }} capital is low"

      - alert: CacheHitRateLow
        expr: lineage_cache_hit_rate < 90
        for: 5m
        annotations:
          summary: "Cache hit rate dropped below 90%"

      - alert: APIErrorRate
        expr: rate(lineage_api_errors_total[5m]) > 0.1
        for: 5m
        annotations:
          summary: "API error rate > 0.1 errors/sec"

      - alert: NoActiveConnections
        expr: lineage_active_connections == 0
        for: 1m
        annotations:
          summary: "No WebSocket clients connected"
```

