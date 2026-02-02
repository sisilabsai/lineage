//! Core traits for ML-based trading strategies

use async_trait::async_trait;
use serde::{Serialize, Deserialize};

/// Market state representation for ML models
/// Normalized input for neural networks
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MarketState {
    /// Normalized cryptocurrency prices (log scale)
    pub prices: Vec<f32>,
    
    /// Historical volatility for each symbol (%)
    pub volatility: Vec<f32>,
    
    /// Agent's current capital (normalized to 0-1)
    pub agent_capital: f32,
    
    /// Number of scars (consequences from losses)
    pub scar_count: u32,
    
    /// Recent win/loss ratio (0.0-1.0)
    pub win_loss_ratio: f32,
    
    /// Unix timestamp of market snapshot
    pub timestamp: u64,
}

impl MarketState {
    /// Convert market state to feature vector for neural network
    pub fn to_features(&self) -> Vec<f32> {
        vec![
            self.prices.iter().sum::<f32>() / self.prices.len().max(1) as f32,
            self.volatility.iter().sum::<f32>() / self.volatility.len().max(1) as f32,
            self.agent_capital,
            (1.0 - (self.scar_count as f32 / 10.0).min(1.0)),
            self.win_loss_ratio,
        ]
    }
}

/// Action selected by ML strategy
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TradeAction {
    Buy,
    Sell,
    Hold,
}

impl TradeAction {
    pub fn to_index(&self) -> usize {
        match self {
            TradeAction::Buy => 0,
            TradeAction::Sell => 1,
            TradeAction::Hold => 2,
        }
    }
    
    pub fn from_index(idx: usize) -> Self {
        match idx {
            0 => TradeAction::Buy,
            1 => TradeAction::Sell,
            _ => TradeAction::Hold,
        }
    }
}

/// Trade decision from ML model with confidence
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeDecision {
    /// Action (Buy/Sell/Hold)
    pub action: TradeAction,
    
    /// Confidence score [0.0, 1.0]
    pub confidence: f32,
    
    /// Amount of capital to deploy
    pub amount: u64,
    
    /// Model identifier for audit trail
    pub model_id: String,
}

/// Metadata about an ML model
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelMetadata {
    /// Model name (e.g., "SimpleQNet")
    pub name: String,
    
    /// Model version
    pub version: String,
    
    /// Parent model ID (if spawned from parent)
    pub parent_id: Option<String>,
    
    /// Number of times mutated
    pub mutation_count: u32,
    
    /// Generation number
    pub generations: u32,
}

/// Core trait for ML-based trading strategies
/// Implementors provide neural network-based trading logic
#[async_trait]
pub trait MlStrategy: Send + Sync {
    /// Predict trading action given current market state
    async fn predict(&self, state: &MarketState) -> Result<TradeDecision, crate::finance::ml::errors::MlError>;
    
    /// Update model weights with gradients (training)
    fn update_weights(&mut self, gradients: &[f32]) -> Result<(), crate::finance::ml::errors::MlError>;
    
    /// Serialize model weights to bytes (for spawning)
    fn serialize(&self) -> Result<Vec<u8>, crate::finance::ml::errors::MlError>;
    
    /// Deserialize model weights from bytes (from parent)
    fn deserialize(&mut self, data: &[u8]) -> Result<(), crate::finance::ml::errors::MlError>;
    
    /// Get model metadata
    fn metadata(&self) -> ModelMetadata;
    
    /// Apply mutation for evolution (Gaussian noise on weights)
    fn mutate(&mut self, mutation_rate: f32, mutation_strength: f32) -> Result<(), crate::finance::ml::errors::MlError>;
    
    /// Clone the model for offspring creation
    fn clone_box(&self) -> Box<dyn MlStrategy>;
}
