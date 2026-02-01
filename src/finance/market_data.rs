//! Real Market Data Integration - CoinDesk API Integration with Rate Limiting
//!
//! This module provides live market data integration with careful rate limiting:
//! - Async HTTP requests to CoinDesk market data API
//! - Token bucket rate limiting to respect API constraints
//! - Exponential backoff on rate limit errors
//! - Response caching to minimize API calls
//! - Type-safe parsing and error handling

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use thiserror::Error;
use std::sync::atomic::{AtomicU64, Ordering};

/// Market data API client with built-in rate limiting
pub struct MarketDataClient {
    /// API base URL
    base_url: String,
    
    /// API key for authentication
    api_key: String,
    
    /// HTTP client
    client: reqwest::Client,
    
    /// Rate limiter state (tokens and timestamp)
    rate_limiter_state: Arc<RateLimiterState>,
    
    /// Response cache with TTL
    cache: Arc<Mutex<ResponseCache>>,
    
    /// Retry configuration
    retry_config: RetryConfig,
}

/// Simple rate limiter state for token bucket
struct RateLimiterState {
    tokens: AtomicU64,
    last_refill: Mutex<Instant>,
    refill_rate: u64, // tokens per second
}

/// Response cache entry with time-to-live
#[derive(Clone, Debug)]
#[allow(dead_code)]
struct CacheEntry<T> {
    data: T,
    timestamp: Instant,
    ttl: Duration,
}

#[allow(dead_code)]
impl<T> CacheEntry<T> {
    fn is_expired(&self) -> bool {
        self.timestamp.elapsed() > self.ttl
    }
}

/// In-memory response cache
struct ResponseCache {
    entries: HashMap<String, (String, Instant)>,
    max_entries: usize,
}

impl ResponseCache {
    fn new(max_entries: usize) -> Self {
        ResponseCache {
            entries: HashMap::new(),
            max_entries,
        }
    }
    
    fn get(&self, key: &str) -> Option<String> {
        if let Some((data, timestamp)) = self.entries.get(key) {
            // Cache valid for 5 seconds
            if timestamp.elapsed() < Duration::from_secs(5) {
                return Some(data.clone());
            }
        }
        None
    }
    
    fn set(&mut self, key: String, value: String) {
        // Simple LRU: remove oldest if at capacity
        if self.entries.len() >= self.max_entries {
            if let Some((key_to_remove, _)) = self.entries.iter()
                .min_by_key(|(_, (_, ts))| ts)
                .map(|(k, v)| (k.clone(), v.clone())) {
                self.entries.remove(&key_to_remove);
            }
        }
        self.entries.insert(key, (value, Instant::now()));
    }
    
    #[allow(dead_code)]
    fn clear_expired(&mut self) {
        let now = Instant::now();
        self.entries.retain(|_, (_, ts)| now.duration_since(*ts) < Duration::from_secs(5));
    }
}

/// Retry configuration for rate limit handling
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_retries: u32,
    
    /// Initial backoff duration
    pub initial_backoff: Duration,
    
    /// Maximum backoff duration
    pub max_backoff: Duration,
    
    /// Backoff multiplier (exponential)
    pub backoff_multiplier: f32,
}

impl Default for RetryConfig {
    fn default() -> Self {
        RetryConfig {
            max_retries: 5,
            initial_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(30),
            backoff_multiplier: 2.0,
        }
    }
}

/// Market data API errors
#[derive(Error, Debug)]
pub enum MarketDataError {
    #[error("API request failed: {0}")]
    RequestFailed(String),
    
    #[error("Rate limit exceeded: {retry_after_secs}s remaining")]
    RateLimited { retry_after_secs: u64 },
    
    #[error("Invalid API response: {0}")]
    InvalidResponse(String),
    
    #[error("Parsing error: {0}")]
    ParseError(String),
    
    #[error("Rate limiter error")]
    RateLimiterError,
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("API error: {0}")]
    ApiError(String),
}

/// CoinDesk API response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceData {
    pub timestamp: u64,
    pub prices: HashMap<String, PricePoint>,
}

/// Individual price point from API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricePoint {
    pub instrument: String,
    pub price: f64,
    pub mid_price: f64,
    pub bid: f64,
    pub ask: f64,
}

/// Tick data from CoinDesk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickData {
    pub timestamp: u64,
    pub instruments: Vec<InstrumentData>,
}

/// Individual instrument data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstrumentData {
    pub instrument: String,
    pub price: f64,
    pub bid: f64,
    pub ask: f64,
}

impl MarketDataClient {
    /// Create a new market data client with rate limiting
    pub fn new(api_key: String, requests_per_second: u32) -> Self {
        MarketDataClient {
            base_url: "https://data-api.coindesk.com".to_string(),
            api_key,
            client: reqwest::Client::new(),
            rate_limiter_state: Arc::new(RateLimiterState {
                tokens: AtomicU64::new(requests_per_second as u64),
                last_refill: Mutex::new(Instant::now()),
                refill_rate: requests_per_second as u64,
            }),
            cache: Arc::new(Mutex::new(ResponseCache::new(100))),
            retry_config: RetryConfig::default(),
        }
    }
    
    /// Check and acquire a token from rate limiter
    fn acquire_token(&self) -> bool {
        let mut last_refill = self.rate_limiter_state.last_refill.lock().unwrap();
        let now = Instant::now();
        
        if let Ok(elapsed) = now.duration_since(*last_refill).as_secs_f64().try_into() {
            let elapsed_secs: f64 = elapsed;
            let tokens_to_add = (elapsed_secs * self.rate_limiter_state.refill_rate as f64) as u64;
            
            if tokens_to_add > 0 {
                let current = self.rate_limiter_state.tokens.load(Ordering::SeqCst);
                let new_tokens = (current + tokens_to_add).min(self.rate_limiter_state.refill_rate);
                self.rate_limiter_state.tokens.store(new_tokens, Ordering::SeqCst);
                *last_refill = now;
            }
        }
        
        let current = self.rate_limiter_state.tokens.load(Ordering::SeqCst);
        if current > 0 {
            self.rate_limiter_state.tokens.store(current - 1, Ordering::SeqCst);
            true
        } else {
            false
        }
    }
    
    /// Fetch latest price data with rate limiting and retries
    pub async fn get_latest_prices(
        &self,
        market: &str,
        instruments: &[&str],
    ) -> Result<PriceData, MarketDataError> {
        let cache_key = format!("{}:{}", market, instruments.join(","));
        
        // Check cache first
        {
            let cache = self.cache.lock().unwrap();
            if let Some(cached) = cache.get(&cache_key) {
                // Try to parse cached response
                if let Ok(data) = serde_json::from_str::<PriceData>(&cached) {
                    return Ok(data);
                }
            }
        }
        
        // Construct request URL
        let instruments_param = instruments.join(",");
        let url = format!(
            "{}/index/cc/v1/latest/tick?market={}&instruments={}&apply_mapping=true",
            self.base_url, market, instruments_param
        );
        
        // Execute with retries and rate limiting
        self.execute_with_retry(&url, &cache_key).await
    }
    
    /// Execute request with exponential backoff retry logic
    async fn execute_with_retry(
        &self,
        url: &str,
        cache_key: &str,
    ) -> Result<PriceData, MarketDataError> {
        let mut backoff = self.retry_config.initial_backoff;
        
        for attempt in 0..=self.retry_config.max_retries {
            // Check rate limiter and wait if needed
            let mut wait_time = 0;
            while !self.acquire_token() {
                wait_time += 10;
                if wait_time > 1000 {
                    return Err(MarketDataError::RateLimited { retry_after_secs: 1 });
                }
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
            
            // Make HTTP request
            match self.client
                .get(url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("User-Agent", "LineageFinance/1.0")
                .timeout(Duration::from_secs(10))
                .send()
                .await
            {
                Ok(response) => {
                    match response.status() {
                        reqwest::StatusCode::OK => {
                            // Parse response
                            match response.json::<TickData>().await {
                                Ok(tick_data) => {
                                    let price_data = self.convert_tick_to_price_data(tick_data);
                                    
                                    // Cache successful response
                                    {
                                        let mut cache = self.cache.lock().unwrap();
                                        cache.set(
                                            cache_key.to_string(),
                                            serde_json::to_string(&price_data).unwrap_or_default(),
                                        );
                                    }
                                    
                                    return Ok(price_data);
                                }
                                Err(e) => {
                                    return Err(MarketDataError::ParseError(e.to_string()));
                                }
                            }
                        }
                        reqwest::StatusCode::TOO_MANY_REQUESTS => {
                            // Rate limited - exponential backoff
                            if attempt < self.retry_config.max_retries {
                                tokio::time::sleep(backoff).await;
                                backoff = Duration::from_millis(
                                    ((backoff.as_millis() as f32) * self.retry_config.backoff_multiplier) as u64
                                ).min(self.retry_config.max_backoff);
                                continue;
                            }
                            return Err(MarketDataError::RateLimited { retry_after_secs: backoff.as_secs() });
                        }
                        reqwest::StatusCode::UNAUTHORIZED => {
                            return Err(MarketDataError::ApiError("Invalid API key".to_string()));
                        }
                        status => {
                            let status_code = status.as_u16();
                            if attempt < self.retry_config.max_retries && status.is_server_error() {
                                tokio::time::sleep(backoff).await;
                                backoff = Duration::from_millis(
                                    ((backoff.as_millis() as f32) * self.retry_config.backoff_multiplier) as u64
                                ).min(self.retry_config.max_backoff);
                                continue;
                            }
                            return Err(MarketDataError::ApiError(format!("HTTP {}", status_code)));
                        }
                    }
                }
                Err(e) => {
                    if attempt < self.retry_config.max_retries {
                        tokio::time::sleep(backoff).await;
                        backoff = Duration::from_millis(
                            ((backoff.as_millis() as f32) * self.retry_config.backoff_multiplier) as u64
                        ).min(self.retry_config.max_backoff);
                        continue;
                    }
                    return Err(MarketDataError::NetworkError(e.to_string()));
                }
            }
        }
        
        Err(MarketDataError::RequestFailed("Max retries exceeded".to_string()))
    }
    
    /// Convert CoinDesk TickData to our PriceData format
    fn convert_tick_to_price_data(&self, tick: TickData) -> PriceData {
        let mut prices = HashMap::new();
        
        for instrument in tick.instruments {
            prices.insert(
                instrument.instrument.clone(),
                PricePoint {
                    instrument: instrument.instrument,
                    price: instrument.price,
                    mid_price: (instrument.bid + instrument.ask) / 2.0,
                    bid: instrument.bid,
                    ask: instrument.ask,
                },
            );
        }
        
        PriceData {
            timestamp: tick.timestamp,
            prices,
        }
    }
    
    /// Get current rate limiter status
    pub fn rate_limiter_status(&self) -> RateLimiterStatus {
        RateLimiterStatus {
            requests_per_second: 10, // Default config
            current_load: "active".to_string(),
        }
    }
    
    /// Clear cache (useful for testing)
    pub fn clear_cache(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.entries.clear();
    }
    
    /// Get cache statistics
    pub fn cache_stats(&self) -> CacheStats {
        let cache = self.cache.lock().unwrap();
        CacheStats {
            entries: cache.entries.len(),
            max_entries: cache.max_entries,
        }
    }
}

/// Rate limiter status information
#[derive(Debug, Clone)]
pub struct RateLimiterStatus {
    pub requests_per_second: u32,
    pub current_load: String,
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub entries: usize,
    pub max_entries: usize,
}

/// Configuration loaded from environment or config file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDataConfig {
    pub api_key: String,
    pub base_url: String,
    pub market: String,
    pub instruments: Vec<String>,
    pub requests_per_second: u32,
    pub cache_ttl_seconds: u64,
}

impl MarketDataConfig {
    /// Load from environment variables
    pub fn from_env() -> Result<Self, MarketDataError> {
        let api_key = std::env::var("COINDESK_API_KEY")
            .map_err(|_| MarketDataError::ApiError("COINDESK_API_KEY not set".to_string()))?;
        
        let market = std::env::var("COINDESK_MARKET")
            .unwrap_or_else(|_| "cadli".to_string());
        
        let instruments = std::env::var("COINDESK_INSTRUMENTS")
            .unwrap_or_else(|_| "BTC-USD,ETH-USD".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        
        let requests_per_second = std::env::var("COINDESK_RPS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(5);
        
        Ok(MarketDataConfig {
            api_key,
            base_url: "https://data-api.coindesk.com".to_string(),
            market,
            instruments,
            requests_per_second,
            cache_ttl_seconds: 5,
        })
    }
    
    /// Create config from explicit values
    pub fn new(
        api_key: String,
        market: String,
        instruments: Vec<String>,
        requests_per_second: u32,
    ) -> Self {
        MarketDataConfig {
            api_key,
            base_url: "https://data-api.coindesk.com".to_string(),
            market,
            instruments,
            requests_per_second,
            cache_ttl_seconds: 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cache_entry_expiration() {
        let entry = CacheEntry {
            data: "test".to_string(),
            timestamp: Instant::now() - Duration::from_secs(10),
            ttl: Duration::from_secs(5),
        };
        assert!(entry.is_expired());
    }
    
    #[test]
    fn test_market_data_config_new() {
        let config = MarketDataConfig::new(
            "test_key".to_string(),
            "cadli".to_string(),
            vec!["BTC-USD".to_string()],
            5,
        );
        assert_eq!(config.market, "cadli");
        assert_eq!(config.instruments.len(), 1);
    }
    
    #[test]
    fn test_retry_config_default() {
        let config = RetryConfig::default();
        assert_eq!(config.max_retries, 5);
        assert_eq!(config.backoff_multiplier, 2.0);
    }
}
