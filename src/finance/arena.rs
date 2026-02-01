//! Multi-Agent Competition Arena - Evolutionary pressure chamber
//!
//! Arena simulates multiple agents competing:
//! - Market conditions affect all agents equally
//! - Agents trade independently but result affects rankings
//! - Successful agents spawn, poor performers die
//! - Creates evolutionary pressure for strategy improvement

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Market state snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketState {
    /// Current time/round number
    pub round: u64,
    
    /// Price of each asset
    pub prices: HashMap<String, f32>,
    
    /// Volatility index
    pub volatility: f32,
    
    /// Market trend (-1.0 = down, 0 = neutral, 1.0 = up)
    pub trend: f32,
    
    /// Black swan event occurred
    pub black_swan: bool,
}

impl MarketState {
    /// Create new market state
    pub fn new(round: u64) -> Self {
        MarketState {
            round,
            prices: HashMap::new(),
            volatility: 0.01,
            trend: 0.0,
            black_swan: false,
        }
    }
    
    /// Update a price
    pub fn set_price(&mut self, asset: String, price: f32) {
        self.prices.insert(asset, price);
    }
    
    /// Generate next market state (random walk)
    pub fn next_state(&self) -> Self {
        let mut next = MarketState::new(self.round + 1);
        let drift = self.trend * 0.0001;
        
        for (asset, price) in &self.prices {
            let change = (rand::random::<f32>() - 0.5) * self.volatility * 2.0 + drift;
            next.set_price(
                asset.clone(),
                (price * (1.0 + change)).max(0.01),
            );
        }
        
        // Volatility tends to mean revert
        next.volatility = self.volatility * 0.95 + rand::random::<f32>() * 0.01;
        
        // Trend random walk
        next.trend = (self.trend + (rand::random::<f32>() - 0.5) * 0.1).clamp(-1.0, 1.0);
        
        // 1% chance of black swan each round
        next.black_swan = rand::random::<f32>() < 0.01;
        
        next
    }
}

/// Competition result for a single agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitionResult {
    /// Agent ID participating
    pub agent_id: String,
    
    /// Final ranking (1 = best)
    pub rank: u32,
    
    /// Total capital at end
    pub final_capital: u64,
    
    /// Return percentage
    pub return_percentage: f32,
    
    /// Trades executed
    pub trades_executed: u64,
    
    /// Win count
    pub wins: u64,
    
    /// Losses
    pub losses: u64,
    
    /// Scars inflicted
    pub scars_inflicted: u32,
    
    /// Ready to spawn?
    pub ready_to_spawn: bool,
}

/// Arena configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArenaConfig {
    /// Number of rounds to simulate
    pub rounds: u64,
    
    /// Initial capital per agent
    pub initial_capital: u64,
    
    /// Assets available to trade
    pub tradeable_assets: Vec<String>,
    
    /// Maximum leverage allowed
    pub max_leverage: f32,
    
    /// Spawning threshold (return percentage)
    pub spawn_threshold: f32,
    
    /// Death threshold (loss percentage)
    pub death_threshold: f32,
}

impl Default for ArenaConfig {
    fn default() -> Self {
        ArenaConfig {
            rounds: 1000,
            initial_capital: 100000,
            tradeable_assets: vec![
                "BTC".to_string(),
                "ETH".to_string(),
                "SOL".to_string(),
                "SPY".to_string(),
            ],
            max_leverage: 10.0,
            spawn_threshold: 25.0,   // 25% return triggers spawn
            death_threshold: -50.0,  // -50% return triggers death
        }
    }
}

/// Arena for multi-agent simulations
pub struct Arena {
    /// Configuration
    pub config: ArenaConfig,
    
    /// Current market state
    pub market_state: MarketState,
    
    /// Competition results
    pub results: Vec<CompetitionResult>,
    
    /// Completed rounds
    pub completed_rounds: u64,
    
    /// Statistics
    pub total_trades_executed: u64,
    pub total_agents_eliminated: u64,
    pub total_spawns: u64,
}

impl Arena {
    /// Create new arena
    pub fn new(config: ArenaConfig) -> Self {
        let mut market = MarketState::new(0);
        
        // Initialize prices
        for asset in &config.tradeable_assets {
            market.set_price(asset.clone(), 100.0);
        }
        
        Arena {
            config,
            market_state: market,
            results: Vec::new(),
            completed_rounds: 0,
            total_trades_executed: 0,
            total_agents_eliminated: 0,
            total_spawns: 0,
        }
    }
    
    /// Run one round of competition
    pub fn tick_round(&mut self) {
        self.market_state = self.market_state.next_state();
        self.completed_rounds += 1;
    }
    
    /// Get current market state
    pub fn get_market_state(&self) -> &MarketState {
        &self.market_state
    }
    
    /// Record agent result
    pub fn record_result(&mut self, result: CompetitionResult) {
        if !result.ready_to_spawn {
            self.total_agents_eliminated += 1;
        } else {
            self.total_spawns += 1;
        }
        
        self.total_trades_executed += result.trades_executed;
        self.results.push(result);
    }
    
    /// Get ranked results
    pub fn get_ranked_results(&self) -> Vec<CompetitionResult> {
        let mut sorted = self.results.clone();
        sorted.sort_by(|a, b| a.rank.cmp(&b.rank));
        sorted
    }
    
    /// Get top performers
    pub fn get_top_performers(&self, count: usize) -> Vec<CompetitionResult> {
        self.get_ranked_results()
            .into_iter()
            .take(count)
            .collect()
    }
    
    /// Get competition statistics
    pub fn get_statistics(&self) -> ArenaStatistics {
        let total_results = self.results.len();
        let avg_return = if total_results > 0 {
            self.results.iter().map(|r| r.return_percentage).sum::<f32>() / total_results as f32
        } else {
            0.0
        };
        
        let max_return = self.results.iter()
            .map(|r| r.return_percentage)
            .fold(f32::NEG_INFINITY, f32::max);
        
        let min_return = self.results.iter()
            .map(|r| r.return_percentage)
            .fold(f32::INFINITY, f32::min);
        
        ArenaStatistics {
            total_agents: total_results as u64,
            total_trades: self.total_trades_executed,
            agents_eliminated: self.total_agents_eliminated,
            spawns_created: self.total_spawns,
            average_return: avg_return,
            max_return,
            min_return,
            rounds_completed: self.completed_rounds,
        }
    }
}

/// Arena statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArenaStatistics {
    pub total_agents: u64,
    pub total_trades: u64,
    pub agents_eliminated: u64,
    pub spawns_created: u64,
    pub average_return: f32,
    pub max_return: f32,
    pub min_return: f32,
    pub rounds_completed: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_state_evolution() {
        let mut state = MarketState::new(0);
        state.set_price("BTC".to_string(), 50000.0);
        
        let next = state.next_state();
        assert_eq!(next.round, 1);
        assert!(next.prices.contains_key("BTC"));
    }

    #[test]
    fn test_arena_creation() {
        let config = ArenaConfig::default();
        let arena = Arena::new(config);
        assert_eq!(arena.completed_rounds, 0);
    }
}
