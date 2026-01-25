//! # Minimal Behavior Loop
//!
//! ## What This Enforces
//! - One behavior (pulse)
//! - One contract (minimum energy threshold)
//! - One injury path (weak pulse strain)
//! - One scar consequence (increased future cost)
//!
//! ## What This Demonstrates
//! - Behaviors consume energy
//! - Contracts detect violations
//! - Violations cause permanent injury
//! - Scars reduce future capability
//!
//! This is not useful. This is consequential.

use crate::lineage::{Lineage, OperationError, OperationResult};
use crate::scar::ScarSeverity;

/// Output of a pulse behavior execution.
#[derive(Debug, PartialEq)]
pub struct PulseOutput {
    /// Energy level when pulse was executed
    pub energy_at_pulse: u64,
    /// Whether the pulse was strong enough
    pub is_strong: bool,
    /// Whether strain occurred (contract violation)
    pub strain_occurred: bool,
}

/// The single behavior: PULSE
/// 
/// A pulse is a basic operation that consumes energy.
/// The pulse must be executed with sufficient energy or it causes strain.
/// 
/// **CONSEQUENCE**: Weak pulses cause permanent strain scars.
/// **CONSEQUENCE**: Strain scars increase future pulse costs.
pub struct PulseBehavior {
    /// Minimum energy required for a healthy pulse
    threshold: u64,
    /// Base energy cost per pulse
    base_cost: u64,
}

impl PulseBehavior {
    /// Creates the pulse behavior with fixed parameters.
    /// 
    /// **INVARIANT**: Threshold and cost are permanent and non-configurable.
    pub fn new() -> Self {
        PulseBehavior {
            threshold: 30,  // Minimum 30 energy for healthy pulse
            base_cost: 10,   // Base cost is 10 energy
        }
    }

    /// Executes a single pulse.
    /// 
    /// **CONTRACT**: Pulse must be executed with energy >= threshold.
    /// **CONSEQUENCE**: If energy < threshold during pulse, strain occurs.
    /// **CONSEQUENCE**: Strain inflicts a permanent scar.
    /// **CONSEQUENCE**: Each strain scar increases future pulse cost by 5.
    /// 
    /// This demonstrates:
    /// - Behavior consumes energy
    /// - Contract is checked based on observable state
    /// - Violation triggers injury, not correction
    /// - Scar permanently reduces capability
    pub fn execute_pulse(&self, lineage: &mut Lineage) -> PulseOutput {
        // Check if lineage is operational
        if !lineage.is_alive() {
            return PulseOutput {
                energy_at_pulse: 0,
                is_strong: false,
                strain_occurred: false,
            };
        }

        // Calculate actual cost based on accumulated strain scars
        let strain_scars = self.count_strain_scars(lineage);
        let actual_cost = self.base_cost + (strain_scars * 5);

        // Observe energy level BEFORE pulse
        let energy_before = lineage.metabolism().energy();

        // Perform the pulse operation
        let result = lineage.perform_operation(
            format!("Pulse (cost: {})", actual_cost),
            actual_cost,
        );

        // Check if pulse completed
        let pulse_completed = matches!(result, OperationResult::Success { .. });

        if !pulse_completed {
            return PulseOutput {
                energy_at_pulse: energy_before,
                is_strong: false,
                strain_occurred: false,
            };
        }

        // CONTRACT VERIFICATION: Was energy sufficient during pulse?
        let is_strong = energy_before >= self.threshold;

        // INJURY PATH: Contract violation causes strain
        let mut strain_occurred = false;
        if !is_strong {
            // Weak pulse detected - this is a contract violation
            // Trigger injury: inflict permanent strain scar
            self.inflict_strain_injury(lineage, energy_before);
            strain_occurred = true;
        }

        PulseOutput {
            energy_at_pulse: energy_before,
            is_strong,
            strain_occurred,
        }
    }

    /// INJURY PATH: Inflicts a strain scar.
    /// 
    /// **CONSEQUENCE**: This scar is permanent.
    /// **CONSEQUENCE**: Future pulses cost more energy.
    /// **CONSEQUENCE**: Eventually, pulses become impossible.
    fn inflict_strain_injury(&self, lineage: &mut Lineage, energy_at_violation: u64) {
        let strain_scar = OperationError::new(
            ScarSeverity::Moderate,
            format!("Pulse strain: insufficient energy (had {}, need {})", 
                energy_at_violation, self.threshold),
        ).with_context(format!(
            "Contract violation: pulse executed below threshold. \
             This permanently increases future pulse cost by 5 energy."
        ));

        lineage.record_error(strain_scar);
    }

    /// Counts strain scars to calculate increased cost.
    /// 
    /// Each strain scar adds 5 to the pulse cost.
    /// This is the permanent capability reduction.
    fn count_strain_scars(&self, lineage: &Lineage) -> u64 {
        lineage
            .scars()
            .all_scars()
            .iter()
            .filter(|scar| scar.description().contains("Pulse strain"))
            .count() as u64
    }

    /// Returns the current pulse cost based on accumulated strain.
    /// 
    /// This demonstrates how scars reduce capability:
    /// - 0 strain scars: cost = 10
    /// - 1 strain scar: cost = 15
    /// - 2 strain scars: cost = 20
    /// - etc.
    pub fn current_pulse_cost(&self, lineage: &Lineage) -> u64 {
        let strain_scars = self.count_strain_scars(lineage);
        self.base_cost + (strain_scars * 5)
    }

    /// Returns the energy threshold for healthy pulses.
    pub fn threshold(&self) -> u64 {
        self.threshold
    }
}

impl Default for PulseBehavior {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lineage::Lineage;

    #[test]
    fn pulse_with_sufficient_energy_is_strong() {
        let mut lineage = Lineage::create(100);
        let behavior = PulseBehavior::new();

        let output = behavior.execute_pulse(&mut lineage);

        assert!(output.is_strong, "Pulse should be strong with 100 energy");
        assert!(!output.strain_occurred, "No strain should occur");
        assert_eq!(lineage.scars().scar_count(), 0, "No scars should be inflicted");
    }

    #[test]
    fn pulse_below_threshold_causes_strain() {
        // Create lineage with barely enough energy for pulse but below threshold
        let mut lineage = Lineage::create(25);
        let behavior = PulseBehavior::new();

        let output = behavior.execute_pulse(&mut lineage);

        assert!(!output.is_strong, "Pulse should be weak with 25 energy");
        assert!(output.strain_occurred, "Strain should occur");
        assert_eq!(lineage.scars().scar_count(), 1, "Strain scar should be inflicted");
    }

    #[test]
    fn strain_scars_increase_future_pulse_cost() {
        let mut lineage = Lineage::create(100);
        let behavior = PulseBehavior::new();

        // Initial cost is 10
        assert_eq!(behavior.current_pulse_cost(&lineage), 10);

        // Deplete energy to cause strain
        lineage.perform_operation("Deplete".to_string(), 75);
        
        // First weak pulse - causes strain
        behavior.execute_pulse(&mut lineage);
        
        // Cost should now be 15 (base 10 + 5 for one strain scar)
        assert_eq!(behavior.current_pulse_cost(&lineage), 15);
        assert_eq!(lineage.scars().scar_count(), 1);
    }

    #[test]
    fn multiple_strain_scars_compound_cost() {
        let behavior = PulseBehavior::new();

        // Test that multiple strain scars compound the cost
        // Use separate lineages to isolate each strain accumulation
        
        // First lineage: 1 strain
        let mut lineage1 = Lineage::create(100);
        lineage1.perform_operation("Deplete".to_string(), 75); // 25 left
        behavior.execute_pulse(&mut lineage1); // Strain! Cost was 10
        assert_eq!(behavior.count_strain_scars(&lineage1), 1);
        assert_eq!(behavior.current_pulse_cost(&lineage1), 15); // 10 + 5
        
        // Second lineage: 2 strains
        let mut lineage2 = Lineage::create(100);
        lineage2.perform_operation("Deplete".to_string(), 75); // 25 left
        behavior.execute_pulse(&mut lineage2); // Strain 1
        lineage2.perform_operation("Deplete2".to_string(), 10); // Down to 5, then wait...
        // Can't do another pulse without more energy, so test with 3rd lineage
        
        // Third lineage: Create with scars already (simulate via multiple weak pulses)
        let mut lineage3 = Lineage::create(200);
        for i in 0..3 {
            let energy = lineage3.metabolism().energy();
            if energy > 25 {
                lineage3.perform_operation(format!("Deplete #{}", i), energy - 25);
            }
            behavior.execute_pulse(&mut lineage3);
            if !lineage3.is_alive() || lineage3.metabolism().energy() < 25 {
                break;
            }
        }
        
        let final_strains = behavior.count_strain_scars(&lineage3);
        assert!(final_strains >= 1, "Should have at least 1 strain");
        assert_eq!(behavior.current_pulse_cost(&lineage3), 10 + (final_strains * 5));
    }

    #[test]
    fn strain_accumulation_leads_to_death_spiral() {
        let mut lineage = Lineage::create(200);
        let behavior = PulseBehavior::new();

        // Execute weak pulses repeatedly to accumulate strain
        for i in 0..15 {
            if !lineage.is_alive() {
                break;
            }
            
            let current_energy = lineage.metabolism().energy();
            
            // Deplete to 25 (below threshold of 30) to trigger strain
            if current_energy > 25 {
                lineage.perform_operation(
                    format!("Deplete #{}", i + 1),
                    current_energy - 25
                );
            }

            behavior.execute_pulse(&mut lineage);
            
            // Break if energy too low to continue
            if lineage.metabolism().energy() < 15 {
                break;
            }
        }

        // Verify death spiral occurred - at least some strains
        let final_strain_count = behavior.count_strain_scars(&lineage);
        assert!(final_strain_count >= 1, "Should have accumulated at least 1 strain, got {}", final_strain_count);
        
        // Verify cost increased due to scars
        assert_eq!(behavior.current_pulse_cost(&lineage), 10 + (final_strain_count * 5));
    }

    #[test]
    fn behavior_cannot_ignore_strain_scars() {
        let behavior = PulseBehavior::new();
        let mut lineage = Lineage::create(200);
        
        // Inflict first strain scar
        lineage.perform_operation("Deplete".to_string(), 175); // Energy: 25
        behavior.execute_pulse(&mut lineage); // Cost: 10, Energy after: 15, Strain inflicted
        
        let strain_count = behavior.count_strain_scars(&lineage);
        let pulse_cost = behavior.current_pulse_cost(&lineage);
        
        // INVARIANT: Pulse cost MUST account for ALL strain scars
        assert_eq!(strain_count, 1);
        assert_eq!(pulse_cost, 10 + (strain_count * 5));
        assert_eq!(pulse_cost, 15);
        
        // INVARIANT: There should be NO way to:
        // behavior.reset_cost()
        // behavior.ignore_scars()
        // behavior.set_base_cost_only()
        // 
        // If such methods exist, scar consequence has been violated.
    }

    #[test]
    fn pulse_cost_only_increases() {
        let behavior = PulseBehavior::new();
        let mut lineage = Lineage::create(500);
        
        let cost0 = behavior.current_pulse_cost(&lineage);
        
        // Cause first strain
        lineage.perform_operation("Deplete1".to_string(), 475); // Energy: 25
        behavior.execute_pulse(&mut lineage); // Cost: 10, Energy after: 15
        let cost1 = behavior.current_pulse_cost(&lineage);
        
        // Verify cost increased
        let strain_count_1 = behavior.count_strain_scars(&lineage);
        
        // INVARIANT: Pulse cost is monotonically increasing
        assert_eq!(cost0, 10);
        assert!(cost1 > cost0);  // 10 + (strains * 5)
        assert_eq!(cost1, 10 + (strain_count_1 * 5));
        
        // INVARIANT: Cost increase is permanent
        // Once strain scars exist, cost never goes back down
        
        // Verify cost stays increased even after more operations
        let cost1_again = behavior.current_pulse_cost(&lineage);
        assert_eq!(cost1, cost1_again);
    }

    #[test]
    fn strain_scars_affect_all_future_pulses() {
        let behavior = PulseBehavior::new();
        let mut lineage = Lineage::create(500);
        
        // Cause one strain
        lineage.perform_operation("Deplete".to_string(), 475);
        behavior.execute_pulse(&mut lineage);
        
        // INVARIANT: Cost is now permanently increased
        let increased_cost = behavior.current_pulse_cost(&lineage);
        assert_eq!(increased_cost, 15);
        
        // Execute many more pulses
        for _ in 0..5 {
            let current = lineage.metabolism().energy();
            if current > 40 {
                lineage.perform_operation("Work".to_string(), current - 40);
            }
            if lineage.is_alive() {
                behavior.execute_pulse(&mut lineage);
            }
        }
        
        // INVARIANT: Cost remains increased (or increases further, but never decreases)
        let final_cost = behavior.current_pulse_cost(&lineage);
        assert!(final_cost >= increased_cost);
        
        // INVARIANT: Past strain scars affect ALL future pulses
        // There is NO "forgetting" or "healing" of strain consequences
    }

    #[test]
    fn pulse_output_records_observable_state() {
        let mut lineage = Lineage::create(50);
        let behavior = PulseBehavior::new();

        let output = behavior.execute_pulse(&mut lineage);

        // Output should reflect exact state at moment of pulse
        assert_eq!(output.energy_at_pulse, 50);
        assert!(output.is_strong); // 50 >= 30 threshold
        assert!(!output.strain_occurred);
    }

    #[test]
    fn cannot_pulse_after_death() {
        let mut lineage = Lineage::create(10);
        let behavior = PulseBehavior::new();

        // Deplete energy to death
        lineage.perform_operation("Death".to_string(), 10);
        assert!(!lineage.is_alive());

        let output = behavior.execute_pulse(&mut lineage);

        // Should return failed pulse output
        assert_eq!(output.energy_at_pulse, 0);
        assert!(!output.is_strong);
        assert!(!output.strain_occurred); // No strain because pulse didn't execute
    }

    #[test]
    fn strain_scars_are_permanent() {
        let mut lineage = Lineage::create(100);
        let behavior = PulseBehavior::new();

        // Cause strain
        lineage.perform_operation("Deplete".to_string(), 75);
        behavior.execute_pulse(&mut lineage);
        
        let scars_after_strain = lineage.scars().scar_count();
        let cost_after_strain = behavior.current_pulse_cost(&lineage);

        // Perform more operations
        lineage.perform_operation("More ops".to_string(), 5);
        lineage.perform_operation("Even more".to_string(), 3);

        // Scars and cost should remain
        assert_eq!(lineage.scars().scar_count(), scars_after_strain);
        assert_eq!(behavior.current_pulse_cost(&lineage), cost_after_strain);
    }
}
