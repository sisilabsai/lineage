//! Spawning and Inheritance System - Evolutionary propagation
//!
//! Successful agents spawn offspring that inherit optimized traits:
//! - Requires minimum capital (immutable cost)
//! - Offspring inherit parent's cost multiplier
//! - Strategy parameters can be inherited with mutations
//! - Creates agent dynasties and lineages

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::finance::agent::AgentId;

/// Strategy parameters that can be inherited
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OffspringTraits {
    /// Inherited cost multiplier from parent's scars
    pub inherited_cost_multiplier: f32,
    
    /// Risk tolerance (0-1, affects leverage)
    pub risk_tolerance: f32,
    
    /// Aggressiveness of strategy (0-1)
    pub aggressiveness: f32,
    
    /// Diversification ratio (0-1)
    pub diversification: f32,
    
    /// Base win rate estimate from parent
    pub estimated_win_rate: f32,
    
    /// Mutation factor applied to parameters
    pub mutation_rate: f32,
}

impl OffspringTraits {
    /// Create offspring traits from parent metrics
    pub fn inherit_from_parent(
        parent_cost_multiplier: f32,
        parent_win_rate: f32,
        mutation_rate: f32,
    ) -> Self {
        // Mutation affects inherited traits slightly
        let mutation = (rand::random::<f32>() - 0.5) * mutation_rate;
        
        OffspringTraits {
            inherited_cost_multiplier: parent_cost_multiplier,
            risk_tolerance: (parent_win_rate / 100.0 * 0.8 + mutation.abs()).clamp(0.0, 1.0),
            aggressiveness: (parent_win_rate / 100.0 * 0.6 + mutation.abs()).clamp(0.0, 1.0),
            diversification: 0.5 + mutation * 0.1,
            estimated_win_rate: parent_win_rate,
            mutation_rate,
        }
    }
    
    /// Apply mutation to create variation
    pub fn mutate(&mut self) {
        let mutation = (rand::random::<f32>() - 0.5) * self.mutation_rate;
        self.risk_tolerance = (self.risk_tolerance + mutation).clamp(0.0, 1.0);
        self.aggressiveness = (self.aggressiveness + mutation).clamp(0.0, 1.0);
    }
}

/// Strategy for inheriting traits to offspring
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InheritanceStrategy {
    /// Exact copy of parent traits
    Identical,
    
    /// Inherit with small mutations (5% variance)
    LowMutation,
    
    /// Inherit with medium mutations (15% variance)
    MediumMutation,
    
    /// Inherit with high mutations (30% variance)
    HighMutation,
    
    /// Random traits within successful range
    Exploration,
}

impl InheritanceStrategy {
    /// Get mutation rate for this strategy
    pub fn mutation_rate(&self) -> f32 {
        match self {
            InheritanceStrategy::Identical => 0.0,
            InheritanceStrategy::LowMutation => 0.05,
            InheritanceStrategy::MediumMutation => 0.15,
            InheritanceStrategy::HighMutation => 0.30,
            InheritanceStrategy::Exploration => 0.50,
        }
    }
}

/// Spawning requirement validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawningRequirement {
    /// Minimum capital required
    pub min_capital: u64,
    
    /// Capital cost percentage (is lost in spawn)
    pub cost_percentage: f32,
    
    /// Minimum win rate required
    pub min_win_rate: f32,
    
    /// Minimum number of trades
    pub min_trades: u64,
    
    /// Maximum scars allowed
    pub max_scars: u32,
}

impl Default for SpawningRequirement {
    fn default() -> Self {
        SpawningRequirement {
            min_capital: 10000,
            cost_percentage: 25.0,
            min_win_rate: 55.0,
            min_trades: 10,
            max_scars: 2,
        }
    }
}

/// Result of spawn operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpawnResult {
    /// Offspring successfully created
    Created {
        offspring_id: AgentId,
        inherited_capital: u64,
        traits: OffspringTraits,
    },
    
    /// Spawning rejected - not eligible
    Rejected { reason: String },
    
    /// Spawning not yet possible
    NotReady { reason: String },
}

/// Offspring record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Offspring {
    /// Unique offspring ID (assigned at birth)
    pub id: AgentId,
    
    /// Parent's ID
    pub parent_id: AgentId,
    
    /// When offspring was spawned
    pub spawn_time: DateTime<Utc>,
    
    /// Generation (parent_gen + 1)
    pub generation: u32,
    
    /// Inherited traits
    pub traits: OffspringTraits,
    
    /// Capital given to offspring
    pub initial_capital: u64,
    
    /// Strategy name
    pub strategy_name: String,
    
    /// Inheritance strategy used
    pub inheritance_strategy: InheritanceStrategy,
}

impl Offspring {
    /// Validate if parent can spawn
    pub fn validate_spawn(
        parent_capital: u64,
        parent_win_rate: f32,
        parent_trades: u64,
        parent_scars: u32,
        requirement: &SpawningRequirement,
    ) -> Result<u64, String> {
        if parent_capital < requirement.min_capital {
            return Err(format!(
                "Insufficient capital: {} < {}",
                parent_capital, requirement.min_capital
            ));
        }
        
        if parent_win_rate < requirement.min_win_rate {
            return Err(format!(
                "Win rate too low: {:.2}% < {:.2}%",
                parent_win_rate, requirement.min_win_rate
            ));
        }
        
        if parent_trades < requirement.min_trades {
            return Err(format!(
                "Insufficient trade history: {} < {}",
                parent_trades, requirement.min_trades
            ));
        }
        
        if parent_scars > requirement.max_scars {
            return Err(format!(
                "Too many scars: {} > {}",
                parent_scars, requirement.max_scars
            ));
        }
        
        let spawn_cost = (parent_capital as f32 * (requirement.cost_percentage / 100.0)) as u64;
        Ok(spawn_cost)
    }
    
    /// Create new offspring
    pub fn create_offspring(
        id: AgentId,
        parent_id: AgentId,
        generation: u32,
        traits: OffspringTraits,
        initial_capital: u64,
        strategy_name: String,
        inheritance_strategy: InheritanceStrategy,
    ) -> Self {
        Offspring {
            id,
            parent_id,
            spawn_time: Utc::now(),
            generation,
            traits,
            initial_capital,
            strategy_name,
            inheritance_strategy,
        }
    }
}

/// Lineage tree tracking family history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageTree {
    /// Mapping of agent IDs to parent IDs
    pub parent_map: std::collections::HashMap<String, String>,
    
    /// Offspring records
    pub offspring_records: Vec<Offspring>,
    
    /// Statistics about evolutionary success
    pub total_spawns: u64,
    pub successful_lineages: u64,
}

impl LineageTree {
    pub fn new() -> Self {
        LineageTree {
            parent_map: std::collections::HashMap::new(),
            offspring_records: Vec::new(),
            total_spawns: 0,
            successful_lineages: 0,
        }
    }
    
    /// Record an offspring creation
    pub fn record_spawn(&mut self, parent_id: AgentId, offspring: Offspring) {
        self.parent_map.insert(
            offspring.id.to_string(),
            parent_id.to_string(),
        );
        self.offspring_records.push(offspring);
        self.total_spawns += 1;
    }
    
    /// Get lineage chain (ancestors)
    pub fn get_lineage_chain(&self, agent_id: AgentId) -> Vec<String> {
        let mut chain = vec![agent_id.to_string()];
        let mut current = agent_id.to_string();
        
        while let Some(parent) = self.parent_map.get(&current) {
            chain.push(parent.clone());
            current = parent.clone();
        }
        
        chain
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offspring_traits_inheritance() {
        let traits = OffspringTraits::inherit_from_parent(1.05, 60.0, 0.1);
        assert!(traits.risk_tolerance >= 0.0);
        assert!(traits.risk_tolerance <= 1.0);
    }

    #[test]
    fn test_spawn_validation() {
        let req = SpawningRequirement::default();
        let result = Offspring::validate_spawn(20000, 60.0, 20, 1, &req);
        assert!(result.is_ok());
    }
}
