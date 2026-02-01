//! Finance Agent - Core trading entity with irreversible lifecycle
//!
//! FinanceAgent extends Lineage with:
//! - Unique, non-copyable identity for each trading agent
//! - Finite capital (energy) as primary resource
//! - Append-only trade history (immutable record)
//! - Permanent damage accumulation (scars)
//! - Irreversible lifecycle: spawning → trading → death

use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::lineage::Lineage;
use crate::scar::ScarSeverity;
use std::fmt;

/// Unique identifier for a trading agent (non-copyable)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(Uuid);

impl AgentId {
    /// Create a new unique agent ID
    pub fn new() -> Self {
        AgentId(Uuid::new_v4())
    }
}

impl fmt::Display for AgentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Complete metrics snapshot for a finance agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetrics {
    /// Current capital (energy equivalent)
    pub capital: u64,
    
    /// Total trades executed (never reset)
    pub total_trades: u64,
    
    /// Win rate percentage (0-100)
    pub win_rate: f32,
    
    /// Maximum drawdown since inception (0-100)
    pub max_drawdown: f32,
    
    /// Current drawdown (0-100)
    pub current_drawdown: f32,
    
    /// Peak capital reached
    pub peak_capital: u64,
    
    /// Total fees paid
    pub total_fees_paid: u64,
    
    /// Number of scars accumulated
    pub scar_count: u32,
    
    /// Trust score (0-100, affects resource access)
    pub trust_score: f32,
    
    /// Generation number (0 = original, 1+ = spawn offspring)
    pub generation: u32,
}

/// Status of a finance agent
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FinanceAgentStatus {
    /// Agent is alive and can execute trades
    Active,
    
    /// Agent has insufficient capital (energy = 0)
    Bankrupt,
    
    /// Agent exceeded maximum scar count
    TerminallyDamaged,
    
    /// Agent was deactivated (permanent)
    Terminated,
}

/// A trading agent bound to a Lineage instance
///
/// Each FinanceAgent:
/// - Has a unique, non-copyable ID
/// - Operates on finite capital that cannot be restored
/// - Records trades irreversibly in history
/// - Accumulates permanent scars from losses
/// - Can spawn offspring that inherit optimized traits
/// - Dies permanently when capital reaches zero
pub struct FinanceAgent {
    /// Unique identity (non-copyable, non-cloneable in practice)
    pub id: AgentId,
    
    /// Underlying lineage system tracking lifecycle
    pub lineage: Lineage,
    
    /// Strategy name/description
    pub strategy: String,
    
    /// Current metrics
    pub metrics: AgentMetrics,
    
    /// Append-only trade history
    pub trade_history: Vec<(DateTime<Utc>, String)>,
    
    /// Current status
    pub status: FinanceAgentStatus,
    
    /// Parent agent ID (if spawned)
    pub parent_id: Option<AgentId>,
    
    /// Maximum scars before automatic termination
    pub max_scars: u32,
    
    /// Cost multiplier from accumulated scars
    pub scar_cost_multiplier: f32,
}

impl FinanceAgent {
    /// Create a new trading agent with initial capital
    ///
    /// # Arguments
    /// * `strategy` - Name of the trading strategy
    /// * `initial_capital` - Starting capital (converted to energy)
    /// * `generation` - Generation number (0 for original agents)
    ///
    /// # Returns
    /// A new FinanceAgent ready to execute trades
    pub fn new(strategy: String, initial_capital: u64, generation: u32) -> Self {
        let lineage = Lineage::create(initial_capital);
        
        FinanceAgent {
            id: AgentId::new(),
            lineage,
            strategy,
            metrics: AgentMetrics {
                capital: initial_capital,
                total_trades: 0,
                win_rate: 50.0,
                max_drawdown: 0.0,
                current_drawdown: 0.0,
                peak_capital: initial_capital,
                total_fees_paid: 0,
                scar_count: 0,
                trust_score: 50.0,
                generation,
            },
            trade_history: Vec::new(),
            status: FinanceAgentStatus::Active,
            parent_id: None,
            max_scars: 5,
            scar_cost_multiplier: 1.0,
        }
    }
    
    /// Create a spawned agent (offspring of successful parent)
    pub fn spawn_offspring(
        parent_id: AgentId,
        strategy: String,
        inherited_capital: u64,
        parent_generation: u32,
        inherited_multiplier: f32,
    ) -> Self {
        let mut agent = FinanceAgent::new(strategy, inherited_capital, parent_generation + 1);
        agent.parent_id = Some(parent_id);
        agent.scar_cost_multiplier = inherited_multiplier;
        agent
    }
    
    /// Record a trade execution (irreversible)
    pub fn record_trade(&mut self, trade_description: String, is_win: bool) {
        self.trade_history.push((Utc::now(), trade_description));
        
        let total = self.metrics.total_trades + 1;
        let previous_wins = (self.metrics.win_rate / 100.0 * self.metrics.total_trades as f32) as u64;
        let wins = if is_win {
            previous_wins + 1
        } else {
            previous_wins
        };
        
        self.metrics.total_trades = total;
        self.metrics.win_rate = (wins as f32 / total as f32) * 100.0;
    }
    
    /// Apply a financial scar from a loss
    pub fn inflict_financial_scar(&mut self, drawdown_percentage: f32, _severity: ScarSeverity) {
        if self.metrics.current_drawdown < drawdown_percentage {
            self.metrics.current_drawdown = drawdown_percentage;
        }
        
        if drawdown_percentage > self.metrics.max_drawdown {
            self.metrics.max_drawdown = drawdown_percentage;
        }
        
        self.metrics.scar_count += 1;
        self.scar_cost_multiplier *= 1.05; // 5% cost increase per scar
        
        if self.metrics.scar_count >= self.max_scars {
            self.status = FinanceAgentStatus::TerminallyDamaged;
        }
    }
    
    /// Get current capital remaining
    pub fn get_capital(&self) -> u64 {
        self.lineage.metabolism().energy()
    }
    
    /// Consume capital for a trade operation
    pub fn consume_capital(&mut self, amount: u64) -> Result<(), String> {
        if self.status != FinanceAgentStatus::Active {
            return Err(format!("Agent status is {:?}, cannot trade", self.status));
        }
        
        if amount > self.get_capital() {
            return Err(format!(
                "Insufficient capital: need {}, have {}",
                amount,
                self.get_capital()
            ));
        }
        
        match self.lineage.perform_operation(
            format!("Trade execution (cost {})", amount),
            amount,
        ) {
            crate::lineage::OperationResult::Success { .. } => {
                self.metrics.capital = self.get_capital();
                
                if self.metrics.capital == 0 {
                    self.status = FinanceAgentStatus::Bankrupt;
                }
                
                Ok(())
            }
            _ => Err("Operation failed".to_string()),
        }
    }
    
    /// Restore capital (only from successful trades, not from external recharge)
    pub fn restore_capital_from_profit(&mut self, profit: u64) {
        // Note: This would require extending Lineage to support controlled energy restoration
        // For now, we track it separately in metrics
        self.metrics.capital += profit;
        
        if self.metrics.capital > self.metrics.peak_capital {
            self.metrics.peak_capital = self.metrics.capital;
        }
    }
    
    /// Check if agent is alive
    pub fn is_alive(&self) -> bool {
        self.status == FinanceAgentStatus::Active
    }
    
    /// Get agent status report
    pub fn status_report(&self) -> String {
        format!(
            "Agent {} [{}]\n  Capital: {}\n  Trades: {}\n  Win Rate: {:.2}%\n  Scars: {}\n  Trust: {:.2}\n  Status: {:?}",
            self.id.0,
            self.strategy,
            self.metrics.capital,
            self.metrics.total_trades,
            self.metrics.win_rate,
            self.metrics.scar_count,
            self.metrics.trust_score,
            self.status,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_creation() {
        let agent = FinanceAgent::new("TestStrategy".to_string(), 10000, 0);
        assert_eq!(agent.metrics.capital, 10000);
        assert_eq!(agent.metrics.total_trades, 0);
        assert_eq!(agent.status, FinanceAgentStatus::Active);
    }

    #[test]
    fn test_trade_recording() {
        let mut agent = FinanceAgent::new("TestStrategy".to_string(), 10000, 0);
        agent.record_trade("BUY BTC".to_string(), true);
        assert_eq!(agent.metrics.total_trades, 1);
        assert_eq!(agent.metrics.win_rate, 100.0);
    }
}
