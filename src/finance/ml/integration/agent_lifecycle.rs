//! ML agent lifecycle integration

use crate::finance::ml::models::q_net::SimpleQNet;
use crate::finance::ml::traits::MlStrategy;
use crate::finance::ml::errors::MlError;

/// Apply learning penalty for losses (scar damage)
pub fn apply_scar_damage(model: &mut SimpleQNet, scar_count: u32) -> Result<(), MlError> {
    match scar_count {
        0 => {},  // No damage
        1..=2 => {
            // Light damage: increase exploration
            model.exploration_rate = (model.exploration_rate + 0.05).min(0.9);
        }
        3..=5 => {
            // Moderate damage: increase exploration + small mutation
            model.exploration_rate = (model.exploration_rate + 0.1).min(0.9);
            model.mutate(0.1, 0.01)?;
        }
        _ => {
            // Severe damage: heavy mutation
            model.mutate(0.5, 0.05)?;
            model.exploration_rate = 0.5;
        }
    }
    Ok(())
}

/// Check if agent should spawn offspring
pub fn should_spawn_offspring(roi: f32, survival_threshold: f32) -> bool {
    roi > survival_threshold
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_spawn_threshold() {
        assert!(should_spawn_offspring(10.0, 5.0));
        assert!(!should_spawn_offspring(3.0, 5.0));
        assert!(should_spawn_offspring(5.01, 5.0));
    }
}
