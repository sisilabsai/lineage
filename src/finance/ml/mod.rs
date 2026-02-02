//! Machine Learning module for adaptive trading strategies
//! 
//! This module provides reinforcement learning capabilities for
//! autonomous trading agents in the lineage-finance library.
//! 
//! # Feature
//! Only included when `ml` feature is enabled.
//! Add to Cargo.toml: `features = ["ml"]`

#![allow(dead_code)]

#[cfg(feature = "ml")]
pub mod errors;

#[cfg(feature = "ml")]
pub mod traits;

#[cfg(feature = "ml")]
pub mod models;

#[cfg(feature = "ml")]
pub mod training;

#[cfg(feature = "ml")]
pub mod evolution;

#[cfg(feature = "ml")]
pub mod integration;

#[cfg(feature = "ml")]
pub mod market_data;

#[cfg(feature = "ml")]
pub mod agent_integration;

// Re-exports for convenience
#[cfg(feature = "ml")]
pub use traits::{MlStrategy, MarketState, TradeDecision, TradeAction, ModelMetadata};

#[cfg(feature = "ml")]
pub use errors::MlError;

#[cfg(feature = "ml")]
pub use models::q_net::SimpleQNet;

#[cfg(feature = "ml")]
pub use market_data::{Candle, MarketDataCache, MarketDataFetcher, CoinMarketCapProvider, CoinDeskProvider};

#[cfg(feature = "ml")]
pub use agent_integration::{MLFinanceAgent, MLAgentArena};
