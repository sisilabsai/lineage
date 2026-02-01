//! # Lineage Finance - Evolutionary AI Trading Platform
//!
//! This module extends Lineage with financial domain concepts:
//! - Irreversible trades with no rollback capability
//! - Finite capital (energy) as resource constraint
//! - Permanent damage from losses (scars)
//! - Evolutionary spawning of successful agents
//! - Trust scoring based on performance history
//! - Multi-agent competitive simulations
//!
//! ## Core Philosophy
//!
//! Financial agents in this system face real consequences:
//! - Losses leave permanent scars that increase future costs
//! - Poor performance leads to death (zero capital)
//! - Successful agents spawn inheriting optimized traits
//! - History is append-only and auditable forever

pub mod agent;
pub mod trade;
pub mod scars;
pub mod spawning;
pub mod trust_scoring;
pub mod arena;
pub mod advanced;

// Re-export core types
pub use agent::{FinanceAgent, FinanceAgentStatus, AgentMetrics};
pub use trade::{Trade, TradeOperation, TradeResult, TradeDirection, ExecutionError};
pub use scars::{FinancialScar, ScarImpact, FinancialDamage};
pub use spawning::{Offspring, OffspringTraits, InheritanceStrategy};
pub use trust_scoring::{PerformanceScore, TrustFormula, TrustGrant};
pub use arena::{Arena, CompetitionResult, MarketState};
pub use advanced::{BlockchainHook, EvolutionaryStrategy, GovernanceVote};

/// Configuration for finance agents
#[derive(Debug, Clone)]
pub struct FinanceConfig {
    /// Percentage of trade value lost as fee (0-100)
    pub fee_percentage: f32,
    
    /// Maximum leverage allowed for an agent
    pub max_leverage: f32,
    
    /// Drawdown percentage threshold that triggers scars
    pub scar_threshold: f32,
    
    /// Cost multiplier per scar (increments trading costs)
    pub scar_cost_multiplier: f32,
    
    /// Minimum capital to spawn offspring
    pub min_spawn_capital: u64,
    
    /// Percentage of capital required to spawn (is lost in spawn)
    pub spawn_cost_percentage: f32,
}

impl Default for FinanceConfig {
    fn default() -> Self {
        FinanceConfig {
            fee_percentage: 0.1,           // 0.1% fees
            max_leverage: 5.0,             // 5x leverage max
            scar_threshold: 5.0,           // 5% drawdown triggers scar
            scar_cost_multiplier: 1.05,   // 5% cost increase per scar
            min_spawn_capital: 10000,
            spawn_cost_percentage: 25.0,   // 25% capital cost to spawn
        }
    }
}
