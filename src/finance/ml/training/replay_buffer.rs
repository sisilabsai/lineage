//! Experience replay buffer for training

use crate::finance::ml::traits::MarketState;
use std::collections::VecDeque;
use serde::{Serialize, Deserialize};
use rand::seq::IteratorRandom;

/// A single training experience
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Experience {
    pub state: MarketState,
    pub action: usize,
    pub reward: f32,
    pub next_state: MarketState,
    pub done: bool,
}

/// Replay buffer for experience memory
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
    
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
    
    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_replay_buffer_push_and_sample() {
        let mut buffer = ReplayBuffer::new(10);
        
        for i in 0..5 {
            buffer.push(Experience {
                state: MarketState {
                    prices: vec![1.0],
                    volatility: vec![0.1],
                    agent_capital: 1.0,
                    scar_count: 0,
                    win_loss_ratio: 0.5,
                    timestamp: i as u64,
                },
                action: i % 3,
                reward: i as f32,
                next_state: MarketState {
                    prices: vec![1.1],
                    volatility: vec![0.1],
                    agent_capital: 1.0,
                    scar_count: 0,
                    win_loss_ratio: 0.5,
                    timestamp: (i + 1) as u64,
                },
                done: false,
            });
        }
        
        assert_eq!(buffer.len(), 5);
        
        let sample = buffer.sample(3);
        assert_eq!(sample.len(), 3);
    }
}
