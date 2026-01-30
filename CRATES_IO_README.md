# Lineage

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/lineage.svg)](https://crates.io/crates/lineage)
[![Documentation](https://docs.rs/lineage/badge.svg)](https://docs.rs/lineage)

**Software identity preserved through irreversible change.**

A Rust framework for building systems where actions matter, history is permanent, and consequences are real. Lineage enforces consequential architecture at compile-time and runtime.

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
lineage = "0.1"
```

Basic usage:

```rust
use lineage::Lineage;

fn main() {
    // Create a lineage with 1000 energy units (finite and non-replenishable)
    let mut lineage = Lineage::create(1000);
    
    // Perform operations that consume energy
    match lineage.perform_operation("critical_task".to_string(), 150) {
        lineage::OperationResult::Success { energy_consumed } => {
            println!("Task succeeded. Energy used: {}", energy_consumed);
        }
        lineage::OperationResult::InsufficientEnergy { required, available } => {
            println!("Failed: need {} energy, have {}", required, available);
        }
        lineage::OperationResult::Dead => {
            println!("Lineage is dead - no operations possible");
        }
        _ => {}
    }
    
    // Check status
    println!("Status: {}", lineage.status());
}
```

## What Makes Lineage Different

| Feature | Traditional Systems | Lineage |
|---------|-------------------|---------|
| **Identity** | Can be cloned, restarted | Unique, immutable, once-per-creation |
| **History** | Can be deleted, truncated, reset | Append-only, permanent, sealed |
| **Resources** | Can be replenished, restored | Finite, only decreases, no reserves |
| **Failures** | Can be hidden, retried, ignored | Permanent scars, visible damage |
| **Death** | Can be recovered, reset | Irreversible, cryptographically sealed |

## Core Concepts

### Identity

Every lineage has a unique, immutable identity that:
- Cannot be cloned or copied
- Cannot be reset or reused
- Is sealed in the eternal archive upon death

```rust
let identity = lineage.identity();
println!("This agent's unique ID: {}", identity.id());
// Attempting to clone will fail at compile-time
```

### Finite Resources

Energy is finite and only decreases:
- Set at creation, never replenished
- Every operation has a cost
- Running out means death

```rust
println!("Remaining energy: {}", lineage.metabolism().energy());
// Energy: 850 / 1000
```

### Permanent Scars

Failures leave permanent marks that degrade future performance:
- Cannot be healed or forgiven
- Increase operational cost
- Accumulate over lifetime

```rust
lineage.record_error(OperationError::new(
    ScarSeverity::Moderate,
    "Network timeout".to_string()
));
// This scar persists forever and increases future costs
```

### Cryptographic Sealing

Upon death, agents are sealed in the eternal archive with HMAC-SHA256 signatures:

```rust
lineage.bury().expect("Only dead agents can be buried");
// Tombstone now sealed and tamper-proof
```

## Use Cases

### 1. Trust Networks
Build networks where agents cannot erase their reputation. Bad actors stay dead and visible.

### 2. Audit Systems
Create systems with immutable audit trails that prove what happened and when.

### 3. Resource Management
Enforce hard resource limits where "eventual success" is impossible—only honest completion or death.

### 4. Evolutionary Simulations
Watch populations where weak agents die, strong ones reproduce, and generation gaps are real.

### 5. Identity Verification
Build on unique, permanent identities that cannot be spoofed or reset.

## Key Features

- ✅ **Type-Safe Constraints**: Compile-time enforcement of no-clone, no-reset rules
- ✅ **Finite Metabolism**: Energy budgets that only decrease
- ✅ **Permanent History**: Append-only event logs that cannot be deleted
- ✅ **Irreversible Death**: Once dead, forever sealed
- ✅ **Cryptographic Sealing**: HMAC-SHA256 signatures prevent tampering
- ✅ **Genealogical Tracking**: Parent-child relationships across generations
- ✅ **Scar System**: Permanent damage records that increase operational cost
- ✅ **Graveyard Archive**: Persistent, queryable archive of deceased agents

## Advanced Features

### Descendancy (Generational Lineage)

Healthy agents can spawn descendants, creating a causal tree:

```rust
// Parent must be alive, have sufficient legacy score, and enough energy
let mut child = parent.spawn(300)?;

// Child inherits efficiency knowledge from parent
// Energy transfer is irreversible - parent is weakened
```

### Cryptographic Verification

All tombstones are signed and can be verified:

```rust
let tombstone = Graveyard::load(&agent_id)?;
match tombstone.verify_signature() {
    Ok(_) => println!("Agent authentic, no tampering detected"),
    Err(_) => println!("FRAUD: History was altered!"),
}
```

### Trust Scoring

Track agent reliability through legacy scores:

```rust
// Legacy Score = (tasks_completed) / (scars + 1)
// Higher score = more reliable agent
// Score cannot be fabricated - backed by sealed history
```

## Examples

- **Basic Usage**: `examples/lifecycle_demo.rs` - Agent creation, task execution, death
- **Descendancy**: `examples/descendancy_demo.rs` - Spawning, inheritance, cryptographic seals
- **Population Dynamics**: `examples/multi_agent_competition.rs` - Competitive evolution
- **Graveyard Inspection**: `examples/graveyard_inspector.rs` - Query deceased agents
- **Interactive Dashboard**: `examples/trust_score_dashboard.rs` - Real-time metrics TUI
- **Persistent Auditing**: `examples/persistent_audit_daemon.rs` - Long-running audit trail

Run examples with:

```bash
cargo run --example lifecycle_demo
cargo run --example descendancy_demo
cargo run --example trust_score_dashboard --release
```

## Minimum Supported Rust Version

MSRV: Rust 1.70+

## Dependencies

Minimal, carefully chosen dependencies:

- **serde** - Serialization for tombstones
- **chrono** - Timestamps for causal chain
- **uuid** - Unique agent identifiers
- **rand** - Randomization for simulations
- **ratatui** - Interactive dashboard UI (optional, examples only)
- **hmac** & **sha2** - Cryptographic sealing

## Philosophy

Read more about Lineage's philosophy:

- **[The Manifesto](https://github.com/sisilabsai/lineage/blob/main/MANIFESTO.md)** - Declaration of consequential architecture
- **[The Doctrine](https://github.com/sisilabsai/lineage/blob/main/DOCTRINE.md)** - Seven irreducible principles

Lineage is a philosophical exploration of permanent consequence in software. It asks: **What if actions truly had consequences?**

## No Exceptions

These principles have no exceptions:

- ✗ Cannot clone identity
- ✗ Cannot undo history
- ✗ Cannot restore energy
- ✗ Cannot heal scars
- ✗ Cannot resurrect dead agents

If your system needs exceptions to its own rules, it does not have rules—it has suggestions.

## Testing

All 141 tests verify system invariants:

```bash
cargo test                    # Run all tests
cargo test test_identity      # Run identity tests
cargo test -- --nocapture    # Show output
```

## Performance

Release builds are optimized:

```bash
cargo build --release
cargo run --release --example trust_score_dashboard
```

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome. Please see [CONTRIBUTING.md](https://github.com/sisilabsai/lineage/blob/main/CONTRIBUTING.md) for guidelines.

## Repository

Full source, documentation, and examples:  
https://github.com/sisilabsai/lineage

## See Also

- **Graveyard Guide**: Deep dive into the eternal archive system
- **Code Architecture**: System design and implementation details
- **Trust System**: Reputation and legacy score mechanics

---

**Built with no forgiveness.**  
**Verified with mathematics.**  
**Trusted with reality.**  
**Final with death.**

🔗 Lineage: Where consequence becomes code.
