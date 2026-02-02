//! Adapter that bridges ML models to the existing TradingStrategy trait
//! This is the key integration point - ML models implement MlStrategy,
//! this adapter implements TradingStrategy for use with FinanceAgent

use crate::finance::traits::{TradingStrategy, MarketSnapshot};
use crate::finance::ml::traits::{MlStrategy, MarketState, TradeAction};
use crate::finance::ml::errors::MlError;
use async_trait::async_trait;
use std::result::Result as StdResult;

/// Result type for ML operations
pub type Result<T> = StdResult<T, MlError>;

/// Adapts any MlStrategy to work as a TradingStrategy
/// This allows ML models to be used in the existing finance agent system
pub struct MlStrategyAdapter {
    /// The underlying ML model
    model: Box<dyn MlStrategy>,
    
    /// Strategy name
    name: String,
    
    /// Exploration rate (for epsilon-greedy in Q-learning)
    exploration_rate: f32,
}

impl MlStrategyAdapter {
    /// Create a new ML strategy adapter
    pub fn new(model: Box<dyn MlStrategy>, name: String) -> Self {
        Self {
            model,
            name,
            exploration_rate: 0.1,
        }
    }
    
    /// Update exploration rate (for learning dynamics)
    pub fn set_exploration_rate(&mut self, rate: f32) {
        self.exploration_rate = rate.clamp(0.0, 1.0);
    }
    
    /// Increase exploration (called when scars damage the model)
    pub fn increase_exploration(&mut self, amount: f32) {
        self.exploration_rate = (self.exploration_rate + amount).clamp(0.0, 1.0);
    }
    
    /// Convert MarketSnapshot to MarketState for neural network input
    fn snapshot_to_ml_state(market: &MarketSnapshot) -> Result<MarketState> {
        // Extract prices and normalize them
        let mut prices = Vec::new();
        let mut volatilities = Vec::new();
        
        for price_point in market.prices.values() {
            prices.push(price_point.price as f32);
            volatilities.push(price_point.volatility as f32);
        }
        
        // Normalize prices to log scale for neural network stability
        let normalized_prices: Vec<f32> = prices.iter()
            .map(|p| if *p > 0.0 { p.ln() as f32 } else { 0.0 })
            .collect();
        
        // Standardize (zero mean, unit variance)
        let mean = normalized_prices.iter().sum::<f32>() / normalized_prices.len().max(1) as f32;
        let variance = normalized_prices.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f32>() / normalized_prices.len().max(1) as f32;
        let std = variance.sqrt().max(0.01); // Avoid division by zero
        
        let standardized: Vec<f32> = normalized_prices.iter()
            .map(|x| (x - mean) / std)
            .collect();
        
        Ok(MarketState {
            prices: standardized,
            volatility: volatilities,
            agent_capital: 0.5, // Default: neutral capital position
            scar_count: 0,      // Default: no scars
            win_loss_ratio: 0.5, // Default: break-even
            timestamp: market.timestamp as u64,
        })
    }
    
    /// Convert ML trade action to TradeDecision
    fn ml_action_to_trade_decision(
        action: TradeAction,
        confidence: f32,
        market: &MarketSnapshot,
        _available_capital: u64,
    ) -> crate::finance::traits::TradeDecision {
        // Select primary symbol (highest price point)
        let symbol = market.prices
            .iter()
            .max_by(|a, b| a.1.price.partial_cmp(&b.1.price).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(k, _)| k.clone())
            .unwrap_or_else(|| "BTC".to_string());
        
        match action {
            TradeAction::Hold => crate::finance::traits::TradeDecision {
                should_trade: false,
                symbol: String::new(),
                allocation_percentage: 0.0,
                direction: "none".to_string(),
            },
            TradeAction::Buy => crate::finance::traits::TradeDecision {
                should_trade: true,
                symbol,
                allocation_percentage: (confidence * 100.0).clamp(1.0, 100.0),
                direction: "buy".to_string(),
            },
            TradeAction::Sell => crate::finance::traits::TradeDecision {
                should_trade: true,
                symbol,
                allocation_percentage: (confidence * 100.0).clamp(1.0, 100.0),
                direction: "sell".to_string(),
            },
        }
    }
}

/// Implement TradingStrategy for the adapter
/// This makes ML models work seamlessly with FinanceAgent
#[async_trait]
impl TradingStrategy for MlStrategyAdapter {
    /// Get trade decision from ML model
    async fn decide_trade(&self, market: &MarketSnapshot) -> crate::finance::traits::TradeDecision {
        // Convert market snapshot to ML state
        match Self::snapshot_to_ml_state(market) {
            Ok(ml_state) => {
                // Get prediction from neural network
                match self.model.predict(&ml_state).await {
                    Ok(ml_decision) => {
                        // Convert to TradeDecision
                        Self::ml_action_to_trade_decision(
                            ml_decision.action,
                            ml_decision.confidence,
                            market,
                            0,
                        )
                    },
                    Err(_) => {
                        // On error, default to hold
                        crate::finance::traits::TradeDecision {
                            should_trade: false,
                            symbol: String::new(),
                            allocation_percentage: 0.0,
                            direction: "hold".to_string(),
                        }
                    }
                }
            },
            Err(_) => {
                // On state conversion error, hold
                crate::finance::traits::TradeDecision {
                    should_trade: false,
                    symbol: String::new(),
                    allocation_percentage: 0.0,
                    direction: "hold".to_string(),
                }
            }
        }
    }
    
    /// Called when a loss occurs - pass damage to ML model
    fn on_loss(&mut self, drawdown: f32, _loss_amount: u64) {
        // Increase exploration rate based on drawdown severity
        if drawdown > 10.0 {
            self.increase_exploration(0.05);
        }
        if drawdown > 20.0 {
            self.increase_exploration(0.10);
        }
    }
    
    /// Called when a win occurs - could decay exploration rate
    fn on_win(&mut self, _gain_amount: u64) {
        // Gradually shift toward exploitation by reducing exploration
        self.exploration_rate = (self.exploration_rate * 0.95).max(0.01);
    }
    
    /// Strategy name
    fn name(&self) -> &str {
        &self.name
    }
}

/// Create an ML strategy adapter from a Q-Net model
#[cfg(feature = "ml")]
pub fn create_q_net_strategy(
    input_size: usize,
    hidden_size: usize,
) -> Result<MlStrategyAdapter> {
    use crate::finance::ml::models::q_net::SimpleQNet;
    
    let model = SimpleQNet::new(input_size, hidden_size)?;
    Ok(MlStrategyAdapter::new(
        Box::new(model),
        "SimpleQNet".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_exploration_rate() {
        let model = Box::new(crate::finance::ml::models::SimpleQNet::new(5, 64).unwrap());
        let mut adapter = MlStrategyAdapter::new(model, "test".to_string());
        
        assert_eq!(adapter.exploration_rate, 0.1);
        adapter.increase_exploration(0.05);
        assert_eq!(adapter.exploration_rate, 0.15);
        adapter.set_exploration_rate(2.0); // Should clamp
        assert_eq!(adapter.exploration_rate, 1.0);
    }
}
