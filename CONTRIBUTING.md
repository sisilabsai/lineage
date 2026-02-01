# Contributing to Lineage

## An Invitation

This is a serious project with serious principles.

If you believe that **consequence matters**, that **permanence is feature not bug**, and that **integrity is more important than convenience**, then we are building the future together.

We are looking for collaborators who want to fundamentally rethink how software handles identity, consequence, and trust.

If this resonates with you, read on. This is your project.

---

## What We're Building

Lineage is not another framework optimizing for adoption metrics.

It is a **manifesto in code**: a proof that systems can be designed where:

- Identity is permanent and cannot be erased
- History is immutable and cannot be rewritten
- Consequence is irreversible and cannot be undone
- Trust is earned through mathematics, not promises

We are building proof that **another way is possible**.

---

## Core Principle

Ontological integrity outranks all other concerns.

Features, convenience, performance, ergonomics, and compatibility are secondary to preserving irreversible constraints.

---

## Before Contributing

Read in order:

1. [MANIFESTO.md](MANIFESTO.md) — The philosophical declaration
2. [DOCTRINE.md](DOCTRINE.md) — The irreducible principles
3. [FREEZE.md](FREEZE.md) — The ontological freeze declaration
4. [README.md](README.md) — The system documentation

If you disagree with irreversibility as a foundational constraint, this project is not for you.

If you believe consequence should be optional, we cannot collaborate.

If you think "just this once" exceptions are acceptable, this is not your project.

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

- Pass all existing tests (120+ tests minimum)
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

Are answered in [FREEZE.md](FREEZE.md).

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

---

# Technical Contribution Guide

Once you accept the philosophical foundation above, here's how to contribute code:

## Getting Started

### Prerequisites

- **Rust 1.70+** (install from [rustup.rs](https://rustup.rs/))
- **Cargo** (comes with Rust)
- **Git**

### Setup

```bash
git clone https://github.com/lineage-finance/lineage.git
cd lineage
cargo build
cargo test
```

### Running Examples

```bash
# Default: 100K capital, 100 rounds
cargo run --example decentralized_trading_agent

# Custom capital and rounds
cargo run --example decentralized_trading_agent -- --capital 75000 --rounds 300

# Save results to CSV
cargo run --example decentralized_trading_agent -- --output results.csv

# Specific strategy
cargo run --example decentralized_trading_agent -- --strategy momentum --verbose
```

## Project Structure

```
src/
├── agent.rs                 # Base Agent trait and behavior types
├── behavior.rs              # Core behavior implementations
├── graveyard.rs             # Archived agents (irreversible)
├── identity.rs              # AgentId, genealogy tracking
├── lineage.rs               # Inheritance and spawning logic
├── memory.rs                # Memory mechanisms (sealed)
├── metabolism.rs            # Energy/resource management
├── scar.rs                  # Injury tracking (irreversible)
├── trust.rs                 # Trust scoring system
├── finance/
│   ├── agent.rs             # FinanceAgent implementation
│   ├── arena.rs             # Competitive arena simulation
│   ├── trade.rs             # Trade execution (immutable)
│   ├── trust.rs             # Trust formula calculations
│   └── spawning.rs          # Offspring creation
├── main.rs                  # CLI entry point
└── lib.rs                   # Public API

examples/
├── decentralized_trading_agent.rs    # Main trading demo (CLI customizable)
├── interactive_consensus_arena.rs    # Arena with user interaction
└── ...

tests/
└── [integration tests]
```

## Code Style

- Use `cargo fmt` to format
- Use `cargo clippy` to lint
- Follow Rust naming conventions:
  - `snake_case` for functions, variables, modules
  - `PascalCase` for types, traits, structs
  - `SCREAMING_SNAKE_CASE` for constants
- Document all public items with examples

### Documentation Template

```rust
/// Brief one-line description.
///
/// Longer explanation if relevant. Include edge cases and
/// any irreversibility guarantees.
///
/// # Examples
/// ```
/// let agent = FinanceAgent::new("Test".to_string(), 100_000, 0);
/// assert_eq!(agent.get_capital(), 100_000);
/// ```
pub fn my_function() { }
```

## Testing

### Add Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_feature() {
        // Test implementation
        assert_eq!(result, expected);
    }
}
```

### Add Integration Tests

Place in `tests/` directory:

```rust
use lineage::finance::FinanceAgent;

#[test]
fn test_full_workflow() {
    let mut agent = FinanceAgent::new("Test".to_string(), 100_000, 0);
    // Full workflow test
}
```

### Run Tests

```bash
cargo test              # All tests
cargo test -- --nocapture  # Show output
cargo test finance::    # Module tests
```

Aim for 80%+ code coverage on new code.

## Contribution Areas

### 1. Finance Engine 📊
- New trading strategies
- Improved market simulation
- Enhanced metrics
- Performance optimization

### 2. Trust Systems 🔐
- Enhanced scoring algorithms
- Blockchain integration
- Audit trail improvements
- Governance proposals

### 3. Evolutionary Mechanics 🧬
- Trait inheritance enhancements
- Mutation strategies
- Spawning improvements
- Genealogy tracking

### 4. Machine Learning 🤖
- ML model integration (tch-rs)
- Feature engineering
- Reinforcement learning
- Strategy optimization

### 5. Visualization 📈
- Dashboard implementation
- Metrics visualization
- Real-time monitoring
- Chart generation

### 6. Documentation 📚
- API documentation
- Tutorials
- Examples
- Architecture guides

### 7. Testing & Quality ✅
- Unit tests
- Integration tests
- Benchmarks
- Property tests

## Pull Request Checklist

- [ ] Code compiles: `cargo build`
- [ ] Tests pass: `cargo test`
- [ ] Formatting: `cargo fmt` (run this!)
- [ ] Linting: `cargo clippy` (no warnings)
- [ ] No new warnings generated
- [ ] Added/updated tests for new functionality
- [ ] Documentation is complete and clear
- [ ] Commits follow conventional format
- [ ] PR description explains what and why
- [ ] No forbidden operations added
- [ ] No bypass patterns introduced
- [ ] All invariants preserved

## PR Description Template

```markdown
## Description
What does this PR do?

## Type
- [ ] Bug fix
- [ ] Feature
- [ ] Refactor
- [ ] Test/Documentation

## Related Issues
Closes #123

## Changes
- What was changed
- Why it was changed
- How it preserves constraints

## Testing
- [ ] New tests added
- [ ] Existing tests pass
- [ ] Invariants verified

## Verification
Steps to test this change:

```bash
cargo run --example decentralized_trading_agent -- --capital 50000 --rounds 100
```

Expected output: [describe]
```

## Commit Message Format

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add feature description
fix: correct specific bug
docs: update documentation
test: add tests for feature
refactor: improve code structure
chore: dependency updates
```

## Acceptance Criteria

All PRs must satisfy:

1. **Ontological**: No weakening of constraints
2. **Functional**: Feature works as designed, no regressions
3. **Tested**: Tests pass, 80%+ coverage on new code
4. **Documented**: Clear docs, normative language for constraints
5. **Formatted**: cargo fmt and cargo clippy pass
6. **Committed**: Clear, conventional messages

## Getting Help

- **Questions**: Open a [Discussion](../../discussions)
- **Bugs**: Open an [Issue](../../issues) with "bug" label
- **Features**: [Discussions](../../discussions) or [Issues](../../issues) with "enhancement"
- **Clarifications**: Read MANIFESTO.md, DOCTRINE.md, FREEZE.md

## Important Reminders

Before contributing, please:

1. Read the philosophical documents (yes, all of them)
2. Confirm you accept irreversibility as foundational
3. Understand that PRs weakening constraints will be rejected
4. Accept that code quality won't override ontological concerns

Thank you for building with integrity! 🚀
