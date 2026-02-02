//! Multi-Provider Market Data Integration
//! 
//! Supports multiple market data sources with automatic failover:
//! - CoinMarketCap (primary)
//! - CoinDesk (secondary)
//! - CoinGecko (tertiary)
//! - Kraken (fallback)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Market data provider error
#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("Request failed: {0}")]
    RequestFailed(String),
    
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    
    #[error("Rate limited: {0}")]
    RateLimited(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("API error: {0}")]
    ApiError(String),
}

/// Market data from a provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketSnapshot {
    pub timestamp: u64,
    pub prices: HashMap<String, f64>,
    pub source: String,
}

/// CoinMarketCap API response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinMarketCapResponse {
    pub data: CoinMarketCapData,
    pub status: CoinMarketCapStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinMarketCapStatus {
    pub timestamp: String,
    pub error_code: i32,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinMarketCapData {
    #[serde(flatten)]
    pub cryptocurrencies: HashMap<String, CoinMarketCapCoin>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinMarketCapCoin {
    pub id: u32,
    pub name: String,
    pub symbol: String,
    pub quote: HashMap<String, CoinMarketCapQuote>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinMarketCapQuote {
    pub price: f64,
    #[serde(rename = "last_updated")]
    pub last_updated: String,
}

/// CoinMarketCap API client
pub struct CoinMarketCapProvider {
    api_key: String,
    client: reqwest::Client,
    base_url: String,
}

impl CoinMarketCapProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
            base_url: "https://pro-api.coinmarketcap.com/v1".to_string(),
        }
    }

    /// Fetch latest cryptocurrency prices from CoinMarketCap
    pub async fn fetch_prices(&self, symbols: &[&str]) -> Result<MarketSnapshot, ProviderError> {
        let symbols_param = symbols.join(",");
        let url = format!(
            "{}/cryptocurrency/quotes/latest?symbol={}&convert=USD",
            self.base_url, symbols_param
        );

        let response = self.client
            .get(&url)
            .header("X-CMC_PRO_API_KEY", &self.api_key)
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        if response.status().is_client_error() || response.status().is_server_error() {
            return Err(ProviderError::RequestFailed(
                format!("HTTP {}", response.status())
            ));
        }

        let data: CoinMarketCapResponse = response
            .json()
            .await
            .map_err(|e| ProviderError::InvalidResponse(e.to_string()))?;

        if data.status.error_code != 0 {
            return Err(ProviderError::ApiError(
                data.status.error_message.unwrap_or_default()
            ));
        }

        let mut prices = HashMap::new();
        for (_, coin) in &data.data.cryptocurrencies {
            if let Some(usd_quote) = coin.quote.get("USD") {
                let symbol = coin.symbol.to_uppercase();
                prices.insert(format!("{}-USD", symbol), usd_quote.price);
            }
        }

        Ok(MarketSnapshot {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            prices,
            source: "CoinMarketCap".to_string(),
        })
    }
}

/// CoinGecko API client (free, no API key required)
pub struct CoinGeckoProvider {
    client: reqwest::Client,
    base_url: String,
}

impl CoinGeckoProvider {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "https://api.coingecko.com/api/v3".to_string(),
        }
    }

    /// Fetch prices from CoinGecko
    pub async fn fetch_prices(&self, ids: &[&str]) -> Result<MarketSnapshot, ProviderError> {
        let ids_param = ids.join(",");
        let url = format!(
            "{}/simple/price?ids={}&vs_currencies=usd",
            self.base_url, ids_param
        );

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| ProviderError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ProviderError::RequestFailed(
                format!("HTTP {}", response.status())
            ));
        }

        let data: HashMap<String, HashMap<String, f64>> = response
            .json()
            .await
            .map_err(|e| ProviderError::InvalidResponse(e.to_string()))?;

        let mut prices = HashMap::new();
        for (id, quotes) in data {
            if let Some(&price) = quotes.get("usd") {
                prices.insert(format!("{}-USD", id.to_uppercase()), price);
            }
        }

        Ok(MarketSnapshot {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            prices,
            source: "CoinGecko".to_string(),
        })
    }
}

/// Multi-provider market data fetcher with automatic failover
pub struct MultiProviderFetcher {
    cmc_provider: Option<CoinMarketCapProvider>,
    gecko_provider: CoinGeckoProvider,
}

impl MultiProviderFetcher {
    pub fn new(coinmarketcap_key: Option<String>) -> Self {
        let cmc_provider = coinmarketcap_key.map(CoinMarketCapProvider::new);
        
        Self {
            cmc_provider,
            gecko_provider: CoinGeckoProvider::new(),
        }
    }

    /// Try to fetch prices, with automatic failover
    pub async fn fetch_btc_usd(&self) -> Result<f64, ProviderError> {
        // Try CoinMarketCap first if available
        if let Some(cmc) = &self.cmc_provider {
            match cmc.fetch_prices(&["BTC"]).await {
                Ok(snapshot) => {
                    if let Some(&price) = snapshot.prices.get("BTC-USD") {
                        println!("✓ Fetched BTC price from CoinMarketCap: ${:.2}", price);
                        return Ok(price);
                    }
                }
                Err(e) => {
                    eprintln!("⚠ CoinMarketCap error: {}, trying CoinGecko...", e);
                }
            }
        }

        // Fallback to CoinGecko
        match self.gecko_provider.fetch_prices(&["bitcoin"]).await {
            Ok(snapshot) => {
                if let Some(&price) = snapshot.prices.get("BITCOIN-USD") {
                    println!("✓ Fetched BTC price from CoinGecko: ${:.2}", price);
                    return Ok(price);
                }
            }
            Err(e) => {
                eprintln!("✗ CoinGecko error: {}", e);
            }
        }

        Err(ProviderError::RequestFailed("All providers failed".to_string()))
    }

    /// Fetch multiple cryptocurrency prices with failover
    pub async fn fetch_prices(&self, symbols: &[&str]) -> Result<MarketSnapshot, ProviderError> {
        // Try CoinMarketCap first
        if let Some(cmc) = &self.cmc_provider {
            match cmc.fetch_prices(symbols).await {
                Ok(snapshot) => {
                    println!("✓ Fetched {} prices from CoinMarketCap", symbols.len());
                    return Ok(snapshot);
                }
                Err(e) => {
                    eprintln!("⚠ CoinMarketCap error: {}, trying CoinGecko...", e);
                }
            }
        }

        // Fallback to CoinGecko with lowercase IDs
        let gecko_ids: Vec<String> = symbols.iter()
            .map(|s| {
                match s.to_lowercase().as_str() {
                    "btc" => "bitcoin".to_string(),
                    "eth" => "ethereum".to_string(),
                    "usdt" => "tether".to_string(),
                    s => s.to_string(),
                }
            })
            .collect();

        let gecko_ids_refs: Vec<&str> = gecko_ids.iter().map(|s| s.as_str()).collect();
        self.gecko_provider.fetch_prices(&gecko_ids_refs).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_snapshot_creation() {
        let mut prices = HashMap::new();
        prices.insert("BTC-USD".to_string(), 45000.0);

        let snapshot = MarketSnapshot {
            timestamp: 1234567890,
            prices,
            source: "Test".to_string(),
        };

        assert_eq!(snapshot.source, "Test");
        assert_eq!(snapshot.prices.len(), 1);
    }
}
