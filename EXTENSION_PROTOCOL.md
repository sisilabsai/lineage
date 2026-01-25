# Extension Evaluation Protocol

All proposed extensions must pass three questions.

---

## Evaluation Questions

### Question 1: Does this introduce reversibility?

**YES if the extension:**
- Enables undo, rollback, or replay
- Restores consumed resources
- Removes or modifies past state
- Bypasses death or termination
- Allows scar removal or healing
- Permits memory erasure or unsealing
- Downgrades error severity after creation

**NO if the extension:**
- Only appends new state
- Only consumes additional resources
- Only reads existing state
- Only accumulates consequences

---

### Question 2: Does this weaken identity, memory, or consequence?

**YES if the extension:**
- Permits identity duplication or comparison
- Allows history modification
- Makes constraints optional or configurable
- Introduces bypass mechanisms
- Reduces damage from errors
- Prevents mandatory death triggers

**NO if the extension:**
- Preserves uniqueness of identity
- Maintains append-only history
- Enforces all existing constraints
- Adds consequences (not removes them)
- Works within existing primitives

---

### Question 3: Does this rely on human discipline instead of enforcement?

**YES if the extension:**
- Requires caller to "promise" not to misuse
- Depends on documentation warnings
- Relies on convention over compilation
- Uses public mutable fields for "immutable" data
- Expects developers to avoid certain methods
- Makes critical invariants opt-in

**NO if the extension:**
- Enforces constraints at compile time (type system)
- Verifies invariants at runtime (checks)
- Makes violations impossible or panics
- Uses private fields with read-only accessors
- Prevents misuse through API design

---

## Decision Matrix

| Q1: Reversibility? | Q2: Weakens core? | Q3: Requires discipline? | **Result** |
|--------------------|-------------------|-------------------------|------------|
| YES | - | - | **REJECT** |
| NO | YES | - | **REJECT** |
| NO | NO | YES | **REJECT** |
| NO | NO | NO | **ACCEPT** |

**If ANY answer is YES: The extension is invalid.**

**If ALL answers are NO: The extension may proceed as compositional only.**

---

## Examples

### Example 1: Add `query_scars_by_severity(severity: ScarSeverity) -> Vec<&Scar>`

**Q1: Reversibility?** NO — Read-only query, no state modification  
**Q2: Weakens core?** NO — Preserves scar immutability  
**Q3: Requires discipline?** NO — Type system prevents mutation (returns `&Scar`, not `&mut Scar`)

**Result**: ACCEPT (compositional read-only method)

---

### Example 2: Add `restore_energy(amount: u64)`

**Q1: Reversibility?** YES — Restores consumed resource  
**Q2: Weakens core?** YES — Violates monotonic decrease  
**Q3: Requires discipline?** N/A

**Result**: REJECT

---

### Example 3: Add `pub healing_enabled: bool` configuration field

**Q1: Reversibility?** YES — Enables constraint bypass  
**Q2: Weakens core?** YES — Makes constraints optional  
**Q3: Requires discipline?** YES — Relies on caller not setting to `true`

**Result**: REJECT (fails all three)

---

### Example 4: Add `NetworkBehavior` enforcing connection limit with strain scars

**Q1: Reversibility?** NO — Accumulates strain, no removal  
**Q2: Weakens core?** NO — Adds consequences, follows PulseBehavior pattern  
**Q3: Requires discipline?** NO — Strain inflicted via `record_error()`, enforced by system

**Result**: ACCEPT (compositional behavior, preserves invariants)

---

### Example 5: Add `OperationError::set_severity(&mut self, new_severity: ScarSeverity)`

**Q1: Reversibility?** YES — Allows severity downgrade (fatal → minor)  
**Q2: Weakens core?** YES — Permits death bypass  
**Q3: Requires discipline?** YES — Relies on caller not downgrading fatal errors

**Result**: REJECT (fails all three)

---

### Example 6: Add `try_consume_energy(amount: u64) -> Option<u64>`

If implementation:
- Returns `Some(consumed)` on success, `None` on insufficient energy
- Consumes energy on success (monotonic decrease preserved)
- Does not restore energy on failure

**Q1: Reversibility?** NO — Energy still monotonically decreases  
**Q2: Weakens core?** NO — Preserves metabolism constraints  
**Q3: Requires discipline?** NO — Type system enforces consumption

**Result**: ACCEPT (compositional API improvement)

---

## Constraints on Accepted Extensions

Even if an extension passes evaluation, it must be:

### Compositional Only

- Built atop existing primitives
- Does not modify primitive semantics
- Does not alter core modules

### Additive Only

- Adds new capabilities
- Does not remove restrictions
- Does not change existing behavior

### Invariant-Preserving

- All existing tests still pass
- Adds tests for new functionality
- No regression in constraint enforcement

---

## Forbidden Redesigns

Do not:

- Refactor core primitives "for clarity"
- Generalize constraints "for flexibility"
- Abstract invariants "for extensibility"
- Make enforcement "pluggable"

Core is frozen. Extensions are peripheral.

---

## Evaluation Procedure

1. Read the proposal
2. Answer Q1, Q2, Q3 with YES or NO
3. If any YES: Reject immediately with reason
4. If all NO: Proceed to compositional review
5. Verify no core modification
6. Require tests
7. Accept or reject on technical merits

---

## Response Templates

### Rejection: Reversibility Introduced

```
This extension introduces reversibility, which violates frozen primitives.

Evaluation:
- Q1 (Reversibility): YES
- Reason: [Specific reversible operation]

See FREEZE.md, Section: Forbidden Operations.

REJECTED.
```

### Rejection: Core Weakened

```
This extension weakens core primitives.

Evaluation:
- Q2 (Weakens core): YES
- Reason: [Specific weakening]

See FREEZE.md, Section: Immutable Foundations.

REJECTED.
```

### Rejection: Requires Discipline

```
This extension relies on human discipline instead of enforcement.

Evaluation:
- Q3 (Requires discipline): YES
- Reason: [Specific unenforced constraint]

Lineage enforces constraints via type system and runtime checks, 
not documentation.

REJECTED.
```

### Acceptance: Compositional Extension

```
This extension passes evaluation.

Evaluation:
- Q1 (Reversibility): NO
- Q2 (Weakens core): NO
- Q3 (Requires discipline): NO

Proceeding to compositional review. Ensure:
- No modification of core modules
- All existing tests pass
- New tests included

If these conditions are met, this may be accepted.
```

---

## Final Authority

The three questions are final.

No exceptions. No appeals. No "but in this case..."

Pass all three or reject immediately.
