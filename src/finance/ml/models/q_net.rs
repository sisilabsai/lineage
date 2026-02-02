//! Simple Q-Network for reinforcement learning trading
//! Pure Rust implementation using ndarray for tensor operations

use crate::finance::ml::traits::{MlStrategy, MarketState, TradeAction, TradeDecision, ModelMetadata};
use crate::finance::ml::errors::MlError;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

#[cfg(feature = "ml")]
use ndarray_rand::RandomExt;
#[cfg(feature = "ml")]
use rand_distr::Normal;

/// Simple Q-Network for discrete action spaces (Buy, Sell, Hold)
/// Pure Rust implementation with ndarray tensors
#[derive(Clone, Serialize, Deserialize)]
pub struct SimpleQNet {
    /// Hidden layer weights (serialized)
    #[serde(skip)]
    hidden_weights: Vec<Vec<f32>>,
    
    /// Output layer weights (serialized)
    #[serde(skip)]
    output_weights: Vec<Vec<f32>>,
    
    /// Bias terms
    #[serde(skip)]
    hidden_bias: Vec<f32>,
    output_bias: Vec<f32>,
    
    /// Exploration rate (epsilon-greedy)
    pub exploration_rate: f32,
    
    /// Model metadata
    metadata: ModelMetadata,
    
    /// Model configuration
    config: QNetConfig,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct QNetConfig {
    pub input_size: usize,
    pub hidden_size: usize,
    pub output_size: usize,  // 3 actions: Buy, Sell, Hold
}

impl SimpleQNet {
    /// Create new Q-network with random initialization
    pub fn new(input_size: usize, hidden_size: usize) -> Result<Self, MlError> {
        let output_size = 3;  // Buy, Sell, Hold
        
        #[cfg(feature = "ml")]
        {
            use ndarray::Array2;
            
            let dist = Normal::new(0.0, 1.0 / (input_size as f32).sqrt())
                .map_err(|e| MlError::InitializationError(e.to_string()))?;
            
            let hidden_w = Array2::random((input_size, hidden_size), dist);
            let output_w = Array2::random((hidden_size, output_size), dist);
            
            Ok(Self {
                hidden_weights: hidden_w.outer_iter().map(|row| row.to_vec()).collect(),
                output_weights: output_w.outer_iter().map(|row| row.to_vec()).collect(),
                hidden_bias: vec![0.0; hidden_size],
                output_bias: vec![0.0; output_size],
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
        
        #[cfg(not(feature = "ml"))]
        {
            Ok(Self {
                hidden_weights: vec![vec![0.0; hidden_size]; input_size],
                output_weights: vec![vec![0.0; output_size]; hidden_size],
                hidden_bias: vec![0.0; hidden_size],
                output_bias: vec![0.0; output_size],
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
    }
    
    /// Forward pass: state → Q-values
    pub fn forward(&self, state: &[f32]) -> Result<Vec<f32>, MlError> {
        if state.len() != self.config.input_size {
            return Err(MlError::InvalidState(
                format!("Expected {} inputs, got {}", self.config.input_size, state.len())
            ));
        }
        
        #[cfg(feature = "ml")]
        {
            use ndarray::Array1;
            
            let _input = Array1::from_vec(state.to_vec());
            
            // Hidden layer
            let mut hidden: Array1<f32> = Array1::zeros(self.config.hidden_size);
            for (i, w_row) in self.hidden_weights.iter().enumerate() {
                for (j, &w) in w_row.iter().enumerate() {
                    hidden[j] += state[i] * w;
                }
            }
            for i in 0..self.config.hidden_size {
                hidden[i] += self.hidden_bias[i];
                hidden[i] = hidden[i].max(0.0);  // ReLU activation
            }
            
            // Output layer
            let mut output = Array1::zeros(self.config.output_size);
            for (i, w_row) in self.output_weights.iter().enumerate() {
                for (j, &w) in w_row.iter().enumerate() {
                    output[j] += hidden[i] * w;
                }
            }
            for i in 0..self.config.output_size {
                output[i] += self.output_bias[i];
            }
            
            Ok(output.to_vec())
        }
        
        #[cfg(not(feature = "ml"))]
        {
            Ok(vec![0.0; self.config.output_size])
        }
    }
    
    /// Forward pass alias for training (computes Q-values for state)
    pub fn forward_pass(&self, state: &[f32]) -> Result<Vec<f32>, MlError> {
        self.forward(state)
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
        let data = (self.hidden_weights.clone(), self.output_weights.clone());
        serde_json::to_vec(&data)
            .map_err(|e| MlError::SerializationError(e.to_string()))
    }
    
    fn deserialize(&mut self, data: &[u8]) -> Result<(), MlError> {
        let (hw, ow): (Vec<Vec<f32>>, Vec<Vec<f32>>) = serde_json::from_slice(data)
            .map_err(|e| MlError::SerializationError(e.to_string()))?;
        self.hidden_weights = hw;
        self.output_weights = ow;
        Ok(())
    }
    
    fn metadata(&self) -> ModelMetadata {
        self.metadata.clone()
    }
    
    fn mutate(&mut self, mutation_rate: f32, _mutation_strength: f32) -> Result<(), MlError> {
        // Apply random mutations to hidden weights
        for row in self.hidden_weights.iter_mut() {
            for weight in row.iter_mut() {
                if rand::random::<f32>() < mutation_rate {
                    *weight += (rand::random::<f32>() - 0.5) * 0.1;
                }
            }
        }
        
        // Apply random mutations to output weights
        for row in self.output_weights.iter_mut() {
            for weight in row.iter_mut() {
                if rand::random::<f32>() < mutation_rate {
                    *weight += (rand::random::<f32>() - 0.5) * 0.1;
                }
            }
        }
        
        self.metadata.mutation_count += 1;
        Ok(())
    }
    fn clone_box(&self) -> Box<dyn MlStrategy> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_q_net_creation() {
        let model = SimpleQNet::new(5, 64).expect("Failed to create model");
        assert_eq!(model.metadata().name, "SimpleQNet");
        assert_eq!(model.exploration_rate, 0.1);
    }
    
    #[tokio::test]
    async fn test_prediction() {
        let model = SimpleQNet::new(5, 64).expect("Failed to create model");
        let state = MarketState {
            prices: vec![1.0, 1.1],
            volatility: vec![0.1, 0.05],
            agent_capital: 0.5,
            scar_count: 0,
            win_loss_ratio: 0.6,
            timestamp: 1000,
        };
        
        let decision = model.predict(&state).await.expect("Prediction failed");
        assert!(decision.confidence >= 0.0 && decision.confidence <= 1.0);
    }
}
