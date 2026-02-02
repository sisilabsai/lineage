//! Extensible traits for custom strategies and data providers
//!
//! Traits allow users to:
//! - Implement custom trading strategies
//! - Connect to alternative data providers (Binance, Kraken, etc.)
//! - Build custom visualizations
//! - Extend the arena with new evolutionary mechanics

use async_trait::async_trait;
use std::collections::HashMap;
use crate::scar::ScarSeverity;

/// Market price data point
#[derive(Debug, Clone)]
pub struct PricePoint {
    pub symbol: String,
    pub price: f64,
    pub timestamp: i64,
    pub volatility: f64, // Historical volatility %
}

/// Market data snapshot
#[derive(Debug, Clone)]
pub struct MarketSnapshot {
    pub prices: HashMap<String, PricePoint>,
    pub timestamp: i64,
}

/// Custom trading strategy trait
/// 
/// Implement this to create your own strategy:
/// ```rust,ignore
/// pub struct MyStrategy;
/// 
/// #[async_trait]
/// impl TradingStrategy for MyStrategy {
///     async fn decide_trade(&self, market: &MarketSnapshot) -> TradeDecision {
///         // Your logic here
///     }
/// }
/// ```
#[async_trait]
pub trait TradingStrategy: Send + Sync {
    /// Given current market data, decide whether to trade and how much
    async fn decide_trade(&self, market: &MarketSnapshot) -> TradeDecision;
    
    /// React to a loss event (e.g., adjust future strategy)
    fn on_loss(&mut self, _drawdown: f32, _loss_amount: u64) {}
    
    /// React to a win event
    fn on_win(&mut self, _gain_amount: u64) {}
    
    /// Get strategy name for display/logging
    fn name(&self) -> &str;
}

/// Trade decision output from a strategy
#[derive(Debug, Clone)]
pub struct TradeDecision {
    /// Should trade or hold?
    pub should_trade: bool,
    
    /// Which symbol to trade (e.g., "BTC-USD")
    pub symbol: String,
    
    /// How much capital to invest (0-1 = percentage of available)
    pub allocation_percentage: f32,
    
    /// Direction: "buy" or "sell"
    pub direction: String,
}

/// Custom market data provider trait
/// 
/// Implement this to connect to any data source:
/// ```rust,ignore
/// pub struct BinanceProvider;
/// 
/// #[async_trait]
/// impl MarketDataProvider for BinanceProvider {
///     async fn fetch_prices(&self, symbols: &[&str]) -> Result<MarketSnapshot, String> {
///         // Binance API calls
///     }
/// }
/// ```
#[async_trait]
pub trait MarketDataProvider: Send + Sync {
    /// Fetch latest prices for given symbols
    async fn fetch_prices(&self, symbols: &[&str]) -> Result<MarketSnapshot, String>;
    
    /// Get historical volatility for a symbol
    async fn get_volatility(&self, symbol: &str, period_days: u32) -> Result<f64, String>;
    
    /// Get provider name
    fn name(&self) -> &str;
    
    /// Indicate if provider requires authentication
    fn requires_auth(&self) -> bool {
        false
    }
}

/// Agent lifecycle event for custom behavior
#[derive(Debug, Clone)]
pub enum AgentEvent {
    /// Agent completed a trade
    TradeExecuted { symbol: String, success: bool, pnl: i64 },
    
    /// Agent suffered a loss and received a scar
    ScarInflicted { drawdown: f32, severity: ScarSeverity },
    
    /// Agent spawned offspring
    OffspringSpawned { parent_id: String, generation: u32 },
    
    /// Agent was terminated
    Terminated { reason: String },
}

/// Custom event handler trait
/// 
/// Implement to react to agent lifecycle events:
/// ```rust,ignore
/// pub struct MyEventHandler;
/// 
/// #[async_trait]
/// impl EventHandler for MyEventHandler {
///     async fn on_event(&self, event: AgentEvent) {
///         // Your logic
///     }
/// }
/// ```
#[async_trait]
pub trait EventHandler: Send + Sync {
    /// Called when agent events occur
    async fn on_event(&self, event: AgentEvent);
}

/// Visualization output format
#[derive(Debug, Clone, Copy)]
pub enum VisualizationFormat {
    ASCII,
    HTML,
    CSV,
    JSON,
}

/// Custom visualization trait for arena results
#[async_trait]
pub trait VisualizationProvider: Send + Sync {
    /// Generate visualization from arena results
    async fn visualize(&self, results: &str, format: VisualizationFormat) -> Result<Vec<u8>, String>;
    
    /// Provider name
    fn name(&self) -> &str;
}
