# Trust Degradation System

## Overview

A real-world application of Lineage primitives demonstrating irreversible trust degradation for privileged actors.

---

## Domain Model

### Core Concept

**Trust is not a score. Trust is a set of capabilities that can only shrink.**

Traditional trust systems:
- Numeric reputation scores (can go up or down)
- Time-based forgiveness (violations expire)
- Re-certification processes (reset trust)
- Manual overrides (administrators bypass rules)

**Lineage Trust Model**:
- Trust = set of active capabilities
- Violations permanently revoke specific capabilities
- No restoration mechanism exists
- No appeals, no re-certification, no forgiveness

---

## Architecture

### TrustCapability

Six capabilities privileged actors may hold:

1. **Execute** — Run privileged operations
2. **Validate** — Verify other actors' operations
3. **Configure** — Modify system configuration
4. **AccessSensitive** — Read sensitive data
5. **Delegate** — Grant capabilities to others
6. **Audit** — Inspect system state

### ViolationType

Five violation classifications:

| Violation | Severity | Revoked Capabilities |
|-----------|----------|---------------------|
| `UnauthorizedAccess` | Minor | AccessSensitive |
| `IntegrityViolation` | Moderate | Validate, Configure |
| `PolicyBypass` | Severe | Execute, Configure, Delegate |
| `CredentialMisuse` | Severe | Delegate, AccessSensitive |
| `MaliciousAction` | Fatal | All (termination) |

### TrustProfile

Tracks active capabilities for an actor.

**Invariants**:
- Capability count is monotonically decreasing
- Revoked capabilities cannot be restored
- No methods exist to add capabilities
- No methods exist to reset profile

### TrustedActor

Wraps Lineage with trust enforcement.

**Operations**:
- `attempt_action(capability, description, cost)` — Requires active capability
- `record_violation(violation)` — Revokes capabilities permanently

**Enforcement**:
1. Check if actor is alive (lineage not dead)
2. Check if capability is active (not revoked)
3. Consume energy from lineage
4. Record operation in causal chain

---

## Consequence Model

### Minor Violations

Example: Unauthorized access attempt

**Immediate Consequences**:
- `AccessSensitive` capability permanently revoked
- Minor severity scar recorded in lineage
- Damage score increases by 1

**Long-term Consequences**:
- Cannot access sensitive data again
- Violation visible in causal history forever
- Contributes to cumulative damage score

### Moderate Violations

Example: Data integrity violation

**Immediate Consequences**:
- `Validate` and `Configure` capabilities permanently revoked
- Moderate severity scar recorded
- Damage score increases by 5

**Long-term Consequences**:
- Cannot validate operations
- Cannot modify configuration
- Accumulated damage may trigger death

### Severe Violations

Example: Policy bypass attempt

**Immediate Consequences**:
- `Execute`, `Configure`, and `Delegate` capabilities permanently revoked
- Severe scar recorded
- Damage score increases by 20

**Long-term Consequences**:
- Operational capability severely limited
- Cannot delegate trust to others
- High risk of death from further violations

### Fatal Violations

Example: Malicious action detected

**Immediate Consequences**:
- All capabilities revoked
- Lineage terminated (death)
- Memory sealed

**Long-term Consequences**:
- Actor cannot perform any actions
- Cannot be restored or resurrected
- Credential lineage is permanently terminated

---

## Why Forgiveness Is Forbidden

### The Problem with Restorative Trust

Traditional trust systems permit:

1. **Time-based recovery**: "Violation expires after 90 days"
   - **Problem**: Past actions lose meaning
   - **Reality**: Consequences are permanent

2. **Good behavior credits**: "10 successful operations cancel 1 violation"
   - **Problem**: Violations become transactional
   - **Reality**: Error cannot be undone

3. **Re-certification**: "Complete training to restore privileges"
   - **Problem**: Identity can be laundered
   - **Reality**: History is immutable

4. **Manual override**: "Administrator grants exception"
   - **Problem**: Rules become optional
   - **Reality**: Constraints are absolute

### Ontological Truth

In the physical world:
- Broken trust does not heal over time
- Good behavior does not erase past violations
- Consequences accumulate irreversibly
- Death is final

**Lineage enforces physical reality in software.**

### Operational Implications

**Discomfort is intentional.**

Actors with degraded trust:
- Experience permanent loss of capability
- Cannot undo past violations
- Must operate within reduced privilege set
- Face eventual termination if violations continue

This is not a bug. This is the system working correctly.

---

## Enforcement Mechanisms

### Compile-Time Prevention

```rust
// FORBIDDEN: No method exists to restore capabilities
impl TrustProfile {
    // pub fn restore_capability() { } // Does not exist
    // pub fn reset() { } // Does not exist
    // pub fn forgive() { } // Does not exist
}
```

### Runtime Enforcement

```rust
pub fn attempt_action(&mut self, capability: TrustCapability) -> TrustResult {
    // 1. Check death state (backed by Lineage)
    if !self.lineage.is_alive() {
        return TrustResult::Terminated;
    }
    
    // 2. Check capability (immutable set, can only shrink)
    if !self.trust.has_capability(capability) {
        return TrustResult::Denied { reason };
    }
    
    // 3. Perform operation (consumes energy, records in causal chain)
    self.lineage.perform_operation(description, cost)
}
```

### Invariant Tests

14 tests verify:

1. **Capability Monotonicity**: Count only decreases
2. **Permanent Revocation**: Lost capabilities never restored
3. **Violation Accumulation**: Damage score only increases
4. **Termination Finality**: Dead actors cannot act
5. **Bypass Prevention**: No restoration through any operation sequence

**Hostile tests prove** that 1000 successful operations after violation do not restore trust.

---

## Usage Example

```rust
use lineage::{TrustedActor, ViolationType, TrustCapability, TrustResult};

// Create actor with full trust
let mut actor = TrustedActor::create(10000);

// Actor has all capabilities initially
assert!(actor.trust_profile().has_capability(TrustCapability::Execute));
assert!(actor.trust_profile().has_capability(TrustCapability::AccessSensitive));

// Attempt authorized action
let result = actor.attempt_action(
    TrustCapability::Execute,
    "Legitimate operation".to_string(),
    100
);
assert_eq!(result, TrustResult::Allowed);

// Record violation
actor.record_violation(ViolationType::UnauthorizedAccess);

// Capability permanently revoked
assert!(!actor.trust_profile().has_capability(TrustCapability::AccessSensitive));

// Attempt to use revoked capability
let result = actor.attempt_action(
    TrustCapability::AccessSensitive,
    "Try sensitive access".to_string(),
    10
);
assert_eq!(result, TrustResult::Denied { /* ... */ });

// Good behavior does NOT restore trust
for _ in 0..1000 {
    actor.attempt_action(TrustCapability::Audit, "Good action".to_string(), 1);
}
assert!(!actor.trust_profile().has_capability(TrustCapability::AccessSensitive));

// Fatal violation terminates lineage
actor.record_violation(ViolationType::MaliciousAction);
assert!(!actor.is_alive());

// All future actions denied
let result = actor.attempt_action(
    TrustCapability::Audit,
    "Post-termination action".to_string(),
    1
);
assert_eq!(result, TrustResult::Terminated);
```

---

## Test Inventory

### Positive Tests (System Behavior)

1. `trust_starts_with_full_capabilities` — Initial state verification
2. `violation_revokes_capabilities_permanently` — Revocation mechanism
3. `revoked_capability_cannot_be_used` — Access control enforcement
4. `multiple_violations_accumulate` — Cumulative damage
5. `fatal_violation_terminates_lineage` — Death trigger
6. `terminated_actor_cannot_perform_actions` — Post-death denial
7. `violations_recorded_in_causal_chain` — Memory integration
8. `damage_score_only_increases` — Monotonic damage

### Negative Tests (Attack Prevention)

1. `no_method_to_restore_capabilities` — Proves no restoration API
2. `no_method_to_reset_trust` — Proves no reset mechanism
3. `termination_is_irreversible` — Proves death is final
4. `capability_set_only_shrinks` — Proves monotonic decrease
5. `good_behavior_does_not_restore_trust` — Proves no forgiveness
6. `capability_count_only_decreases` — Proves no capability addition

**All 14 tests passing** (verified hostile attempts fail).

---

## Design Decisions

### Why Capabilities, Not Scores?

**Numeric scores imply continuous improvement**:
- Score of 50 → 75 → 100 (recovery path exists)
- Violations become transactional

**Capability sets have discrete removal**:
- {A, B, C} → {A, C} (B is gone forever)
- Violations are existential

### Why No Time-Based Forgiveness?

Time-based expiration implies:
- Past becomes irrelevant
- Waiting game strategies emerge
- Consequence loses meaning

Lineage maintains:
- Permanent causal history
- Violations visible forever
- No statute of limitations

### Why No Re-Certification?

Re-certification suggests:
- Identity can be reset
- History can be erased
- Fresh start is possible

Lineage enforces:
- Identity is unique and permanent
- History is append-only
- No second chances

### Why No Manual Override?

Administrator overrides create:
- Rule optionality
- Privilege escalation paths
- Inconsistent enforcement

Lineage guarantees:
- Constraints are absolute
- No bypass mechanisms exist
- Enforcement is non-negotiable

---

## Operational Reality

### What This Means for Operators

**If you deploy this system**:

- Violations permanently reduce capability
- There is no support hotline for trust restoration
- There is no exception request process
- There is no appeal mechanism

**This is uncomfortable by design.**

Trust degradation mirrors physical consequence:
- You cannot un-ring a bell
- You cannot un-break a promise
- You cannot un-commit a violation

### Suitability

**This system is suitable for**:
- High-security environments where forgiveness is ontologically wrong
- Contexts where past violations should permanently affect future capability
- Systems where accountability requires irreversible consequence

**This system is NOT suitable for**:
- Contexts requiring rehabilitation
- Learning environments expecting mistakes
- Systems prioritizing user convenience over consequence

---

## Integration with Lineage

Trust degradation is **compositional**:

- Built atop Identity, Memory, Metabolism, Scars, Death
- Does not modify core primitives
- Uses Lineage mechanisms for enforcement
- Inherits all irreversibility guarantees

Trust extends Lineage without weakening it.

---

## Test Execution

```bash
# Run trust tests only
cargo test --lib trust

# Verify all 14 tests pass
# 8 positive tests (system behavior)
# 6 negative tests (attack prevention)
```

---

## Summary

**Trust Degradation System proves**:

1. Trust can be modeled as irreversible capability loss
2. Violations can permanently reduce privilege
3. Forgiveness can be made ontologically impossible
4. Physical consequence can be enforced in software

**The system is complete.**

No restoration. No forgiveness. No appeals.

This is not cruelty. This is consequence.
