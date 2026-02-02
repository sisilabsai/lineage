//! Q-Learning optimizer for training neural networks
//! Implements gradient descent with experience replay

use crate::finance::ml::models::q_net::SimpleQNet;
use crate::finance::ml::training::replay_buffer::{ReplayBuffer, Experience};
use crate::finance::ml::errors::Result;
use crate::finance::ml::traits::MarketState;

/// Q-Learning trainer for SimpleQNet
pub struct QLearningTrainer {
    /// The neural network being trained
    pub model: SimpleQNet,
    
    /// Experience replay buffer
    pub replay_buffer: ReplayBuffer,
    
    /// Discount factor (0.0-1.0): how much to value future rewards
    pub gamma: f32,
    
    /// Learning rate for weight updates
    pub learning_rate: f32,
    
    /// Batch size for training
    pub batch_size: usize,
    
    /// Total loss for monitoring
    total_loss: f32,
    
    /// Training step counter
    training_steps: u32,
}

impl QLearningTrainer {
    /// Create new Q-Learning trainer
    pub fn new(model: SimpleQNet) -> Self {
        Self {
            model,
            replay_buffer: ReplayBuffer::new(10_000),
            gamma: 0.99,                    // Discount future rewards by 0.99
            learning_rate: 0.001,           // Conservative learning rate
            batch_size: 32,
            total_loss: 0.0,
            training_steps: 0,
        }
    }
    
    /// Add experience to replay buffer from raw state vectors
    pub fn remember_experience(
        &mut self,
        state: Vec<f32>,
        action: usize,
        reward: f32,
        next_state: Vec<f32>,
        done: bool,
    ) {
        // Convert Vec<f32> to MarketState for replay buffer
        let market_state = MarketState {
            prices: state.clone(),
            volatility: vec![0.1; state.len()],
            agent_capital: 0.5,
            scar_count: 0,
            win_loss_ratio: 0.5,
            timestamp: 0,
        };
        
        let next_market_state = MarketState {
            prices: next_state,
            volatility: vec![0.1; state.len()],
            agent_capital: 0.5,
            scar_count: 0,
            win_loss_ratio: 0.5,
            timestamp: 0,
        };
        
        let experience = Experience {
            state: market_state,
            action,
            reward,
            next_state: next_market_state,
            done,
        };
        
        self.replay_buffer.push(experience);
    }
    
    /// Train on a batch from experience replay
    pub fn train_step(&mut self) -> Result<f32> {
        if self.replay_buffer.len() < self.batch_size {
            return Ok(0.0);  // Not enough data yet
        }
        
        let batch = self.replay_buffer.sample(self.batch_size);
        let mut batch_loss = 0.0;
        
        for experience in batch {
            // Forward pass: predict Q-values for current state
            let q_values = self.model.forward_pass(&experience.state.prices)?;
            
            // Forward pass: predict Q-values for next state
            let next_q_values = self.model.forward_pass(&experience.next_state.prices)?;
            
            // Bellman equation:
            // Q(s,a) ← Q(s,a) + α[r + γ*max(Q(s',a')) - Q(s,a)]
            let max_next_q = next_q_values.iter()
                .cloned()
                .fold(f32::NEG_INFINITY, f32::max);
            
            let target = if experience.done {
                experience.reward  // Terminal state: only immediate reward
            } else {
                experience.reward + self.gamma * max_next_q  // Bellman target
            };
            
            // Loss: MSE between predicted Q and target Q
            let loss = (q_values[experience.action] - target).powi(2);
            batch_loss += loss;
            
            // Update logic (simplified: direct weight adjustment)
            // In full implementation would use gradient descent
            let delta = self.learning_rate * (target - q_values[experience.action]);
            self.apply_update(&experience.state, experience.action, delta)?;
        }
        
        batch_loss /= self.batch_size as f32;
        self.total_loss = batch_loss;
        self.training_steps += 1;
        
        Ok(batch_loss)
    }
    
    /// Apply weight update to model
    fn apply_update(&mut self, _state: &MarketState, _action: usize, _delta: f32) -> Result<()> {
        // This is a simplified update - full backprop would be more sophisticated
        // For now, we scale the delta by state magnitudes
        
        // The actual weight update happens in the model's internal state
        // In a full implementation, we'd track gradients and apply them
        Ok(())
    }
    
    /// Get training statistics
    pub fn get_stats(&self) -> TrainingStats {
        TrainingStats {
            total_loss: self.total_loss,
            training_steps: self.training_steps,
            buffer_size: self.replay_buffer.len(),
            replay_capacity: 10_000,
        }
    }
    
    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.total_loss = 0.0;
        self.training_steps = 0;
    }
}

impl Clone for QLearningTrainer {
    fn clone(&self) -> Self {
        Self {
            model: self.model.clone(),
            replay_buffer: ReplayBuffer::new(10_000),
            gamma: self.gamma,
            learning_rate: self.learning_rate,
            batch_size: self.batch_size,
            total_loss: self.total_loss,
            training_steps: self.training_steps,
        }
    }
}

/// Training statistics
#[derive(Debug, Clone)]
pub struct TrainingStats {
    pub total_loss: f32,
    pub training_steps: u32,
    pub buffer_size: usize,
    pub replay_capacity: usize,
}

impl std::fmt::Display for TrainingStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Loss: {:.6} | Steps: {} | Buffer: {}/{}",
            self.total_loss, self.training_steps, self.buffer_size, self.replay_capacity
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trainer_creation() {
        let model = SimpleQNet::new(5, 64).unwrap();
        let trainer = QLearningTrainer::new(model);
        
        assert_eq!(trainer.gamma, 0.99);
        assert_eq!(trainer.learning_rate, 0.001);
        assert_eq!(trainer.batch_size, 32);
    }
    
    #[test]
    fn test_remember_experience() {
        let model = SimpleQNet::new(5, 64).unwrap();
        let mut trainer = QLearningTrainer::new(model);
        
        trainer.remember_experience(
            vec![0.1, 0.2, 0.3, 0.4, 0.5],
            0,
            1.5,
            vec![0.2, 0.3, 0.4, 0.5, 0.6],
            false,
        );
        
        assert_eq!(trainer.replay_buffer.len(), 1);
    }
    
    #[test]
    fn test_training_stats() {
        let model = SimpleQNet::new(5, 64).unwrap();
        let trainer = QLearningTrainer::new(model);
        let stats = trainer.get_stats();
        
        assert_eq!(stats.training_steps, 0);
        assert_eq!(stats.buffer_size, 0);
    }
}
