//! Mutation and evolution mechanics for spawning

use crate::finance::ml::models::q_net::SimpleQNet;
use crate::finance::ml::errors::MlError;
use crate::finance::ml::traits::MlStrategy;

/// Configuration for offspring spawning
pub struct SpawningConfig {
    pub mutation_rate: f32,          // Probability of weight mutation
    pub mutation_strength: f32,      // Magnitude of noise
    pub survival_threshold: f32,     // ROI threshold for spawning
    pub max_offspring: usize,        // Max offspring per parent
}

impl Default for SpawningConfig {
    fn default() -> Self {
        Self {
            mutation_rate: 0.15,
            mutation_strength: 0.02,
            survival_threshold: 5.0,
            max_offspring: 2,
        }
    }
}

impl SpawningConfig {
    /// Create offspring from parent with mutations
    pub fn spawn_offspring(&self, parent: &SimpleQNet) -> Result<SimpleQNet, MlError> {
        let mut offspring = parent.clone();
        offspring.mutate(self.mutation_rate, self.mutation_strength)?;
        Ok(offspring)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_spawning_config_default() {
        let config = SpawningConfig::default();
        assert_eq!(config.mutation_rate, 0.15);
        assert_eq!(config.survival_threshold, 5.0);
    }
}
