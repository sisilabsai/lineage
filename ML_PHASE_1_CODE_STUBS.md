# ML Phase 1: Code Stubs for Immediate Implementation

This document provides all the boilerplate code to make Phase 1 compile successfully with tch-rs and libtorch.

---

## 1. Update Cargo.toml

Add to dependencies section:

```toml
[dependencies]
# ... existing dependencies ...

# ML/Torch support (optional)
tch = { version = "0.15", optional = true }
rand_distr = "0.4"

[features]
default = []
ml = ["tch"]  # Feature flag to keep ML optional
```

---

## 2. Create build.rs (Project Root)

**File**: `build.rs`

```rust
fn main() {
    // Only configure libtorch linking when ml feature is enabled
    #[cfg(feature = "ml")]
    {
        let libtorch_path = std::env::var("LIBTORCH")
            .or_else(|_| {
                // Try default Windows installation location
                std::env::var("ProgramFiles").map(|pf| {
                    format!("{}\\libtorch", pf)
                })
            })
            .or_else(|_| {
                // Try current project libtorch
                let cwd = std::env::current_dir()
                    .expect("Cannot get current directory")
                    .join("libtorch");
                if cwd.exists() {
                    Ok(cwd.to_string_lossy().to_string())
                } else {
                    Err("libtorch not found".to_string())
                }
            })
            .expect(
                "LIBTORCH environment variable not set!\n\
                 Set it to the libtorch installation directory.\n\
                 Or download from: https://download.pytorch.org/libtorch/cpu/"
            );
        
        println!("cargo:rustc-env=LIBTORCH={}", libtorch_path);
        println!("cargo:rustc-link-search=native={}/lib", libtorch_path);
        
        // Link libraries (Windows)
        #[cfg(target_os = "windows")]
        {
            println!("cargo:rustc-link-lib=dylib=torch_cpu");
            println!("cargo:rustc-link-lib=dylib=torch");
            println!("cargo:rustc-link-lib=dylib=c10");
        }
        
        // Link libraries (Linux)
        #[cfg(target_os = "linux")]
        {
            println!("cargo:rustc-link-lib=dylib=torch_cpu");
            println!("cargo:rustc-link-lib=dylib=torch");
        }
    }
}
```

---

## 3. Core ML Module Stubs

### src/finance/ml/mod.rs

```rust
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

// Re-exports for convenience
#[cfg(feature = "ml")]
pub use traits::{MlStrategy, MarketState, TradeDecision, TradeAction, ModelMetadata};

#[cfg(feature = "ml")]
pub use errors::MlError;

#[cfg(feature = "ml")]
pub use models::q_net::SimpleQNet;
```

---

### src/finance/ml/errors.rs

```rust
//! Error types for ML operations

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MlError {
    #[error("Tensor operation failed: {0}")]
    TensorError(String),
    
    #[error("Model serialization failed: {0}")]
    SerializationError(String),
    
    #[error("Training error: {0}")]
    TrainingError(String),
    
    #[error("Mutation error: {0}")]
    MutationError(String),
    
    #[error("Invalid state: {0}")]
    InvalidState(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
```

---

### src/finance/ml/traits.rs

```rust
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
```

---

### src/finance/ml/models/mod.rs

```rust
//! Neural network models for trading

#[cfg(feature = "ml")]
pub mod base;

#[cfg(feature = "ml")]
pub mod q_net;
```

---

### src/finance/ml/models/base.rs

```rust
//! Base model utilities

/// Helper functions for neural network operations
pub mod network_utils {
    use std::collections::HashMap;
    
    /// Normalize prices for neural network input
    pub fn normalize_prices(prices: &HashMap<String, f64>) -> Vec<f32> {
        let mut price_values: Vec<f32> = prices
            .values()
            .map(|&p| (p.ln()) as f32)  // Log prices for stability
            .collect();
        
        if price_values.is_empty() {
            return vec![0.0];
        }
        
        // Standardize: (x - mean) / std
        let mean = price_values.iter().sum::<f32>() / price_values.len() as f32;
        let variance = price_values.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f32>() / price_values.len() as f32;
        let std = variance.sqrt();
        
        if std > 0.0 {
            price_values.iter_mut().for_each(|x| {
                *x = (*x - mean) / std;
            });
        }
        
        price_values
    }
}
```

---

### src/finance/ml/models/q_net.rs

```rust
//! Simple Q-Network for reinforcement learning trading

use crate::finance::ml::traits::{MlStrategy, MarketState, TradeAction, TradeDecision, ModelMetadata};
use crate::finance::ml::errors::MlError;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

/// Simple Q-Network for discrete action spaces (Buy, Sell, Hold)
/// Lightweight network designed for fast inference in trading
#[derive(Clone, Serialize, Deserialize)]
pub struct SimpleQNet {
    /// Network weights (flattened)
    weights: Vec<f32>,
    
    /// Exploration rate (epsilon-greedy)
    pub exploration_rate: f32,
    
    /// Model metadata
    metadata: ModelMetadata,
    
    /// Model configuration
    config: QNetConfig,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct QNetConfig {
    pub input_size: usize,
    pub hidden_size: usize,
    pub output_size: usize,  // 3 actions: Buy, Sell, Hold
}

impl SimpleQNet {
    /// Create new Q-network with random initialization
    pub fn new(input_size: usize, hidden_size: usize) -> Result<Self, MlError> {
        let output_size = 3;  // Buy, Sell, Hold
        
        // Initialize weights with Xavier initialization
        let num_weights = (input_size * hidden_size)      // Layer 1
                        + hidden_size                       // Bias 1
                        + (hidden_size * hidden_size)      // Layer 2
                        + hidden_size                       // Bias 2
                        + (hidden_size * output_size)      // Output layer
                        + output_size;                      // Output bias
        
        let weights = vec![0.0; num_weights];  // Placeholder
        
        Ok(Self {
            weights,
            exploration_rate: 0.1,
            metadata: ModelMetadata {
                name: "SimpleQNet".to_string(),
                version: "1.0".to_string(),
                parent_id: None,
                mutation_count: 0,
                generations: 0,
            },
            config: QNetConfig {
                input_size,
                hidden_size,
                output_size,
            },
        })
    }
    
    /// Forward pass: state → Q-values
    pub fn forward(&self, state: &[f32]) -> Result<Vec<f32>, MlError> {
        // Simple feedforward without tch-rs (for compatibility)
        // Returns dummy Q-values for now
        
        if state.len() != self.config.input_size {
            return Err(MlError::InvalidState(
                format!("Expected {} inputs, got {}", self.config.input_size, state.len())
            ));
        }
        
        // Placeholder: return uniform Q-values
        Ok(vec![0.0; self.config.output_size])
    }
    
    /// Epsilon-greedy action selection
    pub fn select_action(&self, q_values: &[f32]) -> usize {
        if rand::random::<f32>() < self.exploration_rate {
            // Exploration: random action
            (rand::random::<f32>() * 3.0) as usize % 3
        } else {
            // Exploitation: argmax Q-value
            q_values.iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(idx, _)| idx)
                .unwrap_or(2)  // Default to Hold
        }
    }
    
    /// Decay exploration rate over time
    pub fn decay_exploration(&mut self, decay_rate: f32) {
        self.exploration_rate *= decay_rate;
        self.exploration_rate = self.exploration_rate.max(0.01);  // Minimum exploration
    }
}

#[async_trait]
impl MlStrategy for SimpleQNet {
    async fn predict(&self, state: &MarketState) -> Result<TradeDecision, MlError> {
        let features = state.to_features();
        let q_values = self.forward(&features)?;
        let action_idx = self.select_action(&q_values);
        
        let action = TradeAction::from_index(action_idx);
        let confidence = (q_values[action_idx].max(0.0) / 10.0).min(1.0);
        
        Ok(TradeDecision {
            action,
            confidence,
            amount: (state.agent_capital * 10000.0) as u64,
            model_id: self.metadata.name.clone(),
        })
    }
    
    fn update_weights(&mut self, _gradients: &[f32]) -> Result<(), MlError> {
        // Placeholder for training integration
        Ok(())
    }
    
    fn serialize(&self) -> Result<Vec<u8>, MlError> {
        serde_json::to_vec(&self.weights)
            .map_err(|e| MlError::SerializationError(e.to_string()))
    }
    
    fn deserialize(&mut self, data: &[u8]) -> Result<(), MlError> {
        self.weights = serde_json::from_slice(data)
            .map_err(|e| MlError::SerializationError(e.to_string()))?;
        Ok(())
    }
    
    fn metadata(&self) -> ModelMetadata {
        self.metadata.clone()
    }
    
    fn mutate(&mut self, mutation_rate: f32, _mutation_strength: f32) -> Result<(), MlError> {
        // Apply random mutations to weights
        for weight in self.weights.iter_mut() {
            if rand::random::<f32>() < mutation_rate {
                *weight += (rand::random::<f32>() - 0.5) * 0.1;
            }
        }
        self.metadata.mutation_count += 1;
        Ok(())
    }
    
    fn clone_box(&self) -> Box<dyn MlStrategy> {
        Box::new(self.clone())
    }
}
```

---

### src/finance/ml/training/mod.rs

```rust
//! Training infrastructure for ML models

#[cfg(feature = "ml")]
pub mod replay_buffer;

#[cfg(feature = "ml")]
pub mod optimizer;
```

---

### src/finance/ml/training/replay_buffer.rs

```rust
//! Experience replay buffer for training

use crate::finance::ml::traits::MarketState;
use std::collections::VecDeque;
use serde::{Serialize, Deserialize};

/// A single training experience
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Experience {
    pub state: MarketState,
    pub action: usize,
    pub reward: f32,
    pub next_state: MarketState,
    pub done: bool,
}

/// Replay buffer for experience memory
pub struct ReplayBuffer {
    buffer: VecDeque<Experience>,
    capacity: usize,
}

impl ReplayBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
        }
    }
    
    pub fn push(&mut self, experience: Experience) {
        if self.buffer.len() >= self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(experience);
    }
    
    pub fn sample(&self, batch_size: usize) -> Vec<Experience> {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        
        self.buffer
            .iter()
            .choose_multiple(&mut rng, batch_size.min(self.buffer.len()))
            .into_iter()
            .cloned()
            .collect()
    }
    
    pub fn len(&self) -> usize {
        self.buffer.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
    
    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_replay_buffer_push_and_sample() {
        let mut buffer = ReplayBuffer::new(10);
        
        for i in 0..5 {
            buffer.push(Experience {
                state: MarketState {
                    prices: vec![1.0],
                    volatility: vec![0.1],
                    agent_capital: 1.0,
                    scar_count: 0,
                    win_loss_ratio: 0.5,
                    timestamp: i as u64,
                },
                action: i % 3,
                reward: i as f32,
                next_state: MarketState {
                    prices: vec![1.1],
                    volatility: vec![0.1],
                    agent_capital: 1.0,
                    scar_count: 0,
                    win_loss_ratio: 0.5,
                    timestamp: (i + 1) as u64,
                },
                done: false,
            });
        }
        
        assert_eq!(buffer.len(), 5);
        
        let sample = buffer.sample(3);
        assert_eq!(sample.len(), 3);
    }
}
```

---

### src/finance/ml/training/optimizer.rs

```rust
//! Training optimizer stub

use crate::finance::ml::models::q_net::SimpleQNet;
use crate::finance::ml::training::replay_buffer::{ReplayBuffer, Experience};
use crate::finance::ml::errors::MlError;

/// ML Training handler
pub struct MLTrainer {
    pub model: SimpleQNet,
    pub replay_buffer: ReplayBuffer,
    pub gamma: f32,              // Discount factor
    pub learning_rate: f32,      // Learning rate
    pub batch_size: usize,       // Batch size
}

impl MLTrainer {
    pub fn new(model: SimpleQNet) -> Self {
        Self {
            model,
            replay_buffer: ReplayBuffer::new(10_000),
            gamma: 0.99,
            learning_rate: 0.001,
            batch_size: 32,
        }
    }
    
    pub fn train_step(&mut self) -> Result<f32, MlError> {
        if self.replay_buffer.len() < self.batch_size {
            return Ok(0.0);  // Not enough data yet
        }
        
        let _batch = self.replay_buffer.sample(self.batch_size);
        
        // Placeholder: training loop will be implemented with tch-rs
        Ok(0.0)
    }
    
    /// Calculate reward from market outcome
    pub fn calculate_reward(
        roi: f32,
        scar_count: u32,
        drawdown: f32,
    ) -> f32 {
        let scar_penalty = scar_count as f32 * 0.05;
        let drawdown_penalty = drawdown.abs() * 0.1;
        (roi / 100.0) - scar_penalty - drawdown_penalty
    }
}
```

---

### src/finance/ml/evolution/mod.rs

```rust
//! Evolution and mutation mechanics

#[cfg(feature = "ml")]
pub mod mutation;
```

---

### src/finance/ml/evolution/mutation.rs

```rust
//! Mutation and evolution mechanics for spawning

use crate::finance::ml::models::q_net::SimpleQNet;
use crate::finance::ml::errors::MlError;

/// Configuration for offspring spawning
pub struct SpawningConfig {
    pub mutation_rate: f32,          // Probability of weight mutation
    pub mutation_strength: f32,      // Magnitude of noise
    pub survival_threshold: f32,     // ROI threshold for spawning
    pub max_offspring: usize,        // Max offspring per parent
}

impl Default for SpawningConfig {
    fn default() -> Self {
        Self {
            mutation_rate: 0.15,
            mutation_strength: 0.02,
            survival_threshold: 5.0,
            max_offspring: 2,
        }
    }
}

impl SpawningConfig {
    /// Create offspring from parent with mutations
    pub fn spawn_offspring(&self, parent: &SimpleQNet) -> Result<SimpleQNet, MlError> {
        let mut offspring = parent.clone();
        offspring.mutate(self.mutation_rate, self.mutation_strength)?;
        Ok(offspring)
    }
}
```

---

### src/finance/ml/integration/mod.rs

```rust
//! Integration with lineage-finance agent system

#[cfg(feature = "ml")]
pub mod agent_lifecycle;
```

---

### src/finance/ml/integration/agent_lifecycle.rs

```rust
//! ML agent lifecycle integration

use crate::finance::ml::models::q_net::SimpleQNet;
use crate::finance::ml::traits::{MlStrategy, MarketState, TradeAction};
use crate::finance::ml::errors::MlError;

/// Apply learning penalty for losses (scar damage)
pub fn apply_scar_damage(model: &mut SimpleQNet, scar_count: u32) -> Result<(), MlError> {
    match scar_count {
        0 => {},  // No damage
        1..=2 => {
            // Light damage: increase exploration
            model.exploration_rate = (model.exploration_rate + 0.05).min(0.9);
        }
        3..=5 => {
            // Moderate damage: increase exploration + small mutation
            model.exploration_rate = (model.exploration_rate + 0.1).min(0.9);
            model.mutate(0.1, 0.01)?;
        }
        _ => {
            // Severe damage: heavy mutation
            model.mutate(0.5, 0.05)?;
            model.exploration_rate = 0.5;
        }
    }
    Ok(())
}

/// Check if agent should spawn offspring
pub fn should_spawn_offspring(roi: f32, survival_threshold: f32) -> bool {
    roi > survival_threshold
}
```

---

## 4. Update src/finance/mod.rs

Add ML module export:

```rust
// Existing exports
pub mod agent;
pub mod traits;
pub mod data_providers;
pub mod market_data;
pub mod metrics;
pub mod scars;
pub mod spawning;
pub mod trade;
pub mod trust_scoring;
pub mod visualization;
pub mod advanced;

// Add ML module
#[cfg(feature = "ml")]
pub mod ml;
```

---

## 5. First Compilation Test

```bash
# Test without ML (should work immediately)
cargo check

# Output should be:
# Compiling lineage-rs v0.2.1
# Finished `dev` profile [unoptimized + debuginfo] target(s) in X.XXs

# Test with ML (tests libtorch)
cargo check --features ml

# Output should complete without link errors
```

---

## 6. Verification

Run this to confirm everything is working:

```bash
cargo test --features ml --lib ml::models::q_net
```

Expected output:
```
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; ...
```

---

## 7. Next Steps

Once Phase 1 compiles successfully:

1. ✓ Dependency integration complete
2. ✓ ML module structure in place
3. ✓ Core traits defined
4. ✓ Basic Q-Network stub ready
5. **Next**: Implement tch-rs neural network in Phase 2

See `ML_IMPLEMENTATION_INTERNAL.md` for Phase 2 guidance.

---

**Phase 1 Status**: Ready for coding  
**Time Estimate**: 30 minutes setup + 1 hour coding  
**Target**: Compiling code with all ML stubs in place
