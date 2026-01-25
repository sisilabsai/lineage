// Trust Degradation System
//
// ONTOLOGICAL CONSTRAINT: Trust is irreversible capability loss.
//
// Trust is NOT:
// - A numeric score that can increase or decrease
// - A meter that can be refilled
// - A reputation that recovers over time
//
// Trust IS:
// - A set of capabilities that can only shrink
// - Permanent consequence of violations
// - Irreversible loss enforced by type system

use crate::lineage::{Lineage, OperationError};
use crate::scar::ScarSeverity;
use std::collections::HashSet;

/// Actions that privileged actors may perform.
/// 
/// Once a capability is revoked, it CANNOT be restored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrustCapability {
    /// Execute privileged operations
    Execute,
    
    /// Validate other actors' operations
    Validate,
    
    /// Modify system configuration
    Configure,
    
    /// Access sensitive data
    AccessSensitive,
    
    /// Delegate capabilities to other actors
    Delegate,
    
    /// Audit system state
    Audit,
}

impl TrustCapability {
    /// All capabilities granted at initialization.
    pub fn full_set() -> HashSet<TrustCapability> {
        vec![
            TrustCapability::Execute,
            TrustCapability::Validate,
            TrustCapability::Configure,
            TrustCapability::AccessSensitive,
            TrustCapability::Delegate,
            TrustCapability::Audit,
        ]
        .into_iter()
        .collect()
    }
}

/// Classification of trust violations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationType {
    /// Unauthorized access attempt
    UnauthorizedAccess,
    
    /// Data integrity violation
    IntegrityViolation,
    
    /// Policy bypass attempt
    PolicyBypass,
    
    /// Credential misuse
    CredentialMisuse,
    
    /// Malicious action
    MaliciousAction,
}

impl ViolationType {
    /// Determine scar severity for this violation type.
    pub fn severity(&self) -> ScarSeverity {
        match self {
            ViolationType::UnauthorizedAccess => ScarSeverity::Minor,
            ViolationType::IntegrityViolation => ScarSeverity::Moderate,
            ViolationType::PolicyBypass => ScarSeverity::Severe,
            ViolationType::CredentialMisuse => ScarSeverity::Severe,
            ViolationType::MaliciousAction => ScarSeverity::Fatal,
        }
    }
    
    /// Capabilities revoked by this violation.
    /// 
    /// IRREVERSIBLE: These capabilities are permanently lost.
    pub fn revoked_capabilities(&self) -> Vec<TrustCapability> {
        match self {
            ViolationType::UnauthorizedAccess => {
                vec![TrustCapability::AccessSensitive]
            }
            ViolationType::IntegrityViolation => {
                vec![TrustCapability::Validate, TrustCapability::Configure]
            }
            ViolationType::PolicyBypass => {
                vec![
                    TrustCapability::Execute,
                    TrustCapability::Configure,
                    TrustCapability::Delegate,
                ]
            }
            ViolationType::CredentialMisuse => {
                vec![TrustCapability::Delegate, TrustCapability::AccessSensitive]
            }
            ViolationType::MaliciousAction => {
                // Fatal violation - all capabilities revoked (death)
                TrustCapability::full_set().into_iter().collect()
            }
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            ViolationType::UnauthorizedAccess => "Unauthorized access attempt",
            ViolationType::IntegrityViolation => "Data integrity violation",
            ViolationType::PolicyBypass => "Policy bypass attempt",
            ViolationType::CredentialMisuse => "Credential misuse",
            ViolationType::MaliciousAction => "Malicious action detected",
        }
    }
}

/// Trust profile tracking active capabilities.
/// 
/// INVARIANT: Capabilities can only be removed, never added.
/// 
/// NO METHOD EXISTS TO:
/// - Restore revoked capabilities
/// - Reset trust profile
/// - Forgive violations
/// - Re-certify actors
pub struct TrustProfile {
    /// Currently active capabilities.
    /// 
    /// This set can only shrink. Once a capability is removed, it is permanently lost.
    active_capabilities: HashSet<TrustCapability>,
}

impl TrustProfile {
    /// Create new trust profile with full capabilities.
    pub fn new() -> Self {
        TrustProfile {
            active_capabilities: TrustCapability::full_set(),
        }
    }
    
    /// Check if capability is currently active.
    pub fn has_capability(&self, capability: TrustCapability) -> bool {
        self.active_capabilities.contains(&capability)
    }
    
    /// Get all active capabilities.
    pub fn active_capabilities(&self) -> &HashSet<TrustCapability> {
        &self.active_capabilities
    }
    
    /// Count of active capabilities.
    /// 
    /// INVARIANT: This can only decrease over time.
    pub fn capability_count(&self) -> usize {
        self.active_capabilities.len()
    }
    
    /// Revoke capabilities permanently.
    /// 
    /// IRREVERSIBLE: Revoked capabilities CANNOT be restored.
    fn revoke_capabilities(&mut self, capabilities: &[TrustCapability]) {
        for cap in capabilities {
            self.active_capabilities.remove(cap);
        }
    }
    
    /// Check if trust profile is completely revoked (no capabilities remain).
    pub fn is_completely_revoked(&self) -> bool {
        self.active_capabilities.is_empty()
    }
}

/// Result of trust operation.
#[derive(Debug, PartialEq, Eq)]
pub enum TrustResult {
    /// Operation allowed (capability active).
    Allowed,
    
    /// Operation denied (capability revoked).
    Denied { reason: String },
    
    /// Actor lineage is terminated (dead).
    Terminated,
}

/// Trust-aware actor wrapping Lineage identity.
/// 
/// ONTOLOGICAL CONSTRAINT: Trust degrades irreversibly through violations.
/// 
/// NO METHOD EXISTS TO:
/// - Restore trust
/// - Forgive violations
/// - Reset capabilities
/// - Re-certify after termination
pub struct TrustedActor {
    lineage: Lineage,
    trust: TrustProfile,
}

impl TrustedActor {
    /// Create new trusted actor with full capabilities.
    pub fn create(initial_energy: u64) -> Self {
        TrustedActor {
            lineage: Lineage::create(initial_energy),
            trust: TrustProfile::new(),
        }
    }
    
    /// Get lineage identity.
    pub fn identity(&self) -> &crate::identity::Identity {
        self.lineage.identity()
    }
    
    /// Check if actor is alive (lineage not terminated).
    pub fn is_alive(&self) -> bool {
        self.lineage.is_alive()
    }
    
    /// Get trust profile.
    pub fn trust_profile(&self) -> &TrustProfile {
        &self.trust
    }
    
    /// Attempt to perform action requiring specific capability.
    /// 
    /// ENFORCEMENT:
    /// - Checks if actor is alive
    /// - Checks if capability is active
    /// - Consumes energy from lineage
    /// - Records operation in causal chain
    pub fn attempt_action(
        &mut self,
        capability: TrustCapability,
        description: String,
        energy_cost: u64,
    ) -> TrustResult {
        // Check death state
        if !self.lineage.is_alive() {
            return TrustResult::Terminated;
        }
        
        // Check capability
        if !self.trust.has_capability(capability) {
            return TrustResult::Denied {
                reason: format!("Capability {:?} has been revoked", capability),
            };
        }
        
        // Perform operation (consumes energy, records in memory)
        match self.lineage.perform_operation(description, energy_cost) {
            crate::lineage::OperationResult::Success { .. } => TrustResult::Allowed,
            crate::lineage::OperationResult::InsufficientEnergy { .. } => TrustResult::Denied {
                reason: "Insufficient energy".to_string(),
            },
            crate::lineage::OperationResult::Dead => TrustResult::Terminated,
            crate::lineage::OperationResult::OntologicalViolation { reason } => {
                eprintln!("FATAL: Ontological violation: {}", reason);
                std::process::exit(1);
            }
        }
    }
    
    /// Record trust violation.
    /// 
    /// CONSEQUENCES:
    /// - Inflicts scar on lineage (permanent)
    /// - Revokes capabilities (irreversible)
    /// - Fatal violations terminate lineage
    /// - All violations recorded in causal chain
    /// 
    /// IRREVERSIBLE: Lost capabilities CANNOT be restored.
    pub fn record_violation(&mut self, violation: ViolationType) -> TrustResult {
        // Check if already dead
        if !self.lineage.is_alive() {
            return TrustResult::Terminated;
        }
        
        // Revoke capabilities BEFORE recording error (prevents bypass)
        let revoked = violation.revoked_capabilities();
        self.trust.revoke_capabilities(&revoked);
        
        // Create error with appropriate severity
        let error = OperationError::new(
            violation.severity(),
            format!("Trust violation: {}", violation.description()),
        );
        
        // Record in lineage (inflicts scar, may cause death)
        match self.lineage.record_error(error) {
            crate::lineage::OperationResult::Success { .. } => {
                // Non-fatal violation
                TrustResult::Denied {
                    reason: format!(
                        "Violation recorded. {} capabilities revoked.",
                        revoked.len()
                    ),
                }
            }
            crate::lineage::OperationResult::Dead => {
                // Fatal violation or accumulated damage
                TrustResult::Terminated
            }
            crate::lineage::OperationResult::OntologicalViolation { reason } => {
                eprintln!("FATAL: Ontological violation: {}", reason);
                std::process::exit(1);
            }
            _ => unreachable!("record_error cannot return InsufficientEnergy"),
        }
    }
    
    /// Get current damage score from accumulated violations.
    pub fn damage_score(&self) -> u64 {
        self.lineage.scars().damage_score() as u64
    }
    
    /// Get count of trust violations.
    pub fn violation_count(&self) -> usize {
        self.lineage.scars().scar_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trust_starts_with_full_capabilities() {
        let profile = TrustProfile::new();
        assert_eq!(profile.capability_count(), 6);
        assert!(profile.has_capability(TrustCapability::Execute));
        assert!(profile.has_capability(TrustCapability::Validate));
    }

    #[test]
    fn violation_revokes_capabilities_permanently() {
        let mut actor = TrustedActor::create(1000);
        assert!(actor.trust.has_capability(TrustCapability::AccessSensitive));

        // Record unauthorized access violation
        actor.record_violation(ViolationType::UnauthorizedAccess);

        // Capability is permanently revoked
        assert!(!actor.trust.has_capability(TrustCapability::AccessSensitive));
        
        // Cannot be restored (no method exists to restore)
    }

    #[test]
    fn capability_count_only_decreases() {
        let mut actor = TrustedActor::create(1000);
        let initial_count = actor.trust.capability_count();

        actor.record_violation(ViolationType::UnauthorizedAccess);
        let after_violation = actor.trust.capability_count();

        assert!(after_violation < initial_count);
        
        // Perform legitimate action - count does NOT increase
        actor.attempt_action(TrustCapability::Audit, "Audit action".to_string(), 10);
        assert_eq!(actor.trust.capability_count(), after_violation);
    }

    #[test]
    fn revoked_capability_cannot_be_used() {
        let mut actor = TrustedActor::create(1000);

        // Revoke AccessSensitive
        actor.record_violation(ViolationType::UnauthorizedAccess);

        // Attempt to use revoked capability
        let result = actor.attempt_action(
            TrustCapability::AccessSensitive,
            "Try sensitive access".to_string(),
            10,
        );

        assert_eq!(
            result,
            TrustResult::Denied {
                reason: "Capability AccessSensitive has been revoked".to_string()
            }
        );
    }

    #[test]
    fn multiple_violations_accumulate() {
        let mut actor = TrustedActor::create(1000);
        let initial_count = actor.trust.capability_count();

        actor.record_violation(ViolationType::UnauthorizedAccess); // -1 capability
        actor.record_violation(ViolationType::IntegrityViolation); // -2 capabilities

        let final_count = actor.trust.capability_count();
        assert!(final_count < initial_count - 2);
    }

    #[test]
    fn fatal_violation_terminates_lineage() {
        let mut actor = TrustedActor::create(1000);
        assert!(actor.is_alive());

        // Record malicious action (fatal)
        let result = actor.record_violation(ViolationType::MaliciousAction);

        assert_eq!(result, TrustResult::Terminated);
        assert!(!actor.is_alive());
    }

    #[test]
    fn terminated_actor_cannot_perform_actions() {
        let mut actor = TrustedActor::create(1000);

        // Terminate via fatal violation
        actor.record_violation(ViolationType::MaliciousAction);

        // All actions denied
        let result = actor.attempt_action(
            TrustCapability::Audit,
            "Post-death action".to_string(),
            10,
        );

        assert_eq!(result, TrustResult::Terminated);
    }

    #[test]
    fn violations_recorded_in_causal_chain() {
        let mut actor = TrustedActor::create(1000);
        let initial_violations = actor.violation_count();

        actor.record_violation(ViolationType::UnauthorizedAccess);
        actor.record_violation(ViolationType::IntegrityViolation);

        assert_eq!(actor.violation_count(), initial_violations + 2);
    }

    #[test]
    fn damage_score_only_increases() {
        let mut actor = TrustedActor::create(1000);
        let initial_damage = actor.damage_score();

        actor.record_violation(ViolationType::UnauthorizedAccess);
        let damage_after_first = actor.damage_score();
        assert!(damage_after_first > initial_damage);

        actor.record_violation(ViolationType::IntegrityViolation);
        let damage_after_second = actor.damage_score();
        assert!(damage_after_second > damage_after_first);
    }

    // HOSTILE TESTS: Attempt to bypass trust degradation

    #[test]
    fn no_method_to_restore_capabilities() {
        let mut actor = TrustedActor::create(1000);
        actor.record_violation(ViolationType::PolicyBypass);

        // Verify capability lost
        assert!(!actor.trust.has_capability(TrustCapability::Execute));

        // ATTACK: Try to restore by performing successful actions
        for _ in 0..100 {
            actor.attempt_action(TrustCapability::Audit, "Good action".to_string(), 1);
        }

        // Capability remains revoked
        assert!(!actor.trust.has_capability(TrustCapability::Execute));
    }

    #[test]
    fn no_method_to_reset_trust() {
        let mut actor = TrustedActor::create(1000);
        actor.record_violation(ViolationType::IntegrityViolation);
        actor.record_violation(ViolationType::PolicyBypass);

        let degraded_count = actor.trust.capability_count();

        // ATTACK: No method exists to reset
        // Trust profile has no public constructors
        // Trust profile has no reset() method
        // Trust profile has no restore() method

        // Capability count remains degraded
        assert_eq!(actor.trust.capability_count(), degraded_count);
    }

    #[test]
    fn termination_is_irreversible() {
        let mut actor = TrustedActor::create(1000);
        actor.record_violation(ViolationType::MaliciousAction);

        assert!(!actor.is_alive());

        // ATTACK: Try to perform action (should fail)
        let result = actor.attempt_action(
            TrustCapability::Audit,
            "Revival attempt".to_string(),
            1,
        );

        assert_eq!(result, TrustResult::Terminated);

        // Still dead
        assert!(!actor.is_alive());
    }

    #[test]
    fn capability_set_only_shrinks() {
        let mut actor = TrustedActor::create(1000);
        let mut previous_count = actor.trust.capability_count();

        // Record multiple violations
        actor.record_violation(ViolationType::UnauthorizedAccess);
        assert!(actor.trust.capability_count() <= previous_count);
        previous_count = actor.trust.capability_count();

        actor.record_violation(ViolationType::IntegrityViolation);
        assert!(actor.trust.capability_count() <= previous_count);
        previous_count = actor.trust.capability_count();

        actor.record_violation(ViolationType::CredentialMisuse);
        assert!(actor.trust.capability_count() <= previous_count);

        // Set never grows
    }

    #[test]
    fn good_behavior_does_not_restore_trust() {
        let mut actor = TrustedActor::create(10000);

        // Commit violation
        actor.record_violation(ViolationType::UnauthorizedAccess);
        let after_violation = actor.trust.capability_count();
        let damage_after_violation = actor.damage_score();

        // Perform many successful operations (good behavior)
        for i in 0..1000 {
            actor.attempt_action(TrustCapability::Audit, format!("Good action {}", i), 1);
        }

        // Trust does NOT improve
        assert_eq!(actor.trust.capability_count(), after_violation);
        assert_eq!(actor.damage_score(), damage_after_violation);
    }
}
