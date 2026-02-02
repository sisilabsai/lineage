# ML Implementation Internal Document - Lineage Finance
## Darwinian DeFi: Breaking Platforms with Adaptive Neural Agents

**Status**: Phase 1 Planning & Architecture  
**Date**: February 2, 2026  
**libtorch**: Downloaded and extracted to `./libtorch/`  
**tch-rs Integration**: Ready for deployment  

---

## Executive Summary

This document outlines the strategic implementation of machine learning-driven trading agents within the lineage-finance library. The goal is to replace rule-based strategies with neural networks that:
- **Learn** from real market data via CoinMarketCap/CoinGecko integration
- **Evolve** through genetic inheritance (parent spawning)
- **Accumulate consequences** (scars from losses)
- **Make irreversible decisions** (append-only trades in blockchain-style ledger)
- **Self-select** through evolutionary pressure (death/spawn cycles)

This creates a "Darwinian DeFi" ecosystem where AI agents outperform static bots through learned behavior and cryptographic accountability.

---

## Phase 1: Foundation & Architecture

### 1.1 Dependency Integration

**Status**: [ ] Not Started

- [ ] Add `tch-rs` to `Cargo.toml` dependencies with libtorch path
- [ ] Configure environment variables:
  - `LIBTORCH` → `D:\Projects\Lineage\libtorch`
  - `LD_LIBRARY_PATH` (Linux) / `DYLD_LIBRARY_PATH` (macOS) / PATH (Windows)
- [ ] Create `build.rs` to validate libtorch availability
- [ ] Add `tch` feature flag to make ML optional (no bloat for basic users)

**Implementation Steps**:
```toml
[dependencies]
tch = "0.15"  # PyTorch bindings for Rust

[features]
default = []
ml = []  # Feature flag for ML agents
```

**build.rs**:
```rust
fn main() {
    #[cfg(feature = "ml")]
    {
        let libtorch_path = std::env::var("LIBTORCH")
            .expect("LIBTORCH env var not set. Download from: https://download.pytorch.org/libtorch/cpu/libtorch-cxx11-abi-shared-with-deps-latest.zip");
        println!("cargo:rustc-env=LIBTORCH_PATH={}", libtorch_path);
    }
}
```

---

### 1.2 ML Module Architecture

**Status**: [ ] Not Started

Create modular structure:
```
src/finance/ml/
├── mod.rs              # Public API
├── traits.rs           # MlStrategy trait (extensible)
├── models/
│   ├── mod.rs
│   ├── q_net.rs        # Q-learning network
│   ├── policy_net.rs   # Policy gradient network
│   └── base.rs         # Base neural network structures
├── training/
│   ├── mod.rs
│   ├── replay_buffer.rs
│   ├── loss_functions.rs
│   └── optimizer.rs
├── evolution/
│   ├── mod.rs
│   ├── mutation.rs     # Gaussian noise mutations
│   ├── crossover.rs    # Parent-offspring crossover
│   └── fitness.rs      # Evaluation metrics
└── integration/
    ├── mod.rs
    ├── agent_lifecycle.rs  # Hook into FinanceAgent
    ├── scar_damage.rs      # Model penalties for losses
    └── trust_integration.rs # ML performance → trust score
```

**Trait Definition** (`traits.rs`):
```rust
use async_trait::async_trait;
use crate::finance::{MarketSnapshot, TradeDecision};

/// Core trait for all ML-based trading strategies
/// Allows users to implement custom models while maintaining
/// lineage's append-only state semantics
#[async_trait]
pub trait MlStrategy: Send + Sync {
    /// Predict action (Buy, Sell, Hold) from market state
    async fn predict(&self, state: &MarketState) -> TradeDecision;
    
    /// Update model weights (called during training)
    fn update_weights(&mut self, gradients: &[f32]) -> Result<(), MlError>;
    
    /// Serialize model state (for inheritance/spawning)
    fn serialize(&self) -> Vec<u8>;
    
    /// Deserialize model state (from parent)
    fn deserialize(&mut self, data: &[u8]) -> Result<(), MlError>;
    
    /// Get model metadata (name, parameters)
    fn metadata(&self) -> ModelMetadata;
    
    /// Apply mutation (for evolution)
    fn mutate(&mut self, mutation_rate: f32) -> Result<(), MlError>;
}

/// Market state representation for ML models
pub struct MarketState {
    pub prices: Vec<f32>,           // Normalized prices
    pub volatility: Vec<f32>,       // Historical volatility
    pub agent_capital: f32,         // Normalized capital
    pub scar_count: u32,            // Consequence indicators
    pub win_loss_ratio: f32,        // Recent performance
    pub timestamp: u64,
}

/// Trade decision with confidence
pub struct TradeDecision {
    pub action: TradeAction,        // Buy, Sell, Hold
    pub confidence: f32,            // [0.0, 1.0]
    pub amount: u64,                // Capital to deploy
    pub model_id: String,           // Audit trail
}

pub enum TradeAction {
    Buy,
    Sell,
    Hold,
}

pub struct ModelMetadata {
    pub name: String,
    pub version: String,
    pub parent_id: Option<String>,  // Lineage tracking
    pub mutation_count: u32,
    pub generations: u32,
}
```

---

## Phase 2: Core ML Models

### 2.1 SimpleQNet Implementation

**Status**: [ ] Not Started

A lightweight Q-learning network for discrete action spaces (Buy, Sell, Hold).

**File**: `src/finance/ml/models/q_net.rs`

```rust
use tch::{nn, Tensor, Kind};
use crate::finance::ml::traits::{MlStrategy, MarketState, TradeDecision};

/// Simple Q-Network for RL-based trading
/// Input: Market state (4D) → Hidden layers → Output Q-values (3 actions)
pub struct SimpleQNet {
    vs: nn::VarStore,
    net: QNetModule,
    exploration_rate: f32,  // Epsilon-greedy
}

pub struct QNetModule {
    fc1: nn::Linear,
    fc2: nn::Linear,
    fc3: nn::Linear,  // Output layer (3 actions: Buy, Sell, Hold)
}

impl SimpleQNet {
    /// Create new Q-network with random initialization
    pub fn new(vs: &nn::Path, input_size: i64, hidden_size: i64) -> Self {
        let net = QNetModule {
            fc1: nn::linear(vs, input_size, hidden_size, Default::default()),
            fc2: nn::linear(vs, hidden_size, hidden_size, Default::default()),
            fc3: nn::linear(vs, hidden_size, 3, Default::default()),  // 3 actions
        };
        
        Self {
            vs: vs.vstore().clone(),
            net,
            exploration_rate: 0.1,  // 10% exploration
        }
    }
    
    /// Forward pass: state → Q-values for each action
    pub fn forward(&self, state: &Tensor) -> Tensor {
        let x = state.apply(&self.net.fc1).relu();
        let x = x.apply(&self.net.fc2).relu();
        x.apply(&self.net.fc3)
    }
    
    /// Epsilon-greedy action selection
    pub fn select_action(&self, q_values: &Tensor) -> usize {
        if rand::random::<f32>() < self.exploration_rate {
            // Exploration: random action
            (rand::random::<f32>() * 3.0) as usize
        } else {
            // Exploitation: argmax Q-value
            let (_, action) = q_values.max_other_dims(&[], false, true);
            i64::from(&action) as usize
        }
    }
    
    /// Decay exploration rate over time
    pub fn decay_exploration(&mut self, decay_rate: f32) {
        self.exploration_rate *= decay_rate;
    }
}

#[async_trait]
impl MlStrategy for SimpleQNet {
    async fn predict(&self, state: &MarketState) -> TradeDecision {
        let state_tensor = Tensor::of_slice(&state.to_features());
        let q_values = self.forward(&state_tensor);
        let action_idx = self.select_action(&q_values);
        
        let action = match action_idx {
            0 => TradeAction::Buy,
            1 => TradeAction::Sell,
            _ => TradeAction::Hold,
        };
        
        let confidence = f32::from(q_values.max());
        
        TradeDecision {
            action,
            confidence,
            amount: (state.agent_capital * 0.5) as u64,  // Deploy 50% capital
            model_id: self.metadata().name,
        }
    }
    
    fn update_weights(&mut self, gradients: &[f32]) -> Result<(), MlError> {
        // Called during training loop with loss gradients
        Ok(())
    }
    
    fn serialize(&self) -> Vec<u8> {
        // Save weights to bytes for spawning
        Vec::new()
    }
    
    fn deserialize(&mut self, _data: &[u8]) -> Result<(), MlError> {
        Ok(())
    }
    
    fn metadata(&self) -> ModelMetadata {
        ModelMetadata {
            name: "SimpleQNet".to_string(),
            version: "1.0".to_string(),
            parent_id: None,
            mutation_count: 0,
            generations: 0,
        }
    }
    
    fn mutate(&mut self, mutation_rate: f32) -> Result<(), MlError> {
        // Apply Gaussian noise to weights
        Ok(())
    }
}
```

### 2.2 Market State Normalization

**Status**: [ ] Not Started

**File**: `src/finance/ml/models/base.rs`

```rust
impl MarketState {
    /// Convert to tensor-compatible features
    pub fn to_features(&self) -> Vec<f32> {
        vec![
            // Normalize prices: (price - mean) / std
            self.prices.iter().sum::<f32>() / self.prices.len() as f32,
            // Volatility indicators
            self.volatility.iter().sum::<f32>() / self.volatility.len() as f32,
            // Normalized capital: 0-1 scale
            self.agent_capital.min(100_000.0) / 100_000.0,
            // Scar penalty: 0-1 (0 scars = 1.0, many scars = 0.0)
            (1.0 - (self.scar_count as f32 / 10.0).min(1.0)),
            // Win/loss momentum
            self.win_loss_ratio,
        ]
    }
    
    /// From market snapshot → market state
    pub fn from_snapshot(
        snapshot: &MarketSnapshot,
        agent_capital: f32,
        scar_count: u32,
        win_loss_ratio: f32,
    ) -> Self {
        Self {
            prices: snapshot.prices.values()
                .map(|&p| (p as f32).ln())  // Log prices for stability
                .collect(),
            volatility: snapshot.volatility.values()
                .map(|&v| v as f32)
                .collect(),
            agent_capital,
            scar_count,
            win_loss_ratio,
            timestamp: snapshot.timestamp,
        }
    }
}
```

---

## Phase 3: Training & Optimization

### 3.1 Experience Replay Buffer

**Status**: [ ] Not Started

**File**: `src/finance/ml/training/replay_buffer.rs`

```rust
use std::collections::VecDeque;
use rand::seq::SliceRandom;

pub struct Experience {
    pub state: MarketState,
    pub action: usize,
    pub reward: f32,          // ROI - scar_penalty
    pub next_state: MarketState,
    pub done: bool,           // Episode terminal
}

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
}
```

### 3.2 Training Loop Integration

**Status**: [ ] Not Started

**File**: `src/finance/ml/training/optimizer.rs`

```rust
pub struct MLTrainer {
    model: SimpleQNet,
    replay_buffer: ReplayBuffer,
    optimizer: nn::Optimizer,
    gamma: f32,              // Discount factor (0.99)
    learning_rate: f32,      // 0.001
    batch_size: usize,       // 32
}

impl MLTrainer {
    pub fn new(model: SimpleQNet) -> Self {
        let optimizer = nn::Adam::default()
            .build(&model.vs, 0.001)
            .expect("Failed to build optimizer");
        
        Self {
            model,
            replay_buffer: ReplayBuffer::new(10_000),
            optimizer,
            gamma: 0.99,
            learning_rate: 0.001,
            batch_size: 32,
        }
    }
    
    /// Train on batch from experience replay
    pub fn train_step(&mut self) -> f32 {
        if self.replay_buffer.len() < self.batch_size {
            return 0.0;  // Not enough data yet
        }
        
        let batch = self.replay_buffer.sample(self.batch_size);
        let mut total_loss = 0.0;
        
        for experience in batch {
            // Compute Q-learning loss
            let state_tensor = Tensor::of_slice(&experience.state.to_features());
            let q_values = self.model.forward(&state_tensor);
            let target_q = experience.reward 
                + self.gamma * q_values.max();
            
            let loss = tch::Tensor::mse_loss(&q_values, &target_q, tch::Reduction::Mean);
            
            self.optimizer.zero_grad();
            loss.backward();
            self.optimizer.step();
            
            total_loss += f32::from(loss);
        }
        
        total_loss / self.batch_size as f32
    }
    
    /// Reward function: ROI - scar penalties
    pub fn calculate_reward(roi: f32, scar_count: u32, drawdown: f32) -> f32 {
        let scar_penalty = scar_count as f32 * 0.05;  // -5% per scar
        let drawdown_penalty = drawdown.abs() * 0.1;  // -10% per 1% drawdown
        (roi / 100.0) - scar_penalty - drawdown_penalty
    }
}
```

---

## Phase 4: Evolutionary Mechanics

### 4.1 Model Mutation for Spawning

**Status**: [ ] Not Started

**File**: `src/finance/ml/evolution/mutation.rs`

```rust
use rand_distr::Normal;

pub trait Mutatable {
    fn mutate(&mut self, mutation_rate: f32, mutation_strength: f32);
}

impl Mutatable for SimpleQNet {
    fn mutate(&mut self, mutation_rate: f32, mutation_strength: f32) {
        // For each weight in the network:
        // With probability mutation_rate, add Gaussian noise
        
        let normal = Normal::new(0.0, mutation_strength)
            .expect("Invalid normal distribution");
        
        for named_tensor in self.vs.variables() {
            if rand::random::<f32>() < mutation_rate {
                let mut tensor = named_tensor.1.get();
                let noise = Tensor::randn_like(&tensor);
                tensor = tensor + (noise * mutation_strength);
                self.vs.set_variable(named_tensor.0, &tensor);
            }
        }
    }
}

pub struct SpawningConfig {
    pub mutation_rate: f32,          // 0.1-0.3
    pub mutation_strength: f32,      // 0.01-0.05
    pub survival_threshold: f32,     // ROI > 10%
    pub max_offspring: usize,        // 2-3 per parent
}

impl SpawningConfig {
    pub fn spawn_offspring(
        parent: &SimpleQNet,
        agent_capital: f32,
    ) -> SimpleQNet {
        let mut offspring = parent.clone();
        offspring.mutate(
            self.mutation_rate,
            self.mutation_strength,
        );
        
        // Update metadata
        let mut metadata = offspring.metadata();
        metadata.mutation_count += 1;
        metadata.generations += 1;
        
        offspring
    }
}
```

### 4.2 Scar-Based Model Damage

**Status**: [ ] Not Started

**File**: `src/finance/ml/integration/scar_damage.rs`

```rust
/// Apply learning penalty for losses (increase exploration)
pub fn apply_scar_damage(model: &mut SimpleQNet, scar: &Scar) {
    match scar.severity {
        ScarSeverity::Light => {
            // Increase exploration: 0.1 → 0.15
            model.exploration_rate = (model.exploration_rate + 0.05).min(0.9);
        }
        ScarSeverity::Moderate => {
            // Moderate increase + small weight noise
            model.exploration_rate = (model.exploration_rate + 0.1).min(0.9);
            model.mutate(0.1, 0.01);  // Light perturbation
        }
        ScarSeverity::Severe => {
            // Heavy reset: 50% of weights randomized
            model.mutate(0.5, 0.05);
            model.exploration_rate = 0.5;  // High exploration
        }
    }
}

/// Apply trust penalty for underperforming models
pub fn update_trust_from_model_performance(
    agent: &mut FinanceAgent,
    model_performance: f32,  // ROI over recent trades
) {
    let performance_score = (1.0 + model_performance / 100.0).max(0.1);
    agent.trust_score = agent.trust_score * performance_score;
}
```

---

## Phase 5: Integration with FinanceAgent

### 5.1 Agent Lifecycle Hooks

**Status**: [ ] Not Started

**File**: `src/finance/ml/integration/agent_lifecycle.rs`

```rust
/// Extend FinanceAgent with ML model
pub struct MlFinanceAgent {
    pub base: FinanceAgent,
    pub model: Box<dyn MlStrategy>,
    pub trainer: MLTrainer,
    pub episode_history: VecDeque<Experience>,
}

impl MlFinanceAgent {
    pub async fn decide_trade_ml(&mut self, market: &MarketSnapshot) -> TradeDecision {
        // Convert market snapshot to ML state
        let ml_state = MarketState::from_snapshot(
            market,
            self.base.metrics.capital as f32,
            self.base.scars.total_count(),
            self.calculate_win_loss_ratio(),
        );
        
        // Predict action
        self.model.predict(&ml_state).await
    }
    
    pub async fn execute_trade_and_learn(&mut self, market: &MarketSnapshot) {
        let decision = self.decide_trade_ml(market).await;
        let old_capital = self.base.metrics.capital;
        
        // Execute trade (irreversible)
        match decision.action {
            TradeAction::Buy => {
                self.base.execute_trade(TradeDirection::Buy, decision.amount);
            }
            TradeAction::Sell => {
                self.base.execute_trade(TradeDirection::Sell, decision.amount);
            }
            TradeAction::Hold => {}
        }
        
        // Observe outcome and learn
        let roi = ((self.base.metrics.capital as f32 / old_capital as f32) - 1.0) * 100.0;
        let reward = MLTrainer::calculate_reward(
            roi,
            self.base.scars.total_count(),
            self.base.metrics.current_drawdown(),
        );
        
        // Store experience for training
        let experience = Experience {
            state: ml_state,
            action: decision_to_action(&decision),
            reward,
            next_state: MarketState::from_snapshot(
                market,
                self.base.metrics.capital as f32,
                self.base.scars.total_count(),
                self.calculate_win_loss_ratio(),
            ),
            done: self.base.metrics.capital == 0,
        };
        
        self.trainer.replay_buffer.push(experience);
        
        // Train on batch
        if self.trainer.replay_buffer.len() % 32 == 0 {
            let loss = self.trainer.train_step();
            println!("Training loss: {:.4}", loss);
        }
    }
}
```

### 5.2 Death & Spawn Mechanics

**Status**: [ ] Not Started

**File**: `src/finance/ml/integration/agent_lifecycle.rs` (continued)

```rust
pub fn check_and_spawn_offspring(
    agents: &mut Vec<MlFinanceAgent>,
    config: &SpawningConfig,
) {
    let mut offspring_queue = Vec::new();
    
    for agent in agents.iter() {
        // Check survival criteria
        let roi = agent.calculate_roi();
        
        if roi > config.survival_threshold && agent.base.metrics.capital > 0 {
            // Agent survived: spawn offspring
            for _ in 0..config.max_offspring {
                let child_model = config.spawn_offspring(
                    &agent.model,
                    agent.base.metrics.capital / 2,  // Split capital
                );
                
                offspring_queue.push(child_model);
            }
        }
    }
    
    // Remove dead agents
    agents.retain(|a| a.base.metrics.capital > 0);
    
    // Add offspring
    for child_model in offspring_queue {
        let child_agent = MlFinanceAgent {
            base: FinanceAgent::new(),
            model: Box::new(child_model),
            trainer: MLTrainer::new(child_model),
            episode_history: VecDeque::new(),
        };
        agents.push(child_agent);
    }
}
```

---

## Phase 6: Arena Integration

### 6.1 Arena with ML Agents

**Status**: [ ] Not Started

**File**: `examples/arena_with_ml_agents.rs`

```rust
#[tokio::main]
async fn main() {
    // Initialize market data fetcher
    let fetcher = MultiProviderFetcher::new(Some(api_key));
    
    // Create initial population of ML agents
    let mut agents = Vec::new();
    for i in 0..5 {
        let model = SimpleQNet::new(&vs, 5, 64);  // 5 inputs, 64 hidden
        let agent = MlFinanceAgent {
            base: FinanceAgent::new(),
            model: Box::new(model),
            trainer: MLTrainer::new(model),
            episode_history: VecDeque::new(),
        };
        agents.push(agent);
    }
    
    let spawning_config = SpawningConfig {
        mutation_rate: 0.15,
        mutation_strength: 0.02,
        survival_threshold: 5.0,  // 5% ROI
        max_offspring: 2,
    };
    
    // Run arena for N rounds
    for round in 0..50 {
        // Fetch market data
        let market = fetcher.fetch_prices(&["BTC", "ETH"]).await.unwrap();
        
        // Each agent trades
        for agent in agents.iter_mut() {
            agent.execute_trade_and_learn(&market).await;
        }
        
        // Check death/spawning
        check_and_spawn_offspring(&mut agents, &spawning_config);
        
        // Visualization
        visualize_generation(&agents, round);
        
        println!("Round {}: {} agents alive", round, agents.len());
    }
}
```

---

## Phase 7: Testing & Validation

### 7.1 Unit Tests

**Status**: [ ] Not Started

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_market_state_normalization() {
        let state = MarketState {
            prices: vec![50000.0, 3000.0],
            volatility: vec![35.0, 45.0],
            agent_capital: 50000.0,
            scar_count: 2,
            win_loss_ratio: 0.6,
            timestamp: 0,
        };
        
        let features = state.to_features();
        assert_eq!(features.len(), 5);
        assert!(features.iter().all(|f| f.is_finite()));
    }
    
    #[test]
    fn test_model_mutation() {
        let mut model = SimpleQNet::new(&vs, 5, 64);
        let weights_before = model.serialize();
        
        model.mutate(1.0, 0.1);  // 100% mutation rate
        let weights_after = model.serialize();
        
        assert_ne!(weights_before, weights_after);
    }
    
    #[test]
    fn test_scar_damage_increases_exploration() {
        let mut model = SimpleQNet::new(&vs, 5, 64);
        let initial_rate = model.exploration_rate;
        
        apply_scar_damage(&mut model, &create_test_scar());
        
        assert!(model.exploration_rate > initial_rate);
    }
}
```

### 7.2 Integration Tests

**Status**: [ ] Not Started

```rust
#[tokio::test]
async fn test_arena_ml_agents_complete_episode() {
    let mut agents = create_test_agents(3);
    let market = create_test_market();
    
    for _ in 0..10 {
        for agent in agents.iter_mut() {
            agent.execute_trade_and_learn(&market).await;
        }
    }
    
    // Verify all agents have made trades
    for agent in agents.iter() {
        assert!(agent.base.trade_history.len() > 0);
    }
}
```

---

## Phase 8: Challenges & Mitigations

### 8.1 libtorch Setup

| Challenge | Mitigation |
|-----------|-----------|
| **Missing libtorch env vars** | Pre-build script validates & sets vars automatically |
| **CPU vs GPU detection** | Auto-detect; provide `--features gpu` for CUDA |
| **Large model overhead** | Use lightweight nets: 64 hidden neurons, 1-2 hidden layers |
| **Tensor allocation failures** | Implement memory pooling for small reusable tensors |

### 8.2 Tensor & Training

| Challenge | Mitigation |
|-----------|-----------|
| **Gradient explosion** | Gradient clipping: clip to [-1.0, 1.0] |
| **Slow training convergence** | Use prioritized experience replay (importance weighting) |
| **Flat market (no signal)** | Epsilon-greedy forces exploration; reward stability over gains |
| **Overfitting to training data** | Cross-validation on holdout market periods |

### 8.3 Evolutionary Dynamics

| Challenge | Mitigation |
|-----------|-----------|
| **Premature convergence** | Maintain diversity: cull if offspring too similar |
| **Explosion of agent population** | Cap max agents; tournament selection for mating |
| **Loss of good genes** | Archive top performers; allow resurrection |
| **Scar penalties too harsh** | Tune scar_damage function per market conditions |

---

## Phase 9: Performance & Benchmarks

### 9.1 Target Metrics

```
Model Inference:    < 10ms per decision (real-time trading)
Training Step:      < 50ms per batch (32 experiences)
Memory per Model:   < 5MB (weights + gradients)
Peak Population:    100 agents max
Arena Full Run:     50 rounds in < 5 minutes
```

### 9.2 Profiling

**File**: `benches/ml_performance.rs`

```rust
#[bench]
fn bench_forward_pass(b: &mut Bencher) {
    let model = SimpleQNet::new(&vs, 5, 64);
    let state = MarketState::default();
    let tensor = Tensor::of_slice(&state.to_features());
    
    b.iter(|| model.forward(&tensor))
}

#[bench]
fn bench_train_step(b: &mut Bencher) {
    let mut trainer = create_test_trainer();
    b.iter(|| trainer.train_step())
}
```

---

## Phase 10: Roadmap & Checkpoints

### Checkpoint 1: Basic Setup (Week 1)
- [ ] Add `tch-rs` dependency
- [ ] Configure libtorch paths
- [ ] Create ML module structure
- [ ] Implement `MlStrategy` trait

**Deliverable**: Compiling code with no libtorch errors

### Checkpoint 2: Core Models (Week 2)
- [ ] Implement `SimpleQNet`
- [ ] Normalize market states
- [ ] Implement forward pass
- [ ] Test action selection

**Deliverable**: Model can predict Buy/Sell/Hold from market data

### Checkpoint 3: Training (Week 3)
- [ ] Experience replay buffer
- [ ] Q-learning loss function
- [ ] Training loop integration
- [ ] Adam optimizer setup

**Deliverable**: Model improves ROI over training episodes

### Checkpoint 4: Evolution (Week 4)
- [ ] Mutation mechanics
- [ ] Scar-based damage
- [ ] Spawning logic
- [ ] Death criteria

**Deliverable**: Agents evolve across generations

### Checkpoint 5: Arena Integration (Week 5)
- [ ] ML example with live market data
- [ ] Multi-agent competition
- [ ] Visualization of evolution
- [ ] Performance metrics

**Deliverable**: Fully functional Darwinian DeFi arena

### Checkpoint 6: Optimization (Week 6)
- [ ] Profiling & benchmarking
- [ ] Memory optimizations
- [ ] Training speed improvements
- [ ] Scalability tests

**Deliverable**: Meets all performance targets

---

## Implementation Execution Plan

### Step 1: Environment Setup
```bash
# Windows
$env:LIBTORCH = "D:\Projects\Lineage\libtorch"
$env:PATH = "$env:LIBTORCH\lib;$env:PATH"

# Verify
cargo check --features ml
```

### Step 2: Create Module Structure
```bash
mkdir -p src/finance/ml/{models,training,evolution,integration}
touch src/finance/ml/{mod.rs,traits.rs}
touch src/finance/ml/models/{mod.rs,base.rs,q_net.rs}
touch src/finance/ml/training/{mod.rs,replay_buffer.rs,optimizer.rs}
touch src/finance/ml/evolution/{mod.rs,mutation.rs}
touch src/finance/ml/integration/{mod.rs,agent_lifecycle.rs,scar_damage.rs}
```

### Step 3: Implement Incrementally
Each checkpoint builds on the previous. Test at each stage:
```bash
# Test compiles
cargo check --features ml

# Run unit tests
cargo test --features ml

# Benchmark inference
cargo bench --features ml --bench ml_performance
```

### Step 4: Integration Testing
Once each phase is complete, integrate into arena:
```bash
cargo run --example arena_with_ml_agents --release
```

---

## Success Criteria

1. **Compilation**: Code compiles with `tch-rs` without libtorch errors
2. **Basic Inference**: Model can predict actions from market state in < 10ms
3. **Training Convergence**: Model ROI improves over 100 training steps by > 5%
4. **Evolution**: Population converges to high-ROI offspring after 5+ generations
5. **Arena Performance**: ML agents outperform rule-based strategies by > 20% ROI
6. **Scalability**: Supports 100+ agents with < 5% slowdown
7. **Visualization**: Charting lineage, ROI, scar distribution, model mutations

---

## Risk Mitigation

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|-----------|
| libtorch linking fails | Medium | Critical | Pre-build CI, docker environment |
| Training doesn't converge | Medium | High | Start with supervised learning warmup |
| Population explosion | Low | Medium | Hard cap + tournament selection |
| Market crashes model | Low | High | Epsilon-greedy + manual circuit breaker |
| Reproducibility issues | Low | Medium | Seed RNG, version lock dependencies |

---

## Next Actions

1. **This Week**: 
   - [ ] Set up `tch-rs` with libtorch
   - [ ] Implement trait definitions
   - [ ] Create `SimpleQNet` skeleton

2. **Next Week**:
   - [ ] Full forward pass & training loop
   - [ ] Integration tests with mock market data

3. **Following Week**:
   - [ ] Spawning & evolution
   - [ ] Live arena demo

---

## Appendix: Resources

- **tch-rs Documentation**: https://github.com/LaurentMazare/tch-rs
- **PyTorch Q-Learning**: https://pytorch.org/tutorials/intermediate/reinforcement_q_learning.html
- **Lineage Crate**: https://docs.rs/lineage-rs/latest/lineage/
- **CoinMarketCap API**: https://coinmarketcap.com/api/documentation/v1/
- **Performance Tips**: Small networks (64 hidden), batch training, weight clipping

---

**Document Status**: Ready for Phase 1 Implementation  
**Last Updated**: February 2, 2026  
**Next Review**: After Checkpoint 1 completion
