//! Training loop coordinator
//! Orchestrates ML training with agent lifecycle

use crate::finance::ml::models::q_net::SimpleQNet;
use crate::finance::ml::training::QLearningTrainer;
use crate::finance::ml::training::rewards::RewardCalculator;
use crate::finance::ml::errors::Result;
use crate::finance::agent::{FinanceAgent, AgentMetrics};
use crate::finance::traits::MarketSnapshot;

/// Coordinates training episodes with agents
pub struct TrainingCoordinator {
    /// The Q-Learning trainer
    trainer: QLearningTrainer,
    
    /// Reward calculator
    reward_calc: RewardCalculator,
    
    /// Episode metrics
    current_episode: u32,
    episodes_trained: u32,
    best_episode_reward: f32,
    average_loss: f32,
}

impl TrainingCoordinator {
    /// Create new training coordinator
    pub fn new(model: SimpleQNet) -> Self {
        Self {
            trainer: QLearningTrainer::new(model),
            reward_calc: RewardCalculator::new(),
            current_episode: 0,
            episodes_trained: 0,
            best_episode_reward: f32::NEG_INFINITY,
            average_loss: 0.0,
        }
    }
    
    /// Run a training episode with an agent
    /// An episode = one complete market cycle (e.g., one Arena round)
    pub fn run_training_episode(
        &mut self,
        agent: &mut FinanceAgent,
        market_history: &[MarketSnapshot],
        initial_capital: u64,
    ) -> Result<EpisodeMetrics> {
        let mut episode_reward = 0.0;
        let mut trade_count = 0;
        let mut previous_metrics = AgentMetrics {
            capital: initial_capital,
            total_trades: 0,
            win_rate: 50.0,
            max_drawdown: 0.0,
            current_drawdown: 0.0,
            peak_capital: initial_capital,
            total_fees_paid: 0,
            scar_count: 0,
            trust_score: 50.0,
            generation: 0,
        };
        
        // Simulate trading across market history
        for market in market_history {
            // Agent makes trading decision (this is where neural network predicts)
            // In real usage: agent.decide_trade(market).await
            // Here we simulate the decision
            
            // After trade executes (simulated), calculate reward
            let immediate_reward = self.reward_calc.calculate_immediate_reward(
                &previous_metrics,
                &agent.metrics,  // Updated metrics after trade
            );
            
            episode_reward += immediate_reward;
            trade_count += 1;
            
            // Store experience in replay buffer for training
            // State: normalized market prices + agent metrics
            let state = self.market_to_feature_vector(market, &previous_metrics);
            let action = 0; // Simplified: would come from actual agent decision
            let next_state = self.market_to_feature_vector(market, &agent.metrics);
            
            self.trainer.remember_experience(
                state,
                action,
                immediate_reward,
                next_state,
                false,  // Not terminal yet
            );
            
            previous_metrics = agent.metrics.clone();
        }
        
        // End of episode: calculate terminal reward
        let terminal_reward = self.reward_calc.calculate_episode_reward(
            initial_capital,
            &agent.metrics,
        );
        episode_reward += terminal_reward;
        
        // Remember terminal experience
        let final_state = self.market_to_feature_vector(
            &market_history.last().unwrap(),
            &agent.metrics,
        );
        self.trainer.remember_experience(
            final_state.clone(),
            0,
            terminal_reward,
            final_state,
            true,  // Terminal state
        );
        
        // Train on batch from replay buffer
        let loss = self.trainer.train_step().unwrap_or(0.0);
        self.average_loss = (self.average_loss * 0.9) + (loss * 0.1);  // EMA smoothing
        
        // Update episode tracking
        self.current_episode += 1;
        self.episodes_trained += 1;
        
        if episode_reward > self.best_episode_reward {
            self.best_episode_reward = episode_reward;
        }
        
        Ok(EpisodeMetrics {
            episode_number: self.current_episode,
            reward: episode_reward,
            trades: trade_count,
            final_capital: agent.metrics.capital,
            loss: loss,
            scars_inflicted: agent.metrics.scar_count,
        })
    }
    
    /// Convert market data to feature vector for neural network
    fn market_to_feature_vector(
        &self,
        market: &MarketSnapshot,
        metrics: &AgentMetrics,
    ) -> Vec<f32> {
        let mut features = Vec::new();
        
        // Aggregate market prices
        let avg_price = market.prices.values()
            .map(|p| p.price as f32)
            .sum::<f32>() / market.prices.len().max(1) as f32;
        features.push(avg_price);
        
        // Average volatility
        let avg_volatility = market.prices.values()
            .map(|p| p.volatility as f32)
            .sum::<f32>() / market.prices.len().max(1) as f32;
        features.push(avg_volatility);
        
        // Agent state
        features.push(metrics.capital as f32 / 10000.0);  // Normalize capital
        features.push(metrics.win_rate / 100.0);           // Win rate [0,1]
        features.push(metrics.current_drawdown / 100.0);   // Drawdown [0,1]
        features.push(metrics.scar_count as f32 / 5.0);    // Normalized scars
        features.push(metrics.trust_score / 100.0);        // Trust [0,1]
        
        // Pad to fixed size (for neural network input)
        while features.len() < 10 {
            features.push(0.0);
        }
        features.truncate(10);
        
        features
    }
    
    /// Get training progress
    pub fn get_progress(&self) -> TrainingProgress {
        TrainingProgress {
            episodes_trained: self.episodes_trained,
            current_episode: self.current_episode,
            best_episode_reward: self.best_episode_reward,
            average_loss: self.average_loss,
            buffer_size: self.trainer.replay_buffer.len(),
        }
    }
    
    /// Get underlying trainer for direct access
    pub fn trainer_mut(&mut self) -> &mut QLearningTrainer {
        &mut self.trainer
    }
}

impl Clone for TrainingCoordinator {
    fn clone(&self) -> Self {
        // Note: This is simplified - in production would properly clone neural network state
        Self {
            trainer: self.trainer.clone(),
            reward_calc: RewardCalculator::new(),
            current_episode: self.current_episode,
            episodes_trained: self.episodes_trained,
            best_episode_reward: self.best_episode_reward,
            average_loss: self.average_loss,
        }
    }
}

/// Result of a training episode
#[derive(Debug, Clone)]
pub struct EpisodeMetrics {
    pub episode_number: u32,
    pub reward: f32,
    pub trades: u32,
    pub final_capital: u64,
    pub loss: f32,
    pub scars_inflicted: u32,
}

/// Training progress summary
#[derive(Debug, Clone)]
pub struct TrainingProgress {
    pub episodes_trained: u32,
    pub current_episode: u32,
    pub best_episode_reward: f32,
    pub average_loss: f32,
    pub buffer_size: usize,
}

impl std::fmt::Display for TrainingProgress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Ep: {} | Best Reward: {:.2} | Loss: {:.6} | Buffer: {}",
            self.episodes_trained,
            self.best_episode_reward,
            self.average_loss,
            self.buffer_size
        )
    }
}
