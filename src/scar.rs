//! # Scar System
//!
//! ## What This Enforces
//! - Permanent recording of all errors and failures
//! - Scars remain visible forever
//! - Accumulation of scars over time
//!
//! ## What This Forbids
//! - Scar removal or healing
//! - Hiding or obscuring scars
//! - Scar reset or clearing
//!
//! ## Violations
//! - Attempting to remove scars is an ontological violation

use chrono::{DateTime, Utc};

/// Severity level of a scar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScarSeverity {
    /// Minor issue - functionality preserved
    Minor,
    /// Moderate issue - some capability lost
    Moderate,
    /// Severe issue - significant capability lost
    Severe,
    /// Fatal - should trigger death
    Fatal,
}

/// A permanent scar from an error or failure.
/// 
/// Once created, a scar exists forever and cannot be removed or healed.
/// Scars accumulate over the lifetime of a lineage.
/// 
/// **INVARIANT**: Scars are immutable after creation.
#[derive(Debug)]
pub struct Scar {
    /// When this scar was inflicted
    timestamp: DateTime<Utc>,
    /// Severity of the injury
    severity: ScarSeverity,
    /// Description of what went wrong
    description: String,
    /// Context or stack trace
    context: Option<String>,
}

impl Scar {
    /// Creates a new scar.
    /// 
    /// **CONSEQUENCE**: This scar is now permanent.
    pub fn new(severity: ScarSeverity, description: String, context: Option<String>) -> Self {
        Scar {
            timestamp: Utc::now(),
            severity,
            description,
            context,
        }
    }

    /// Returns when this scar was inflicted.
    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    /// Returns the severity of this scar.
    pub fn severity(&self) -> ScarSeverity {
        self.severity
    }

    /// Returns the description of this scar.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the context of this scar, if any.
    pub fn context(&self) -> Option<&str> {
        self.context.as_deref()
    }
}

/// Permanent scar tissue accumulated over a lineage's lifetime.
/// 
/// This structure maintains all scars that have ever been inflicted.
/// Scars cannot be removed, hidden, or healed.
/// 
/// **INVARIANT**: Scar count can only increase, never decrease.
#[derive(Debug)]
pub struct ScarTissue {
    /// All scars, in chronological order
    scars: Vec<Scar>,
}

impl ScarTissue {
    /// Creates new, unmarked scar tissue.
    pub fn new() -> Self {
        ScarTissue { scars: Vec::new() }
    }

    /// Inflicts a new scar.
    /// 
    /// **CONSEQUENCE**: This scar is permanent and visible forever.
    pub fn inflict(&mut self, severity: ScarSeverity, description: String, context: Option<String>) {
        let scar = Scar::new(severity, description, context);
        self.scars.push(scar);
    }

    /// Returns all scars in chronological order.
    pub fn all_scars(&self) -> &[Scar] {
        &self.scars
    }

    /// Returns the total number of scars.
    pub fn scar_count(&self) -> usize {
        self.scars.len()
    }

    /// Returns scars of a specific severity.
    pub fn scars_by_severity(&self, severity: ScarSeverity) -> Vec<&Scar> {
        self.scars
            .iter()
            .filter(|scar| scar.severity() == severity)
            .collect()
    }

    /// Returns the most recent scar, if any.
    pub fn latest_scar(&self) -> Option<&Scar> {
        self.scars.last()
    }

    /// Checks if this tissue has any fatal scars.
    /// 
    /// Fatal scars indicate the lineage should be dead.
    pub fn has_fatal_scars(&self) -> bool {
        self.scars
            .iter()
            .any(|scar| scar.severity() == ScarSeverity::Fatal)
    }

    /// Calculates a "damage score" based on accumulated scars.
    /// 
    /// This is informational and does not affect behavior.
    pub fn damage_score(&self) -> u32 {
        self.scars
            .iter()
            .map(|scar| match scar.severity() {
                ScarSeverity::Minor => 1,
                ScarSeverity::Moderate => 5,
                ScarSeverity::Severe => 20,
                ScarSeverity::Fatal => 100,
            })
            .sum()
    }
}

// EXPLICIT PREVENTION: No method to remove scars
// EXPLICIT PREVENTION: No method to heal scars
// EXPLICIT PREVENTION: No method to clear scar tissue
// EXPLICIT PREVENTION: No method to hide scars

impl Default for ScarTissue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scar_tissue_starts_clean() {
        let tissue = ScarTissue::new();
        
        assert_eq!(tissue.scar_count(), 0);
        assert_eq!(tissue.damage_score(), 0);
        assert!(!tissue.has_fatal_scars());
    }

    #[test]
    fn inflicting_scars_increases_count() {
        let mut tissue = ScarTissue::new();
        
        tissue.inflict(ScarSeverity::Minor, "Minor error".to_string(), None);
        assert_eq!(tissue.scar_count(), 1);
        
        tissue.inflict(ScarSeverity::Moderate, "Moderate error".to_string(), None);
        assert_eq!(tissue.scar_count(), 2);
        
        tissue.inflict(ScarSeverity::Severe, "Severe error".to_string(), Some("Context".to_string()));
        assert_eq!(tissue.scar_count(), 3);
    }

    #[test]
    fn scars_are_permanent() {
        let mut tissue = ScarTissue::new();
        
        tissue.inflict(ScarSeverity::Minor, "Error 1".to_string(), None);
        tissue.inflict(ScarSeverity::Minor, "Error 2".to_string(), None);
        
        assert_eq!(tissue.scar_count(), 2);
        
        // There should be NO method like:
        // tissue.remove_scar(0);
        // tissue.heal_scar(1);
        // tissue.clear_scars();
        
        // If such methods exist, the scar system has been violated.
        
        assert_eq!(tissue.scar_count(), 2);
    }

    #[test]
    fn scar_count_only_increases() {
        let mut tissue = ScarTissue::new();
        
        let count0 = tissue.scar_count();
        tissue.inflict(ScarSeverity::Minor, "Scar 1".to_string(), None);
        let count1 = tissue.scar_count();
        tissue.inflict(ScarSeverity::Moderate, "Scar 2".to_string(), None);
        let count2 = tissue.scar_count();
        
        // INVARIANT: Scar count is monotonically increasing
        assert_eq!(count0, 0);
        assert_eq!(count1, 1);
        assert_eq!(count2, 2);
        assert!(count1 > count0);
        assert!(count2 > count1);
        
        // INVARIANT: There should be NO method like:
        // tissue.remove_scar(0)
        // tissue.reduce_scar_count()
        // 
        // If such methods exist, scar permanence has been violated.
    }

    #[test]
    fn damage_score_only_increases() {
        let mut tissue = ScarTissue::new();
        
        let score0 = tissue.damage_score();
        tissue.inflict(ScarSeverity::Minor, "Damage 1".to_string(), None);
        let score1 = tissue.damage_score();
        tissue.inflict(ScarSeverity::Moderate, "Damage 2".to_string(), None);
        let score2 = tissue.damage_score();
        tissue.inflict(ScarSeverity::Severe, "Damage 3".to_string(), None);
        let score3 = tissue.damage_score();
        
        // INVARIANT: Damage score is monotonically increasing
        assert_eq!(score0, 0);
        assert!(score1 > score0);  // Minor = 1
        assert!(score2 > score1);  // + Moderate = 5
        assert!(score3 > score2);  // + Severe = 20
        
        // INVARIANT: There should be NO method like:
        // tissue.heal_damage(5)
        // tissue.reduce_severity()
        // 
        // If such methods exist, scar permanence has been violated.
    }

    #[test]
    fn scar_descriptions_are_immutable() {
        let mut tissue = ScarTissue::new();
        
        tissue.inflict(ScarSeverity::Minor, "Original error".to_string(), None);
        
        let scars = tissue.all_scars();
        let desc1 = scars[0].description();
        let desc2 = scars[0].description();
        
        // INVARIANT: Scar descriptions never change
        assert_eq!(desc1, desc2);
        assert_eq!(desc1, "Original error");
        
        // INVARIANT: There should be NO method like:
        // scar.set_description("Modified")
        // tissue.update_scar(0, "New description")
        // 
        // If such methods exist, scar immutability has been violated.
    }

    #[test]
    fn fatal_scar_status_is_permanent() {
        let mut tissue = ScarTissue::new();
        
        assert!(!tissue.has_fatal_scars());
        
        tissue.inflict(ScarSeverity::Fatal, "Fatal error".to_string(), None);
        
        // Check multiple times
        assert!(tissue.has_fatal_scars());
        assert!(tissue.has_fatal_scars());
        assert!(tissue.has_fatal_scars());
        
        // INVARIANT: Once fatal scars exist, they always exist
        // INVARIANT: There should be NO method like:
        // tissue.remove_fatal_scars()
        // tissue.downgrade_severity()
        // 
        // If such methods exist, scar permanence has been violated.
    }

    #[test]
    fn scars_maintain_chronological_order() {
        let mut tissue = ScarTissue::new();
        
        tissue.inflict(ScarSeverity::Minor, "First".to_string(), None);
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        tissue.inflict(ScarSeverity::Moderate, "Second".to_string(), None);
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        tissue.inflict(ScarSeverity::Severe, "Third".to_string(), None);
        
        let scars = tissue.all_scars();
        assert_eq!(scars[0].description(), "First");
        assert_eq!(scars[1].description(), "Second");
        assert_eq!(scars[2].description(), "Third");
        
        assert!(scars[0].timestamp() < scars[1].timestamp());
        assert!(scars[1].timestamp() < scars[2].timestamp());
    }

    #[test]
    fn fatal_scars_are_detected() {
        let mut tissue = ScarTissue::new();
        
        assert!(!tissue.has_fatal_scars());
        
        tissue.inflict(ScarSeverity::Minor, "Minor".to_string(), None);
        assert!(!tissue.has_fatal_scars());
        
        tissue.inflict(ScarSeverity::Fatal, "Fatal error".to_string(), None);
        assert!(tissue.has_fatal_scars());
    }

    #[test]
    fn damage_score_accumulates() {
        let mut tissue = ScarTissue::new();
        
        assert_eq!(tissue.damage_score(), 0);
        
        tissue.inflict(ScarSeverity::Minor, "".to_string(), None); // +1
        assert_eq!(tissue.damage_score(), 1);
        
        tissue.inflict(ScarSeverity::Moderate, "".to_string(), None); // +5
        assert_eq!(tissue.damage_score(), 6);
        
        tissue.inflict(ScarSeverity::Severe, "".to_string(), None); // +20
        assert_eq!(tissue.damage_score(), 26);
        
        tissue.inflict(ScarSeverity::Fatal, "".to_string(), None); // +100
        assert_eq!(tissue.damage_score(), 126);
    }

    #[test]
    fn scars_by_severity_filters_correctly() {
        let mut tissue = ScarTissue::new();
        
        tissue.inflict(ScarSeverity::Minor, "Minor 1".to_string(), None);
        tissue.inflict(ScarSeverity::Severe, "Severe 1".to_string(), None);
        tissue.inflict(ScarSeverity::Minor, "Minor 2".to_string(), None);
        tissue.inflict(ScarSeverity::Moderate, "Moderate 1".to_string(), None);
        
        let minor_scars = tissue.scars_by_severity(ScarSeverity::Minor);
        assert_eq!(minor_scars.len(), 2);
        
        let severe_scars = tissue.scars_by_severity(ScarSeverity::Severe);
        assert_eq!(severe_scars.len(), 1);
        
        let fatal_scars = tissue.scars_by_severity(ScarSeverity::Fatal);
        assert_eq!(fatal_scars.len(), 0);
    }
}
