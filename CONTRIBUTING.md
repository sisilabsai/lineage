# Contributing to Lineage

## Core Principle

Ontological integrity outranks all other concerns.

Features, convenience, performance, ergonomics, and compatibility are secondary to preserving irreversible constraints.

---

## Before Contributing

Read in order:

1. [FREEZE.md](FREEZE.md) — The ontological freeze declaration
2. [prompt.md](prompt.md) — The development constitution
3. [README.md](README.md) — The system documentation

If you disagree with irreversibility as a foundational constraint, this project is not for you.

---

## Contribution Standards

### Invariants Outrank Features

A contribution that:

- Adds useful functionality
- Improves performance
- Enhances ergonomics
- Increases adoption

**Will be rejected** if it weakens any ontological primitive.

### Convenience as Attack Vector

Contributions proposing convenience features must demonstrate they do not:

- Introduce bypass mechanisms
- Weaken invariant enforcement
- Enable prohibited operations through indirection
- Create configuration options that disable constraints

"Developer experience" arguments will not override ontological requirements.

### Forbidden Contributions

The following will be rejected immediately:

- Clone or Copy traits on Identity, Memory, Metabolism, Scars, or OperationError
- Methods that restore energy, remove scars, or erase history
- Debug modes, override flags, or developer bypasses
- Configuration options that weaken constraints
- Any form of reset, replay, rollback, undo, or recovery
- Healing, resurrection, revival, or death circumvention
- Identity recreation, duplication, or cloning
- Error severity mutation after creation
- Memory unsealing or termination reversal

### Allowed Contributions

Contributions are welcome that:

- Add compositional behaviors (like PulseBehavior)
- Improve error diagnostics without changing severity
- Add read-only inspection methods
- Extend serialization support
- Add derived metrics from existing state
- Improve documentation accuracy
- Fix bugs that preserve invariants
- Add tests verifying invariants

---

## Pull Request Process

### 1. Ontological Review

Every PR undergoes ontological review before code review.

Questions evaluated:

- Does this weaken any primitive?
- Does this introduce bypass potential?
- Does this enable prohibited operations?
- Does this violate the freeze declaration?

If any answer is yes, the PR is rejected regardless of code quality.

### 2. Test Requirements

All PRs must:

- Pass all existing tests (145 tests minimum)
- Add tests for new functionality
- Include invariant tests if touching core types
- Demonstrate no regression in constraint enforcement

### 3. Documentation Requirements

All PRs must:

- Update README.md if changing public API
- Use normative language (MUST, MUST NOT) for constraints
- Document forbidden operations explicitly
- Avoid marketing or subjective language

---

## Rejection Criteria

PRs may be rejected solely because they:

- Violate ontological principles
- Introduce convenience at the cost of integrity
- Enable future weakening of constraints
- Create precedent for prohibited patterns

Code quality, usefulness, and contributor reputation are not factors in ontological rejection.

---

## Review Philosophy

We assume:

- Contributors are competent
- Contributors are well-intentioned
- Contributors have read the documentation

Rejection is not personal. Rejection is ontological.

If a contribution is rejected on ontological grounds, it means:

- The primitive cannot be changed
- The constraint cannot be relaxed
- The system's nature prohibits this approach

This is not negotiable.

---

## Scope Boundaries

Lineage is:

- A reference implementation of irreversible consequence
- An exploration of ontological constraints in software
- A demonstration that certain guarantees are enforceable

Lineage is not:

- A general-purpose library seeking broad adoption
- A framework optimizing for developer convenience
- A system prioritizing feature richness

Contributions should align with what the system is, not what it could become.

---

## Questions and Discussion

Questions about:

- Why a constraint exists
- Whether an exception can be made
- If a bypass is acceptable "just this once"

Are answered in [FREEZE.md](FREEZE.md) and [prompt.md](prompt.md).

If those documents do not satisfy your question, the answer is: **No.**

---

## Commitment

By contributing, you acknowledge:

- You have read FREEZE.md and accept the ontological freeze
- You understand convenience does not justify weakening constraints
- You accept that PRs may be rejected on ontological grounds alone
- You will not propose prohibited operations

---

## Final Note

Lineage's constraints are intentional, not accidental.

They are features, not bugs.

They define the system's identity.

Weakening them does not improve Lineage.

It destroys what Lineage is.
