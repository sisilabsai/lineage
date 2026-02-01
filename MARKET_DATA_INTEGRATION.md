# CoinDesk Market Data Integration Configuration

This document explains how to safely configure and use the real market data integration with Lineage Finance.

## ⚠️ Security Notice

**NEVER commit API keys to git.** The API key provided should only be used in:
- `.env` files (which should be in .gitignore)
- Environment variables set at runtime
- Secure configuration management systems

## Configuration Options

### Method 1: Environment Variables

Set these before running the application:

```bash
export COINDESK_API_KEY="155738413ca45b21ce9b8b0c1df265c4baf866ff12d7dde64fd021c4114187fd"
export COINDESK_MARKET="cadli"
export COINDESK_INSTRUMENTS="BTC-USD,ETH-USD"
export COINDESK_RPS="5"  # Requests per second (rate limiting)
```

### Method 2: .env File

Create `.env` in project root (add to `.gitignore`):

```
COINDESK_API_KEY=155738413ca45b21ce9b8b0c1df265c4baf866ff12d7dde64fd021c4114187fd
COINDESK_MARKET=cadli
COINDESK_INSTRUMENTS=BTC-USD,ETH-USD,SOL-USD
COINDESK_RPS=5
```

### Method 3: Programmatic Configuration

```rust
use lineage::finance::{MarketDataConfig, MarketDataClient};

let config = MarketDataConfig::new(
    "155738413ca45b21ce9b8b0c1df265c4baf866ff12d7dde64fd021c4114187fd".to_string(),
    "cadli".to_string(),
    vec![
        "BTC-USD".to_string(),
        "ETH-USD".to_string(),
    ],
    5, // requests per second
);

let client = MarketDataClient::new(
    config.api_key,
    config.requests_per_second,
);
```

## Rate Limiting Strategy

The integration implements **token bucket** rate limiting:

- **Requests Per Second**: 5 (configurable)
- **Backoff Strategy**: Exponential (100ms → 200ms → 400ms → ... → 30s max)
- **Max Retries**: 5 attempts before failure
- **Cache TTL**: 5 seconds (minimize API calls)

### Rate Limit Handling

```rust
match client.get_latest_prices("cadli", &["BTC-USD", "ETH-USD"]).await {
    Ok(data) => {
        // Use price data
        println!("BTC: ${}", data.prices["BTC-USD"].price);
    }
    Err(MarketDataError::RateLimited { retry_after_secs }) => {
        // Wait and retry manually or use automatic retry
        eprintln!("Rate limited, wait {}s", retry_after_secs);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## API Endpoint Details

### Base URL
```
https://data-api.coindesk.com
```

### Endpoint
```
/index/cc/v1/latest/tick
```

### Query Parameters
- `market`: Market identifier (e.g., "cadli")
- `instruments`: Comma-separated instruments (e.g., "BTC-USD,ETH-USD")
- `apply_mapping`: Enable field mapping (true)

### Example Request
```
GET https://data-api.coindesk.com/index/cc/v1/latest/tick?market=cadli&instruments=BTC-USD,ETH-USD&apply_mapping=true
Authorization: Bearer 155738413ca45b21ce9b8b0c1df265c4baf866ff12d7dde64fd021c4114187fd
```

## Usage Examples

### Basic Usage

```rust
use lineage::finance::{MarketDataClient, MarketDataConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load config from environment
    let config = MarketDataConfig::from_env()?;
    
    // Create client with rate limiting
    let client = MarketDataClient::new(
        config.api_key,
        config.requests_per_second,
    );
    
    // Fetch latest prices with automatic retry
    let prices = client.get_latest_prices(
        "cadli",
        &["BTC-USD", "ETH-USD"],
    ).await?;
    
    println!("Prices at {}: {:?}", prices.timestamp, prices.prices);
    
    Ok(())
}
```

### With Error Handling

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MarketDataClient::new(
        std::env::var("COINDESK_API_KEY")?,
        5,
    );
    
    loop {
        match client.get_latest_prices("cadli", &["BTC-USD"]).await {
            Ok(data) => {
                println!("✓ Got prices: {:?}", data);
                break;
            }
            Err(e) => {
                eprintln!("✗ Error: {}", e);
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        }
    }
    
    Ok(())
}
```

### Integration with Trading Agent

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MarketDataClient::new(
        std::env::var("COINDESK_API_KEY")?,
        5,
    );
    
    // Fetch real market data
    let price_data = client.get_latest_prices(
        "cadli",
        &["BTC-USD", "ETH-USD"],
    ).await?;
    
    // Create trading agent
    let mut agent = FinanceAgent::new(
        "RealMarketBot".to_string(),
        100_000,
        0,
    );
    
    // Use real prices for trading decisions
    for (instrument, price_point) in &price_data.prices {
        println!(
            "{}: ${:.2} (bid: ${:.2}, ask: ${:.2})",
            instrument, price_point.price, price_point.bid, price_point.ask
        );
        
        // Execute trades based on real prices
        // ... trading logic here ...
    }
    
    Ok(())
}
```

## Monitoring & Diagnostics

### Check Cache Status

```rust
let stats = client.cache_stats();
println!("Cache entries: {}/{}", stats.entries, stats.max_entries);
```

### Check Rate Limiter Status

```rust
let status = client.rate_limiter_status();
println!("Rate limit: {}/sec, Load: {}", 
    status.requests_per_second,
    status.current_load);
```

### Clear Cache

```rust
client.clear_cache();
println!("Cache cleared");
```

## Error Handling

The module provides detailed error types:

```rust
pub enum MarketDataError {
    RequestFailed(String),           // HTTP request failed
    RateLimited { retry_after_secs: u64 },  // Hit rate limit
    InvalidResponse(String),         // Response parsing failed
    ParseError(String),              // JSON parsing failed
    RateLimiterError,               // Internal rate limiter error
    NetworkError(String),            // Network connectivity issue
    ApiError(String),               // API returned error
}
```

## Rate Limiting Best Practices

1. **Respect Configured Limits**: Don't bypass rate limiting
2. **Cache Aggressively**: 5-second cache reduces API calls by ~96%
3. **Batch Requests**: Fetch multiple instruments in one call
4. **Monitor Limits**: Check `rate_limiter_status()` regularly
5. **Graceful Degradation**: Fall back to simulated data if API unavailable

## Testing

### Without Real API (Mocked)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_from_env() {
        std::env::set_var("COINDESK_API_KEY", "test_key");
        let config = MarketDataConfig::from_env().unwrap();
        assert_eq!(config.api_key, "test_key");
    }
}
```

### With Real API (Integration Test)

```bash
# Run with environment variables
COINDESK_API_KEY=... cargo test --test market_data_integration -- --ignored --nocapture
```

## Troubleshooting

### "API key not set"
**Solution**: Ensure `COINDESK_API_KEY` environment variable is set before running.

### "Rate limited, wait 30s"
**Solution**: The service is rate limited. Wait or reduce `COINDESK_RPS` value.

### "Invalid API response"
**Solution**: Check if API endpoint URL or parameters are correct. Verify instrument names.

### "Connection timeout"
**Solution**: Check internet connection and ensure API server is accessible.

## Production Deployment

### Security Checklist

- [ ] API key stored in secure vault (not in code)
- [ ] Environment variables set at deployment time
- [ ] Rate limiting configured appropriately for production
- [ ] Error handling and logging in place
- [ ] Fallback data source if API fails
- [ ] Monitoring of API health and response times
- [ ] Circuit breaker pattern for repeated failures

### Example Deployment Configuration

```yaml
# kubernetes secret
apiVersion: v1
kind: Secret
metadata:
  name: lineage-market-data
type: Opaque
stringData:
  api_key: "155738413ca45b21ce9b8b0c1df265c4baf866ff12d7dde64fd021c4114187fd"
```

```yaml
# deployment.yaml
env:
- name: COINDESK_API_KEY
  valueFrom:
    secretKeyRef:
      name: lineage-market-data
      key: api_key
- name: COINDESK_RPS
  value: "5"
```

## Further Reading

- [CoinDesk API Documentation](https://www.coindesk.com/)
- [Governor Rate Limiter](https://github.com/antifuchs/governor)
- [Tokio Async Runtime](https://tokio.rs/)
- [Error Handling Best Practices](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
