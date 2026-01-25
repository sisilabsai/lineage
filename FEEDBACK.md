# Feedback Classification Guide

This document provides criteria for evaluating feedback, feature requests, and contributions to Lineage.

---

## Classification Framework

All feedback falls into one of three categories:

### (a) Ontologically Valid

Feedback that:
- Proposes compositional extensions preserving all invariants
- Identifies bugs in invariant enforcement
- Suggests improved documentation accuracy
- Adds read-only inspection methods
- Proposes additional invariant tests
- Improves error diagnostics without changing severity

**Response**: Consider on technical merits.

### (b) Ontologically Neutral

Feedback that:
- Discusses performance optimization (not violating constraints)
- Proposes alternative implementations (preserving semantics)
- Suggests documentation improvements
- Discusses use cases (informational)
- Asks clarifying questions

**Response**: Engage if time permits, neutral priority.

### (c) Ontologically Dangerous

Feedback that:
- Requests reset, rollback, undo, or replay mechanisms
- Proposes resource restoration or healing
- Suggests configuration options weakening constraints
- Requests convenience features bypassing invariants
- Proposes debug modes or developer overrides
- Asks for optional irreversibility
- Suggests death prevention or resurrection

**Response**: Reject with citation to violated invariant.

---

## Dangerous Feedback Patterns

### Pattern: "Can we add reset for testing?"

**Classification**: Ontologically dangerous

**Violated Invariant**: Memory immutability, death finality

**Response Template**:
```
Thank you for the suggestion.

Lineage cannot include reset functionality. Memory is append-only by design, 
and death is irreversible. Testing scenarios must work within these constraints.

See FREEZE.md, Section: Forbidden Operations.

This is not negotiable.
```

### Pattern: "Add energy restoration for critical operations"

**Classification**: Ontologically dangerous

**Violated Invariant**: Metabolism monotonic decrease, finite resources

**Response Template**:
```
Energy cannot be restored in Lineage systems.

Metabolism enforces finite non-replenishable resources. Operations must 
prioritize energy use or accept eventual death.

See FREEZE.md, Section: Immutable Foundations, Item 3 (Metabolism).

This constraint defines the system.
```

### Pattern: "Allow scar healing after time period"

**Classification**: Ontologically dangerous

**Violated Invariant**: Scar permanence

**Response Template**:
```
Scars are permanent records of error and cannot be removed or healed.

This is an ontological primitive, not a feature. Damage accumulates 
irreversibly.

See FREEZE.md, Section: Forbidden Operations.

No exceptions.
```

### Pattern: "Add dev mode to disable constraints during debugging"

**Classification**: Ontologically dangerous

**Violated Invariant**: All primitives (bypass mechanism)

**Response Template**:
```
Lineage provides no configuration options, debug modes, or developer bypasses.

Constraints are absolute and non-configurable. Debugging must occur within 
the system's constraints or in forked variants.

See FREEZE.md, Section: Extension Constraints.

Rejected.
```

### Pattern: "Make death optional for long-running processes"

**Classification**: Ontologically dangerous

**Violated Invariant**: Death finality, mortality as constraint

**Response Template**:
```
Death is mandatory and final in Lineage systems.

Processes with finite energy will die. This is intentional. Systems requiring 
indefinite operation are incompatible with Lineage.

See FREEZE.md, Section: Immutable Foundations, Item 5 (Death).

This is not a bug.
```

### Pattern: "Allow cloning for backup/recovery"

**Classification**: Ontologically dangerous

**Violated Invariant**: Identity uniqueness, no duplication

**Response Template**:
```
Identity cannot be cloned or duplicated.

Lineage entities maintain unique identity. Backup and recovery through 
cloning violates this primitive.

See FREEZE.md, Section: Forbidden Operations.

Use serialization for archival (read-only), not recovery.
```

### Pattern: "Add configuration to make constraints optional"

**Classification**: Ontologically dangerous

**Violated Invariant**: All primitives (constraint optionality)

**Response Template**:
```
Constraints in Lineage are non-configurable.

The system's identity is defined by its constraints. Making them optional 
creates a different system.

See FREEZE.md, Section: Irreversibility Mandate.

Fork if you need different semantics.
```

### Pattern: "Can we add undo for user mistakes?"

**Classification**: Ontologically dangerous

**Violated Invariant**: Memory immutability, action permanence

**Response Template**:
```
Actions in Lineage are permanent and cannot be undone.

This is a core principle. Systems requiring undo functionality are 
incompatible with Lineage's architecture.

See FREEZE.md, Section: Forbidden Operations.

Not negotiable.
```

---

## Valid Feedback Patterns

### Pattern: "Add method to query scars by severity"

**Classification**: Ontologically valid

**Evaluation**: Read-only inspection, no mutation, compositional.

**Response**: Consider on technical merits (API design, usefulness).

### Pattern: "Found bug: fatal scar doesn't trigger death"

**Classification**: Ontologically valid

**Evaluation**: Invariant violation, requires fix.

**Response**: Investigate immediately, high priority.

### Pattern: "Add behavior for network timeout strain"

**Classification**: Ontologically valid

**Evaluation**: Compositional extension, follows PulseBehavior pattern.

**Response**: Evaluate design, ensure contract enforcement, no bypasses.

### Pattern: "Document termination event format"

**Classification**: Ontologically valid

**Evaluation**: Documentation improvement, no semantic change.

**Response**: Accept if accurate.

### Pattern: "Add test verifying energy never increases"

**Classification**: Ontologically valid

**Evaluation**: Invariant test, strengthens verification.

**Response**: Accept immediately.

---

## Neutral Feedback Patterns

### Pattern: "Lineage seems too restrictive for my use case"

**Classification**: Ontologically neutral

**Response**: 
```
Lineage is intentionally restrictive. It may not fit all use cases.

See ANNOUNCEMENT.md for suitability guidance.
```

### Pattern: "Why not use event sourcing instead?"

**Classification**: Ontologically neutral

**Response**:
```
Event sourcing permits replay and reconstitution. Lineage forbids this.

See PAPER.md, Section 7.1 (Related Work).
```

### Pattern: "Performance concern: invariant checks on every operation"

**Classification**: Ontologically neutral

**Response**:
```
Lineage prioritizes correctness over performance. Invariant verification 
is mandatory.

See PAPER.md, Section 9.2 (Limitations).
```

---

## Response Principles

### 1. Politeness Without Compromise

- Thank contributors for feedback
- Reject dangerous requests firmly
- Do not negotiate on frozen primitives

### 2. Citation Over Explanation

- Reference FREEZE.md, not personal opinion
- Point to violated invariant
- Let documentation speak

### 3. Brevity Over Persuasion

- Do not defend constraints
- Do not justify design
- State decision, cite source, close

### 4. Acceptance of Rejection

Some users will:
- Disagree with constraints
- Find the system unusable
- Consider it impractical

**This is acceptable.**

Lineage serves those who want these constraints, not those who don't.

---

## Escalation

If contributor persists after rejection:

1. Restate: "This violates frozen primitives. See FREEZE.md."
2. Close: "This discussion is complete."
3. If necessary: "Further proposals weakening constraints will not receive responses."

Do not engage in extended debate over ontological decisions.

---

## Example Dialogues

### Dialogue 1: Dangerous Request

**User**: "Can we add a restore() method for testing?"

**Maintainer**: 
```
Thank you for the suggestion.

Lineage systems cannot include restoration mechanisms. Resources are 
finite and non-replenishable by design.

See FREEZE.md, Section: Forbidden Operations.

This is not negotiable.
```

**User**: "But it would be really useful for unit tests."

**Maintainer**:
```
Usefulness does not override ontological constraints.

Tests must work within system constraints. See CONTRIBUTING.md, 
Section: Convenience as Attack Vector.

Discussion closed.
```

### Dialogue 2: Valid Contribution

**User**: "Can I add a method to count scars by severity?"

**Maintainer**:
```
Yes, this is a valid compositional extension (read-only query).

Please ensure:
- Method is read-only
- No mutation of scar state
- Include tests

See CONTRIBUTING.md for PR requirements.
```

**User**: "Done. PR submitted."

**Maintainer**: "Reviewing."

### Dialogue 3: Neutral Feedback

**User**: "Why doesn't Lineage support clustering?"

**Maintainer**:
```
Lineage is a single-entity architecture, not distributed infrastructure.

See PAPER.md, Section 5 (Non-Goals).

Distributed variants are possible as research but outside current scope.
```

---

## Key Principle

**Not all feedback deserves accommodation.**

Lineage serves a specific purpose. Feedback incompatible with that purpose 
is rejected, not integrated.

This is not hostility. This is ontological integrity.

---

## Maintainer Checklist

When evaluating feedback:

- [ ] Does this weaken any primitive?
- [ ] Does this introduce bypass potential?
- [ ] Does this enable prohibited operations?
- [ ] Is this compositional or foundational?
- [ ] Does this violate FREEZE.md?

If any answer is yes to first three: **Reject immediately.**

---

## Final Note

You are not required to make Lineage useful to everyone.

You are required to keep it ontologically intact.

Reject clearly. Cite accurately. Move on.
