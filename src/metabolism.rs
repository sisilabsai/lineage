//! # Metabolism System
//!
//! ## What This Enforces
//! - Finite energy budget that depletes with operations
//! - Death when energy reaches zero
//! - No recharge or energy restoration mechanisms
//!
//! ## What This Forbids
//! - Energy recharge or restoration
//! - Negative energy values
//! - Energy bypass mechanisms
//! - Resurrection after death
//!
//! ## Violations
//! - Attempting to add energy is an ontological violation
//! - Operating after death is an ontological violation

/// The metabolic state of a lineage.
/// 
/// Energy is finite and depletes with each operation.
/// When energy reaches zero, the lineage dies.
/// 
/// **INVARIANT**: Energy can only decrease, never increase.
/// **CONSEQUENCE**: Death is irreversible.
#[derive(Debug)]
pub struct Metabolism {
    /// Current energy level (cannot increase)
    energy: u64,
    /// Initial energy at creation
    initial_energy: u64,
    /// Whether this metabolism has reached death
    is_dead: bool,
}

/// Result of an energy consumption attempt.
#[derive(Debug, Clone, PartialEq)]
pub enum ConsumptionResult {
    /// Energy was consumed successfully
    Success { remaining: u64 },
    /// Insufficient energy - operation must fail
    Insufficient { requested: u64, available: u64 },
    /// Already dead - no operations possible
    Dead,
}

impl Metabolism {
    /// Creates a new metabolism with finite initial energy.
    /// 
    /// **CONSEQUENCE**: This energy can never be replenished.
    pub fn new(initial_energy: u64) -> Self {
        Metabolism {
            energy: initial_energy,
            initial_energy,
            is_dead: false,
        }
    }

    /// Returns current energy level.
    pub fn energy(&self) -> u64 {
        self.energy
    }

    /// Returns initial energy at creation.
    pub fn initial_energy(&self) -> u64 {
        self.initial_energy
    }

    /// Returns whether this metabolism has died.
    /// 
    /// **INVARIANT**: Once true, this can never become false.
    pub fn is_dead(&self) -> bool {
        self.is_dead
    }

    /// Calculates energy consumption percentage.
    pub fn consumption_rate(&self) -> f64 {
        if self.initial_energy == 0 {
            return 1.0;
        }
        
        let consumed = self.initial_energy - self.energy;
        (consumed as f64) / (self.initial_energy as f64)
    }

    /// Attempts to consume energy for an operation.
    /// 
    /// **CONSEQUENCE**: If successful, energy is permanently lost.
    /// **CONSEQUENCE**: If this causes death, the lineage ends.
    /// 
    /// Returns ConsumptionResult indicating success or reason for failure.
    pub fn consume(&mut self, cost: u64) -> ConsumptionResult {
        if self.is_dead {
            return ConsumptionResult::Dead;
        }

        if cost > self.energy {
            // Insufficient energy - operation fails
            // Do NOT set is_dead here - lineage continues but operation fails
            return ConsumptionResult::Insufficient {
                requested: cost,
                available: self.energy,
            };
        }

        self.energy -= cost;

        if self.energy == 0 {
            self.is_dead = true;
            return ConsumptionResult::Success { remaining: 0 };
        }

        ConsumptionResult::Success {
            remaining: self.energy,
        }
    }

    /// Forces death regardless of remaining energy.
    /// 
    /// This represents a fatal injury or ontological violation.
    /// **CONSEQUENCE**: This is irreversible.
    pub fn die(&mut self) {
        self.energy = 0;
        self.is_dead = true;
    }

    /// Returns the number of operations possible at a given cost.
    /// 
    /// This is purely informational - it does not reserve energy.
    pub fn operations_remaining(&self, operation_cost: u64) -> u64 {
        if self.is_dead || operation_cost == 0 {
            return 0;
        }
        
        self.energy / operation_cost
    }
}

// EXPLICIT PREVENTION: No method to add energy
// EXPLICIT PREVENTION: No method to restore energy
// EXPLICIT PREVENTION: No method to revive after death
// EXPLICIT PREVENTION: No method to reset metabolism

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metabolism_starts_with_initial_energy() {
        let metabolism = Metabolism::new(1000);
        
        assert_eq!(metabolism.energy(), 1000);
        assert_eq!(metabolism.initial_energy(), 1000);
        assert!(!metabolism.is_dead());
    }

    #[test]
    fn energy_consumption_depletes_energy() {
        let mut metabolism = Metabolism::new(1000);
        
        let result = metabolism.consume(300);
        assert_eq!(result, ConsumptionResult::Success { remaining: 700 });
        assert_eq!(metabolism.energy(), 700);
        
        let result = metabolism.consume(400);
        assert_eq!(result, ConsumptionResult::Success { remaining: 300 });
        assert_eq!(metabolism.energy(), 300);
    }

    #[test]
    fn insufficient_energy_fails_operation() {
        let mut metabolism = Metabolism::new(100);
        
        let result = metabolism.consume(150);
        assert_eq!(result, ConsumptionResult::Insufficient {
            requested: 150,
            available: 100,
        });
        
        // Energy should not change on failed consumption
        assert_eq!(metabolism.energy(), 100);
        assert!(!metabolism.is_dead());
    }

    #[test]
    fn depleting_all_energy_causes_death() {
        let mut metabolism = Metabolism::new(100);
        
        metabolism.consume(50);
        assert!(!metabolism.is_dead());
        
        let result = metabolism.consume(50);
        assert_eq!(result, ConsumptionResult::Success { remaining: 0 });
        assert!(metabolism.is_dead());
        assert_eq!(metabolism.energy(), 0);
    }

    #[test]
    fn operations_after_death_are_impossible() {
        let mut metabolism = Metabolism::new(100);
        
        metabolism.die();
        
        let result = metabolism.consume(10);
        assert_eq!(result, ConsumptionResult::Dead);
        assert_eq!(metabolism.energy(), 0);
    }

    #[test]
    fn death_is_irreversible() {
        let mut metabolism = Metabolism::new(100);
        
        metabolism.die();
        assert!(metabolism.is_dead());
        
        // There should be NO way to revive:
        // metabolism.revive();
        // metabolism.restore_energy(100);
        // metabolism.reset();
        
        // If such methods exist, the metabolism system has been violated.
        
        assert!(metabolism.is_dead());
        assert_eq!(metabolism.energy(), 0);
    }

    #[test]
    fn energy_can_only_decrease() {
        let mut metabolism = Metabolism::new(1000);
        
        metabolism.consume(200);
        let energy_after_first = metabolism.energy();
        
        metabolism.consume(300);
        let energy_after_second = metabolism.energy();
        
        assert!(energy_after_second < energy_after_first);
        assert!(energy_after_second < 1000);
        
        // There should be NO method like:
        // metabolism.add_energy(100);
        // metabolism.recharge(500);
        
        // If such methods exist, the metabolism system has been violated.
    }

    #[test]
    fn consumption_rate_tracks_depletion() {
        let mut metabolism = Metabolism::new(1000);
        
        assert_eq!(metabolism.consumption_rate(), 0.0);
        
        metabolism.consume(250);
        assert_eq!(metabolism.consumption_rate(), 0.25);
        
        metabolism.consume(500);
        assert_eq!(metabolism.consumption_rate(), 0.75);
        
        metabolism.consume(250);
        assert_eq!(metabolism.consumption_rate(), 1.0);
        assert!(metabolism.is_dead());
    }

    #[test]
    fn energy_never_increases() {
        let mut metabolism = Metabolism::new(1000);
        
        let energy0 = metabolism.energy();
        metabolism.consume(100);
        let energy1 = metabolism.energy();
        metabolism.consume(200);
        let energy2 = metabolism.energy();
        metabolism.consume(50);
        let energy3 = metabolism.energy();
        
        // INVARIANT: Energy is monotonically decreasing
        assert!(energy1 <= energy0);
        assert!(energy2 <= energy1);
        assert!(energy3 <= energy2);
        
        // INVARIANT: Energy can only decrease, never increase
        assert_eq!(energy1, 900);
        assert_eq!(energy2, 700);
        assert_eq!(energy3, 650);
    }

    #[test]
    fn no_negative_cost_operations() {
        let mut metabolism = Metabolism::new(1000);
        
        // INVARIANT: All operations must have non-negative cost
        // consume() only accepts u64, which cannot be negative
        
        metabolism.consume(0); // Zero cost is allowed
        assert_eq!(metabolism.energy(), 1000);
        
        // INVARIANT: There should be NO method like:
        // metabolism.consume(-100)  // Would increase energy
        // metabolism.add(100)
        // 
        // If such methods exist, energy monotonicity has been violated.
    }

    #[test]
    fn death_state_is_permanent() {
        let mut metabolism = Metabolism::new(100);
        
        metabolism.die();
        
        // Check death multiple times
        assert!(metabolism.is_dead());
        assert!(metabolism.is_dead());
        assert!(metabolism.is_dead());
        
        // INVARIANT: Once dead, always dead
        // INVARIANT: There should be NO method like:
        // metabolism.revive()
        // metabolism.resurrect()
        // metabolism.set_alive(true)
        // 
        // If such methods exist, death finality has been violated.
        
        assert!(metabolism.is_dead());
        assert_eq!(metabolism.energy(), 0);
    }

    #[test]
    fn death_at_zero_energy_is_mandatory() {
        let mut metabolism = Metabolism::new(50);
        
        metabolism.consume(50);
        
        // INVARIANT: Zero energy MUST mean death
        assert_eq!(metabolism.energy(), 0);
        assert!(metabolism.is_dead());
        
        // INVARIANT: There should be NO way to have zero energy but be alive
        // If alive status can be independent of energy, the metabolism system has been violated.
    }
}
