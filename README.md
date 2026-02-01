# Lineage

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-141%20passing-brightgreen.svg)]

**Lineage is a Rust framework for building agents and systems where history cannot be erased, identity cannot be duplicated, and actions have irreversible consequences.**

Five core constraints define every system:
- **Unique Identity** — Never cloned, fully immutable
- **Permanent History** — Append-only, tamper-proof
- **Finite Resources** — Energy only decreases
- **Lasting Consequences** — Scars persist forever
- **Irreversible Death** — No resurrection, no exceptions

Learn more: [MANIFESTO.md](MANIFESTO.md) | [DOCTRINE.md](DOCTRINE.md) | [CODE_ARCHITECTURE.md](CODE_ARCHITECTURE.md)

## Getting Started

### Installation

```bash
git clone https://github.com/sisilabsai/lineage.git
cd lineage
```

Add to `Cargo.toml`:
```toml
[dependencies]
lineage = { path = "path/to/lineage" }
```

### Quick Start

**Build & Run:**
```bash
cargo build --release
cargo run                    # Main demo
cargo test                   # 141 tests
```

**Interactive Examples:**
```bash
cargo run --example trust_score_dashboard --release          # Real-time trust dynamics
cargo run --example descendancy_demo                         # Generational lineage
cargo run --example lifecycle_demo                           # System lifecycle
cargo run --example graveyard_inspector -- --summarize       # Tamper-proof archive
cargo run --example decentralized_trading_agent --release    # AI trading with real consequences
```

### Lineage Finance: Evolutionary Trading Platform

**NEW!** Lineage now includes a complete finance module for building decentralized, autonomous AI trading platforms. Watch agents that **live, trade, evolve, and die**:

```bash
cargo run --example decentralized_trading_agent
```

**Output**: Live demo of evolutionary trading with:
- ✅ Irreversible trades (no reset buttons)
- ✅ Permanent scars from losses
- ✅ Spawning offspring (inheritance)
- ✅ Trust-based access control
- ✅ Multi-agent arena competition
- ✅ Blockchain integration (ready)
- ✅ Evolutionary AI mechanics (ready)
- ✅ Irreversible governance (ready)

**What makes it radical**: Unlike traditional trading bots (infinite retries, backtests forever), Lineage traders have **finite capital, permanent consequences, and evolutionary dynasties**. This is **Darwinian DeFi**—where survival drives intelligence.

**Get Started**: 
- [Finance Quick Start](FINANCE_GETTING_STARTED.md)
- [Implementation Roadmap](FINANCE_IMPLEMENTATION_ROADMAP.md)

### Your First System

```rust
use lineage::Lineage;

let mut lineage = Lineage::create(1000);  // 1000 energy units

// Perform an operation
match lineage.perform_operation("critical task".to_string(), 150) {
    OperationResult::Success { energy_consumed } => {
        println!("Energy remaining: {}", 1000 - energy_consumed);
    }
    OperationResult::InsufficientEnergy { required, available } => {
        println!("Need {} energy, have {}", required, available);
    }
    OperationResult::Dead => println!("System is dead"),
    _ => eprintln!("Error!"),
}

// Record a failure
lineage.record_error(OperationError::new(
    ScarSeverity::Moderate,
    "Network timeout".to_string()
));

// Check status anytime
println!("{}", lineage.status());
```

## Core Features

### Type-Safe Immutability
These operations **don't compile**:
```rust
let copy = lineage.clone();           // ❌ No Clone trait
lineage.metabolism().add_energy(100); // ❌ Doesn't exist
lineage.scars().clear();              // ❌ Impossible
lineage.memory().delete_event(0);     // ❌ Forbidden
```

### Permanent Consequences
Every failure leaves a scar. Scars increase operation costs. Accumulated damage spirals into death.

```rust
lineage.record_error(OperationError::new(
    ScarSeverity::Fatal,  // Causes immediate death
    "Critical failure".to_string()
));

assert!(!lineage.is_alive());  // Permanently dead, no operations possible
```

### Trust Calculation
Built-in trust scoring reflects real performance:
```rust
let trust = lineage.calculate_trust_score();  // Tasks completed / total scars
```

### Genealogical Inheritance
Agents can spawn descendants, passing efficiency metrics across generations:
```bash
cargo run --example descendancy_demo
```

### Tamper-Proof Archive
All dead agents sealed with cryptographic signatures (HMAC-SHA256) in the Graveyard:
```bash
cargo run --example graveyard_inspector -- --verify <AGENT_ID>
```

## Project Structure

```
src/
├── lib.rs               # Public API
├── agent.rs             # TaskAgent type
├── behavior.rs          # PulseBehavior contracts
├── identity.rs          # Unique identification
├── lineage.rs           # Core system
├── memory.rs            # Append-only log
├── metabolism.rs        # Energy & death
├── scar.rs              # Permanent consequences
├── trust.rs             # Trust calculations
└── finance/             # 🆕 EVOLUTIONARY TRADING PLATFORM
    ├── mod.rs           # Module configuration
    ├── agent.rs         # FinanceAgent lifecycle
    ├── trade.rs         # Irreversible trade execution
    ├── scars.rs         # Financial damage system
    ├── spawning.rs      # Offspring inheritance
    ├── trust_scoring.rs # Cryptographic trust
    ├── arena.rs         # Multi-agent competition
    └── advanced.rs      # Blockchain, evolution, governance

examples/               # 13 interactive demos (including decentralized_trading_agent)
tests/                  # 141 comprehensive tests
```

## System Guarantees

| Constraint | Enforced By |
|-----------|-------------|
| Unique Identity | SHA-256 hash, no Clone trait |
| Permanent Memory | Append-only Vec, no delete method |
| Finite Energy | consume() only, never increases |
| Lasting Scars | Monotonic damage score |
| Irreversible Death | State flag prevents all operations |

## What's Impossible (By Design)

- ❌ Undo/rollback operations
- ❌ Clone or duplicate agents
- ❌ Restore or add energy
- ❌ Remove or heal scars
- ❌ Resurrect dead systems
- ❌ Override constraints

If your use case needs these, Lineage isn't the right tool.

## Testing

Run the full test suite:
```bash
cargo test                    # All 141 tests
cargo test test_identity      # Specific category
cargo test -- --nocapture    # With output
```

## Contributing

Before contributing, understand our values:
1. Read [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)
2. Understand [CODE_ARCHITECTURE.md](CODE_ARCHITECTURE.md)
3. Review [CONTRIBUTING.md](CONTRIBUTING.md)

**Rejected contributions:**
- Any Clone/Copy additions
- Energy restoration features
- History modification capabilities
- Scar removal methods
- Death bypass mechanisms

**How to contribute:**
```bash
git checkout -b feature/my-feature
cargo test
git commit -m "Add: [description]"
git push origin feature/my-feature
```

## Documentation

| Document | Purpose |
|----------|---------|
| [DOCTRINE.md](DOCTRINE.md) | Core principles |
| [CODE_ARCHITECTURE.md](CODE_ARCHITECTURE.md) | System design |
| [EXTENSION_PROTOCOL.md](EXTENSION_PROTOCOL.md) | Extension guide |
| [TRUST_SYSTEM.md](TRUST_SYSTEM.md) | Trust scoring |
| [GRAVEYARD_GUIDE.md](GRAVEYARD_GUIDE.md) | Archive system |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Contribution rules |

## Examples

### 13 Interactive Demonstrations

**Core Systems:**
- `cargo run` — Main showcase
- `trust_score_dashboard` — Real-time trust UI
- `lifecycle_demo` — Full agent lifecycle

**Finance & Trading (NEW):**
- `decentralized_trading_agent` — Evolutionary AI trading with irreversible consequences

**Advanced:**
- `descendancy_demo` — Generational inheritance
- `graveyard_inspector` — Tamper-proof archive analysis
- `multi_agent_competition` — Population dynamics
- `ghost_in_the_machine` — Death mechanics
- `persistent_audit_daemon` — Audit trail
- `permadeath_adventurers` — Consequence spiral
- `ethical_decision_wrapper` — Contract enforcement
- `interactive_consensus_arena` — Distributed voting
- `mortality` — Lifespan exploration

## Support

- **Questions**: [Discussions](https://github.com/sisilabsai/lineage/discussions)
- **Bug Reports**: [Issues](https://github.com/sisilabsai/lineage/issues)
- **Philosophy**: [MANIFESTO.md](MANIFESTO.md)
- **Doctrine**: [DOCTRINE.md](DOCTRINE.md)

## License

MIT License — See [LICENSE](LICENSE) for details

---

**Created by**: Wilson Ecaat at [Sisi Labs](https://github.com/sisilabsai)  
**Status**: Production Ready  
**Last Updated**: January 31, 2026
