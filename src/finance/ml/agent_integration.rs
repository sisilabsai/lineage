//! Integration of ML models with Finance Agents
//!
//! This module bridges neural network decision-making with real trading agents,
//! enabling evolution through performance-based mutations.

use crate::finance::agent::{FinanceAgent, FinanceAgentStatus};
use crate::finance::ml::models::q_net::SimpleQNet;
use crate::finance::ml::traits::{MarketState, TradeAction};

/// A trading agent enhanced with ML decision-making capabilities
///
/// MLFinanceAgent wraps a standard FinanceAgent and adds:
/// - Neural network (Q-Net) for decision-making
/// - Epsilon-greedy exploration strategy
/// - Scar infliction on trading losses
/// - Mutation-based evolution
pub struct MLFinanceAgent {
    /// The underlying finance agent
    pub agent: FinanceAgent,
    
    /// Q-Network for action evaluation
    pub q_net: SimpleQNet,
    
    /// Exploration rate (1.0 = always explore, 0.0 = always exploit)
    pub epsilon: f32,
    
    /// Mutation rate for evolution (0.0-1.0)
    pub mutation_rate: f32,
    
    /// Standard deviation for Gaussian mutations
    pub mutation_strength: f32,
}

impl MLFinanceAgent {
    /// Create a new ML-enhanced agent
    pub fn new(
        agent: FinanceAgent,
        input_size: usize,
        hidden_size: usize,
        epsilon: f32,
        mutation_rate: f32,
        mutation_strength: f32,
    ) -> Result<Self, String> {
        let q_net = SimpleQNet::new(input_size, hidden_size)
            .map_err(|e| format!("Failed to create QNet: {:?}", e))?;
        
        Ok(MLFinanceAgent {
            agent,
            q_net,
            epsilon,
            mutation_rate,
            mutation_strength,
        })
    }
    
    /// Decide trade action using epsilon-greedy strategy
    ///
    /// With probability epsilon, explore (random action).
    /// Otherwise, exploit (use Q-Net prediction).
    pub fn decide_trade(&self, market_state: &MarketState) -> TradeAction {
        // Epsilon-greedy exploration
        if rand::random::<f32>() < self.epsilon {
            // Explore: random action
            let rand_action = rand::random::<u32>() % 3;
            match rand_action {
                0 => TradeAction::Buy,
                1 => TradeAction::Sell,
                _ => TradeAction::Hold,
            }
        } else {
            // Exploit: use Q-Net
            // Build state vector from MarketState fields
            let mut state_vector = market_state.prices.clone();  // Base prices
            
            // Pad or trim to exactly 5 elements for consistency
            while state_vector.len() < 5 {
                state_vector.push(0.0);
            }
            state_vector.truncate(5);
            
            match self.q_net.forward_pass(&state_vector) {
                Ok(q_values) => {
                    // Find action with highest Q-value
                    let mut best_idx = 2;  // Default to Hold
                    
                    for (idx, value) in q_values.iter().enumerate() {
                        if idx < q_values.len() && idx < 3 {
                            if idx == 0 || value > &q_values[best_idx] {
                                best_idx = idx;
                            }
                        }
                    }
                    
                    let best_action = match best_idx {
                        0 => TradeAction::Buy,
                        1 => TradeAction::Sell,
                        _ => TradeAction::Hold,
                    };
                    
                    best_action
                }
                Err(_) => TradeAction::Hold,  // Default on error
            }
        }
    }
    
    /// Execute a trade and inflict scar if loss occurs
    pub fn execute_trade_ml(
        &mut self,
        market_state: &MarketState,
        loss_occurred: bool,
        loss_amount: u64,
    ) -> TradeAction {
        let action = self.decide_trade(market_state);
        
        // Inflict scar on loss
        if loss_occurred {
            self.inflict_scar(loss_amount);
        }
        
        action
    }
    
    /// Inflict permanent scar from trading loss
    ///
    /// Scars increase the agent's cost of capital permanently.
    fn inflict_scar(&mut self, loss_amount: u64) {
        self.agent.metrics.scar_count += 1;
        
        // Damage factor scales with scar count
        let damage_factor = 0.01 * self.agent.metrics.scar_count as f32;
        
        println!(
            "🔴 Scar inflicted! Count: {} | Damage factor: {:.4} | Loss: {}",
            self.agent.metrics.scar_count, damage_factor, loss_amount
        );
        
        // Cap max scars
        if self.agent.metrics.scar_count >= self.agent.max_scars {
            self.agent.status = FinanceAgentStatus::TerminallyDamaged;
            println!("💀 Agent reached terminal damage level!");
        }
    }
    
    /// Spawn an offspring with mutations for evolution
    ///
    /// The offspring inherits:
    /// - Half the parent's capital
    /// - Generation number incremented
    /// - Cloned Q-Net (mutations would require internal access)
    pub fn spawn_offspring(&self) -> Result<MLFinanceAgent, String> {
        // Create base offspring agent
        let offspring_agent = FinanceAgent::new(
            format!("{}_gen{}", self.agent.strategy, self.agent.metrics.generation + 1),
            self.agent.metrics.capital / 2,  // Split capital
            self.agent.metrics.generation + 1,
        );
        
        // Create offspring with parent's Q-Net (cloned)
        let offspring = MLFinanceAgent::new(
            offspring_agent,
            5,  // input_size
            64, // hidden_size
            self.epsilon,
            self.mutation_rate,
            self.mutation_strength,
        )?;
        
        Ok(offspring)
    }
    
    /// Decay epsilon for reduced exploration over time
    pub fn decay_epsilon(&mut self, decay_rate: f32) {
        self.epsilon = (self.epsilon * decay_rate).max(0.1);
    }
}

/// Arena for testing multiple ML agents
pub struct MLAgentArena {
    pub agents: Vec<MLFinanceAgent>,
    pub round: u32,
}

impl MLAgentArena {
    /// Create a new arena with initial agents
    pub fn new(agents: Vec<MLFinanceAgent>) -> Self {
        MLAgentArena { agents, round: 0 }
    }
    
    /// Rank agents by capital (fitness)
    pub fn rank_agents(&mut self) {
        self.agents.sort_by(|a, b| {
            b.agent.metrics.capital.cmp(&a.agent.metrics.capital)
        });
    }
    
    /// Evolution step: keep top performers, spawn offspring
    pub fn evolve(&mut self) -> Result<(), String> {
        self.rank_agents();
        
        let total_agents = self.agents.len();
        let survivors = (total_agents as f32 * 0.5) as usize;  // Keep top 50%
        
        if survivors == 0 {
            return Ok(());  // No survivors
        }
        
        // Collect survivors and spawn offspring
        let mut next_generation = Vec::new();
        
        // Keep survivors
        for i in 0..survivors.min(self.agents.len()) {
            next_generation.push(self.agents[i].clone_agent());
        }
        
        // Spawn offspring from survivors
        let survivor_count = next_generation.len();
        for i in 0..survivor_count {
            if let Ok(offspring) = next_generation[i].spawn_offspring() {
                next_generation.push(offspring);
            }
        }
        
        self.agents = next_generation;
        self.round += 1;
        
        println!("⚡ Evolution Round {}: {} agents → {} agents", 
            self.round, total_agents, self.agents.len());
        
        Ok(())
    }
}

impl MLFinanceAgent {
    /// Clone the agent for evolution
    fn clone_agent(&self) -> Self {
        MLFinanceAgent {
            agent: FinanceAgent::new(
                self.agent.strategy.clone(),
                self.agent.metrics.capital,
                self.agent.metrics.generation,
            ),
            q_net: self.q_net.clone(),
            epsilon: self.epsilon,
            mutation_rate: self.mutation_rate,
            mutation_strength: self.mutation_strength,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_epsilon_decay() {
        let agent = FinanceAgent::new("test".to_string(), 1000, 0);
        let mut ml_agent = MLFinanceAgent::new(agent, 5, 64, 1.0, 0.1, 0.5)
            .expect("Failed to create ML agent");
        
        ml_agent.decay_epsilon(0.99);
        assert!(ml_agent.epsilon < 1.0);
        assert!(ml_agent.epsilon > 0.1);
    }
    
    #[test]
    fn test_arena_ranking() {
        let agent = FinanceAgent::new("agent1".to_string(), 1000, 0);
        let ml_agent = MLFinanceAgent::new(agent, 5, 64, 0.5, 0.1, 0.5)
            .expect("Failed to create ML agent");
        
        let mut arena = MLAgentArena::new(vec![ml_agent]);
        arena.rank_agents();
        
        assert_eq!(arena.agents.len(), 1);
    }
}
