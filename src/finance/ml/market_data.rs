//! Market Data Integration: CoinMarketCap and CoinDesk providers
//! Fetches historical BTC/crypto prices and caches locally to respect API limits

use std::collections::HashMap;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use crate::finance::ml::errors::Result;

/// Historical candle (OHLCV data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    pub timestamp: DateTime<Utc>,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub volume: f32,
}

impl Candle {
    /// Return (open, high, low, close, volume) as features
    pub fn to_features(&self) -> Vec<f32> {
        vec![self.open, self.high, self.low, self.close, self.volume]
    }
}

/// Market data cache to minimize API calls
pub struct MarketDataCache {
    /// Symbol -> (candles, fetched_at)
    candles: HashMap<String, (Vec<Candle>, DateTime<Utc>)>,
    
    /// Cache validity duration
    cache_ttl: Duration,
    
    /// Maximum cache size per symbol (in candles)
    max_candles: usize,
}

impl MarketDataCache {
    pub fn new() -> Self {
        Self {
            candles: HashMap::new(),
            cache_ttl: Duration::hours(1),  // Refresh every hour
            max_candles: 1000,               // Keep 1000 candles max
        }
    }
    
    /// Check if cache is valid for symbol
    pub fn is_valid(&self, symbol: &str) -> bool {
        if let Some((_, fetched_at)) = self.candles.get(symbol) {
            let age = Utc::now() - *fetched_at;
            age < self.cache_ttl
        } else {
            false
        }
    }
    
    /// Get cached candles
    pub fn get(&self, symbol: &str) -> Option<Vec<Candle>> {
        self.candles.get(symbol).map(|(candles, _)| candles.clone())
    }
    
    /// Store candles in cache
    pub fn set(&mut self, symbol: String, candles: Vec<Candle>) {
        // Keep only most recent candles
        let trimmed = if candles.len() > self.max_candles {
            candles[candles.len() - self.max_candles..].to_vec()
        } else {
            candles
        };
        
        self.candles.insert(symbol, (trimmed, Utc::now()));
    }
    
    /// Clear old cache entries
    pub fn cleanup_expired(&mut self) {
        let now = Utc::now();
        self.candles.retain(|_, (_, fetched_at)| {
            now - *fetched_at < self.cache_ttl * 2  // Keep for 2x TTL before cleanup
        });
    }
}

/// CoinMarketCap API provider (mock for now)
pub struct CoinMarketCapProvider {
    cache: MarketDataCache,
}

impl CoinMarketCapProvider {
    pub fn new() -> Self {
        Self {
            cache: MarketDataCache::new(),
        }
    }
    
    /// Fetch historical price data (mocked)
    pub async fn fetch_candles(
        &mut self,
        symbol: &str,
        days: u32,
    ) -> Result<Vec<Candle>> {
        // Check cache first
        if self.cache.is_valid(symbol) {
            if let Some(candles) = self.cache.get(symbol) {
                println!("Cache hit for {}: {} candles", symbol, candles.len());
                return Ok(candles);
            }
        }
        
        // In real implementation, would call:
        // https://api.coinmarketcap.com/data/cryptocurrency/historical
        // For now, generate synthetic data
        println!("Generating synthetic data for {} ({} days)", symbol, days);
        
        let mut candles = Vec::new();
        let mut price = 30_000.0;  // Start BTC at ~30k
        let mut timestamp = Utc::now() - Duration::days(days as i64);
        
        for _ in 0..days {
            // Random walk
            let change = (rand::random::<f32>() - 0.5) * 2000.0;
            let close = (price + change).max(100.0);
            let open = price;
            let high = price.max(close) + rand::random::<f32>() * 500.0;
            let low = price.min(close) - rand::random::<f32>() * 500.0;
            let volume = rand::random::<f32>() * 1_000_000.0;
            
            candles.push(Candle {
                timestamp,
                open,
                high,
                low,
                close,
                volume,
            });
            
            price = close;
            timestamp = timestamp + Duration::days(1);
        }
        
        // Cache result
        self.cache.set(symbol.to_string(), candles.clone());
        
        Ok(candles)
    }
}

/// CoinDesk API provider (alternative source)
pub struct CoinDeskProvider {
    cache: MarketDataCache,
}

impl CoinDeskProvider {
    pub fn new() -> Self {
        Self {
            cache: MarketDataCache::new(),
        }
    }
    
    /// Fetch from CoinDesk (mocked)
    pub async fn fetch_current_price(&mut self, symbol: &str) -> Result<f32> {
        // In real implementation, would call:
        // https://api.coindesk.com/v1/bpi/currentprice/BTC
        
        // For now, return simulated price
        let base_price = match symbol {
            "BTC" => 30_000.0,
            "ETH" => 2_000.0,
            _ => 100.0,
        };
        
        let noise = (rand::random::<f32>() - 0.5) * 2000.0;
        Ok(base_price + noise)
    }
}

/// Market data fetcher with provider abstraction
pub struct MarketDataFetcher {
    coinmarketcap: CoinMarketCapProvider,
    coindesk: CoinDeskProvider,
}

impl MarketDataFetcher {
    pub fn new() -> Self {
        Self {
            coinmarketcap: CoinMarketCapProvider::new(),
            coindesk: CoinDeskProvider::new(),
        }
    }
    
    /// Fetch historical candles (prefers cached data)
    pub async fn fetch_historical(
        &mut self,
        symbol: &str,
        days: u32,
    ) -> Result<Vec<Candle>> {
        self.coinmarketcap.fetch_candles(symbol, days).await
    }
    
    /// Fetch current price (quick call)
    pub async fn fetch_current_price(&mut self, symbol: &str) -> Result<f32> {
        self.coindesk.fetch_current_price(symbol).await
    }
    
    /// Convert candles to market states for training
    pub fn candles_to_states(&self, candles: &[Candle], window_size: usize) -> Vec<Vec<f32>> {
        let mut states = Vec::new();
        
        for window in candles.windows(window_size) {
            // Aggregate features from window
            let mut features = vec![0.0; 5];
            
            for candle in window {
                let candle_features = candle.to_features();
                for (i, f) in candle_features.iter().enumerate() {
                    features[i] += f;
                }
            }
            
            // Average
            for f in &mut features {
                *f /= window_size as f32;
            }
            
            states.push(features);
        }
        
        states
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cache_validity() {
        let mut cache = MarketDataCache::new();
        
        assert!(!cache.is_valid("BTC"));
        
        let candles = vec![Candle {
            timestamp: Utc::now(),
            open: 30_000.0,
            high: 31_000.0,
            low: 29_000.0,
            close: 30_500.0,
            volume: 1_000_000.0,
        }];
        
        cache.set("BTC".to_string(), candles.clone());
        assert!(cache.is_valid("BTC"));
        assert_eq!(cache.get("BTC").unwrap(), candles);
    }
    
    #[test]
    fn test_candle_features() {
        let candle = Candle {
            timestamp: Utc::now(),
            open: 30_000.0,
            high: 31_000.0,
            low: 29_000.0,
            close: 30_500.0,
            volume: 1_000_000.0,
        };
        
        let features = candle.to_features();
        assert_eq!(features.len(), 5);
        assert_eq!(features[0], 30_000.0);  // open
        assert_eq!(features[4], 1_000_000.0);  // volume
    }
    
    #[tokio::test]
    async fn test_market_data_fetcher() {
        let mut fetcher = MarketDataFetcher::new();
        
        let candles = fetcher.fetch_historical("BTC", 30).await.unwrap();
        assert_eq!(candles.len(), 30);
        
        let price = fetcher.fetch_current_price("BTC").await.unwrap();
        assert!(price > 0.0);
    }
}
