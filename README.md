# 🔗 Lineage

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-production%20ready-brightgreen.svg)]
[![Tests](https://img.shields.io/badge/tests-141%20passing-brightgreen.svg)]

> **Software identity preserved through irreversible change**
>
> An ontological software system enforcing permanent consequence. Identity cannot be cloned, history cannot be erased, energy cannot be restored, scars are permanent, and death is final.

**👉 Read the full vision: [The Lineage Manifesto](MANIFESTO.md)**

---

## 🎯 What is Lineage?

Lineage is a Rust framework implementing **consequential software architecture** where every system has:

- **Unique Identity**: Created once, never duplicated or rewritten
- **Permanent History**: Append-only event log that cannot be erased
- **Finite Resources**: Energy that only decreases, never increases
- **Lasting Scars**: Permanent records of failure and consequence
- **Mortal Existence**: Death is irreversible and seals all state

These are **enforced constraints**, not configurable features.

### Why Lineage Matters

Traditional software treats state as malleable - we can undo, revert, and reset. Lineage takes the opposite approach: **consequences are permanent**. This creates systems where:

- Agents develop real reputation based on actual performance
- Trust scores reflect history, not fabrication
- Decisions cannot be erased or rewritten
- Systems learn from mistakes through permanent scars
- Networks converge on reliable entities

---

## 📚 Core Concepts at a Glance

| Concept | What It Does | Philosophy |
|---------|-------------|-----------|
| **Identity** | Unique, immutable identifier | You cannot be cloned |
| **Memory** | Append-only event log | Your history is permanent |
| **Metabolism** | Finite energy budget | Resources have limits |
| **Scars** | Permanent damage records | Mistakes leave marks |
| **Death** | Irreversible termination | Existence ends |
| **Behavior** | Consequential contracts | Actions have consequences |

---

## 🚀 Quick Start

### 1️⃣ Installation

Clone the repository:

```bash
git clone https://github.com/sisilabsai/lineage.git
cd lineage
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
lineage = { path = "path/to/lineage" }
```

### 2️⃣ Build

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### 3️⃣ Run Examples

```bash
# Main demonstration (shows all system features)
cargo run

# Descendancy & Generational Lineage
cargo run --example descendancy_demo        # Spawn, inheritance, and cryptographic seals

# Trust Score Dashboard - Interactive TUI with 5 agents
cargo run --example trust_score_dashboard --release

# Graveyard System - The Eternal Archive of Dead Agents
cargo run --example ghost_in_the_machine          # Single agent death and burial
cargo run --example multi_agent_competition       # Population dynamics + auto-burial
cargo run --example graveyard_inspector -- --summarize   # Analyze dead agents
cargo run --example graveyard_inspector -- --verify <ID> # Check cryptographic signatures
cargo run --example graveyard_inspector -- --darwinian   # Find evolutionary winners

# Run archaeologist scenario (5 generations, ~50 dead agents)
.\archaeologist.ps1                # Windows
bash archaeologist.sh              # Linux/Mac

# Other fascinating examples
cargo run --example lifecycle_demo
cargo run --example persistent_audit_daemon
cargo run --example permadeath_adventurers
```

### 4️⃣ Run Tests

```bash
# All 141 tests
cargo test

# Specific test
cargo test test_identity_cannot_clone

# With output
cargo test -- --nocapture
```

---

## 📖 Graveyard System (The Eternal Archive)

The Graveyard is a persistent, tamper-proof archive of dead agents. Every deceased agent receives an immutable tombstone containing:

- Complete identity and metabolic record
- All scars with timestamps and severity
- Cryptographic proof against tampering
- Legacy score calculation (tasks/scars ratio)

**See:** [GRAVEYARD_GUIDE.md](GRAVEYARD_GUIDE.md) and [GRAVEYARD_TESTING.md](GRAVEYARD_TESTING.md)

### Quick Graveyard Test
```bash
cargo run --example ghost_in_the_machine
cargo run --example graveyard_inspector -- --summarize
```

---

## 🧬 Descendancy & Generational Lineage

Healthy agents can now spawn descendants, creating a causal tree across generations. This implements evolutionary pressure and inheritance of efficiency metrics.

### The `spawn()` Method

**Requirements for spawning:**
- Parent must be ALIVE (not dead)
- Parent must have legacy score ≥ 0.5
- Parent must have sufficient energy for transfer (child_energy + 50)
- Parent should have completed 5+ tasks successfully

**Energy Transfer Mechanics:**
- Parent irreversibly loses transferred energy
- Child inherits with efficiency knowledge from parent
- Genealogical lineage permanently recorded
- Child gains genetic advantage but at higher risk

**Example:**
```bash
cargo run --example descendancy_demo
```

This demonstrates:
- Part 1: Parent executes 8 tasks, builds legacy score 8.00
- Part 2: Parent spawns child with 300 energy transfer
- Part 3: Both agents buried with cryptographic seals
- Part 4: Signatures verified - no tampering detected

### Genealogical Records

Children record their parent's ID in their memory and inheritance details. The Graveyard tracks parent-child relationships, enabling evolutionary analysis:

```bash
cargo run --example graveyard_inspector -- --autopsy <CHILD_ID>
```

Shows genealogical records and inheritance metrics.

---

## 🔐 Cryptographic Seals (Tamper Detection)

All tombstones are now signed with **HMAC-SHA256**, preventing fraudulent modifications. If someone manually edits a JSON `.tomb` file to increase their Legacy Score, the signature will fail verification.

### What's Protected

The cryptographic seal covers:
- Agent identity and creation time
- All metabolic records (energy, efficiency, tasks)
- Every scar with timestamp and severity
- Cause of death and burial timestamp
- Complete causal chain

### Signature Verification

```bash
# Verify a specific agent's signature
cargo run --example graveyard_inspector -- --verify <AGENT_ID>

# Summarize archive (includes signature status)
cargo run --example graveyard_inspector -- --summarize
```

**Output Example:**
```
✓ Parent's cryptographic signature verified
  Status: No tampering detected
✓ Child's cryptographic signature verified
  Status: No tampering detected
  Genealogy: Descended from parent agent
```

### Fraudulent History Detection

If a Legacy Score or other critical field is tampered with:
```
✗ Signature verification failed: FRAUDULENT HISTORY DETECTED
```

This prevents agents from falsifying their records in the permanent archive.

---

## 💡 Basic Usage

### Creating a Lineage

```rust
use lineage::Lineage;

let mut lineage = Lineage::create(1000);  // 1000 energy units
```

### Performing Operations

```rust
use lineage::OperationResult;

match lineage.perform_operation("My task".to_string(), 150) {
    OperationResult::Success { energy_consumed } => {
        println!("Success! Used {} energy", energy_consumed);
    }
    OperationResult::InsufficientEnergy { required, available } => {
        println!("Not enough energy: need {}, have {}", required, available);
    }
    OperationResult::Dead => {
        println!("Lineage is dead - no more operations possible");
    }
    OperationResult::OntologicalViolation { reason } => {
        eprintln!("FATAL: {}", reason);
        std::process::exit(1);
    }
}
```

### Recording Errors

```rust
use lineage::OperationError;
use lineage::scar::ScarSeverity;

// Non-fatal error inflicts a scar
let error = OperationError::new(
    ScarSeverity::Moderate,
    "Network failure detected".to_string()
);
lineage.record_error(error);

// Fatal error causes immediate death
let fatal = OperationError::new(
    ScarSeverity::Fatal,
    "Unrecoverable corruption".to_string()
);
lineage.record_error(fatal);  // System dies here

assert!(!lineage.is_alive());  // Permanently dead
```

### Checking Status

```rust
let status = lineage.status();
println!("{}", status);

/* Output:
=== Lineage Status ===
Identity: 9d004d21170cfc19a5f8c7d2e1b6a3f9
Birth Time: 1769037701430050300
Status: ALIVE
Energy: 750/1000 (75% remaining)
Events: 5
Scars: 2 (damage score: 6)
*/
```

---

## 🎮 Featured Example: Trust Score Dashboard

An interactive terminal UI showing real-time trust dynamics:

```bash
cargo run --example trust_score_dashboard --release
```

**What You'll See:**
- 5 AI agents (ARIA, NEXUS, ATLAS, SAGE, PRISM) making decisions
- Projects with varying risk levels
- Real-time trust score updates
- Scar accumulation from mistakes
- Three interactive views (Dashboard, Agent Profiles, History)

**Controls:**
- `SPACE` - Execute next project
- `←/→` - Switch between views
- `Q` - Exit

[📖 Full Dashboard Documentation](examples/TRUST_SCORE_DASHBOARD_README.md)

---

## 📖 System Architecture

### Module Structure

```
src/
├── lib.rs              # Public API
├── agent.rs            # TaskAgent - decision-making entity
├── behavior.rs         # PulseBehavior - consequential contracts
├── identity.rs         # Unique identification system
├── lineage.rs          # Core Lineage struct
├── memory.rs           # Append-only event log
├── metabolism.rs       # Energy and death mechanics
├── scar.rs             # Permanent consequences
└── trust.rs            # Trust score calculations

examples/               # 8 interactive demonstrations
├── trust_score_dashboard.rs
├── lifecycle_demo.rs
├── multi_agent_competition.rs
└── ...

tests/                  # 141 comprehensive tests
```

### Key Types

**OperationResult** - Outcome of any operation:
- `Success { energy_consumed }`
- `InsufficientEnergy { required, available }`
- `Dead` - System is dead
- `OntologicalViolation { reason }` - Invariant broken

**ScarSeverity** - Impact levels:
- `Minor` (damage: 1)
- `Moderate` (damage: 5)
- `Severe` (damage: 20)
- `Fatal` (damage: 100) → immediate death

---

## ✨ Key Features

### 🔒 Type-Safe Constraints

These operations **will not compile**:

```rust
let copy = lineage.clone();              // ❌ No Clone trait
let copy2 = lineage;                     // ❌ No Copy trait
lineage.metabolism().add_energy(100);    // ❌ Method doesn't exist
lineage.scars().clear();                 // ❌ Method doesn't exist
lineage.memory().delete_event(0);        // ❌ Method doesn't exist
```

### ⏱️ Runtime Verification

Every operation verifies invariants:
- Memory causal chain integrity
- Death state consistency
- Fatal scar enforcement
- Energy monotonicity

### 📊 Real-Time Metrics

Track system health at any time:

```rust
println!("Energy: {}/{}", lineage.metabolism().energy(), 1000);
println!("Scars: {}", lineage.scars().scar_count());
println!("Damage: {}", lineage.scars().damage_score());
println!("Events: {}", lineage.memory().event_count());
```

### 🎯 Consequential Behaviors

The `PulseBehavior` demonstrates contracts with consequences:
- **Contract**: Energy must be ≥ 30 before pulse
- **Violation**: Weak pulses inflict strain scars
- **Escalation**: Each scar increases future cost
- **Spiral**: Accumulated strain → higher cost → lower energy → more strain

---

## 🧪 Testing

All 141 tests verify system invariants:

```bash
cargo test                           # Run all tests
cargo test test_identity             # Run identity tests
cargo test -- --nocapture           # Show output
cargo test -- --test-threads=1      # Single-threaded
```

**Test Categories:**
- Identity: Cannot clone or copy
- Memory: Append-only integrity
- Metabolism: Energy mechanics
- Scars: Permanent consequences
- Death: Irreversibility
- Behavior: Contract enforcement

---

## 📚 Documentation

| Document | Purpose |
|----------|---------|
| [**DOCTRINE.md**](DOCTRINE.md) | Philosophical foundations |
| [**CODE_ARCHITECTURE.md**](CODE_ARCHITECTURE.md) | System design details |
| [**EXTENSION_PROTOCOL.md**](EXTENSION_PROTOCOL.md) | How to extend |
| [**TRUST_SYSTEM.md**](TRUST_SYSTEM.md) | Trust calculations |
| [**examples/TRUST_SCORE_DASHBOARD_README.md**](examples/TRUST_SCORE_DASHBOARD_README.md) | Dashboard guide |
| [**CONTRIBUTING.md**](CONTRIBUTING.md) | Contribution guidelines |

---

## 🏗️ System Guarantees

Lineage enforces these properties through **compile-time** and **runtime** mechanisms:

| Property | Guarantee |
|----------|-----------|
| **Unique Identity** | SHA-256 hash, no Clone, survives lifetime |
| **Permanent Memory** | Append-only Vec, no delete/clear/rollback |
| **Finite Energy** | Only `consume()` exists, never increases |
| **Lasting Scars** | Monotonically increasing, cannot remove |
| **Mortal Death** | Irreversible state, seals all operations |
| **Sealed Termination** | `terminate()` prevents future appends |

---

## 🚫 What Lineage Cannot Do

These are **intentionally impossible**:

- ❌ Undo or rollback operations
- ❌ Clone or duplicate entities
- ❌ Restore or recharge energy
- ❌ Heal or remove scars
- ❌ Resurrect or revive dead systems
- ❌ Override constraints with debug modes
- ❌ Reconfigure system constraints

This is **by design**. If your use case needs these, Lineage is not the right tool.

---

## 🎓 Learning Path

### 👶 Beginner (30 minutes)

1. Read this README overview
2. Review [DOCTRINE.md](DOCTRINE.md)
3. Run: `cargo run`
4. Study [Core Concepts](#-core-concepts-at-a-glance)

### 👨‍💼 Intermediate (2-3 hours)

1. Study [CODE_ARCHITECTURE.md](CODE_ARCHITECTURE.md)
2. Run examples:
   - `cargo run --example lifecycle_demo`
   - `cargo run --example persistent_audit_daemon`
3. Read source files: `src/agent.rs`, `src/trust.rs`
4. Run and examine tests: `cargo test`

### 🔬 Advanced (4+ hours)

1. Review [EXTENSION_PROTOCOL.md](EXTENSION_PROTOCOL.md)
2. Study behavior implementation
3. Analyze trust calculations
4. Run interactive dashboard
5. Create custom implementations

---

## 🤝 Contributing

### This is a Serious Project

Before contributing, you must understand and agree with our values:

1. **Read [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)** — Our community standards (required)
2. **Read [CONTRIBUTING.md](CONTRIBUTING.md)** — How to contribute (required)
3. **Understand [CODE_ARCHITECTURE.md](CODE_ARCHITECTURE.md)** — System design (required)

We are looking for collaborators who believe in **consequence, permanence, and integrity**. If you do, welcome.

### What Will Be Rejected

**Any contribution that:**

- Adds Clone or Copy to core types
- Introduces energy restoration
- Allows history modification
- Enables scar removal
- Provides death bypass
- Bypasses constraints

### How to Contribute

```bash
# 1. Create feature branch
git checkout -b feature/my-feature

# 2. Make changes
cargo test

# 3. Commit with clear messages
git commit -m "Add: [description]"

# 4. Push and create PR
git push origin feature/my-feature
```

---

## 📊 Project Status

- ✅ **Core System**: Complete and battle-tested
- ✅ **Documentation**: Comprehensive and thorough
- ✅ **Examples**: 12 interactive demonstrations
- ✅ **Tests**: 141 passing tests (100% core coverage)
- ✅ **Performance**: Optimized release builds
- ✅ **Production Ready**: Yes, fully functional

### Statistics

- **Lines of Code**: ~8,400+ (4,740 core + 3,625 examples)
- **Test Count**: 141
- **Documentation Files**: 9+
- **Example Programs**: 12
- **Dependencies**: Minimal (serde, rand, uuid, chrono, ratatui)

---

## 💬 Support & Community

- **� Philosophy**: [Read the Manifesto](MANIFESTO.md) - Our declaration of consequential architecture
- **📚 Doctrine**: [Read the Doctrine](DOCTRINE.md) - Seven irreducible principles
- **�🐛 Bug Reports**: [Open Issue](https://github.com/sisilabsai/lineage/issues/new)
- **💡 Feature Requests**: [Start Discussion](https://github.com/sisilabsai/lineage/discussions/new)
- **❓ Questions**: [Ask in Discussions](https://github.com/sisilabsai/lineage/discussions)
- **📣 Share Your Work**: [Use Discussions](https://github.com/sisilabsai/lineage/discussions)

---

## 📜 License

This project is released under the **MIT License**.

Read full license: [LICENSE](LICENSE)

### Philosophy

Lineage is a philosophical exploration of permanent consequence in software. Use it to:

- ✓ Learn about ontological constraints
- ✓ Understand irreversible systems
- ✓ Build trust-based networks
- ✓ Explore agent accountability
- ✓ Research consequential AI
- ✓ Teach decision permanence

If you build something with these ideas, **please share it**! 🙏

---

## 🔗 Resources

- **[Rust Book](https://doc.rust-lang.org/book/)** - Learn Rust
- **[Type System Docs](https://doc.rust-lang.org/reference/types.html)** - Type safety
- **[Multi-Agent Systems](https://en.wikipedia.org/wiki/Multi-agent_system)** - Theory
- **[AI Accountability](https://arxiv.org/abs/2301.04819)** - Research

---

## 🌟 Highlights

### What Makes Lineage Unique

1. **Compile-Time Prevention** - Impossible operations don't compile
2. **Type-System Enforcement** - Constraints baked into types
3. **Runtime Verification** - Every operation verifies invariants
4. **Consequential Design** - Contracts cause real damage
5. **Agent Accountability** - Trust reflects actual performance

### Real-World Applications

- **Multi-Agent Networks**: Track agent reliability over time
- **Governance Systems**: Enforce decision accountability
- **Audit Systems**: Immutable permanent audit trails
- **Trust Networks**: Dynamic trust based on consequences
- **Educational Tool**: Teach consequence and responsibility

---

## 🎉 Getting Started Now

```bash
# 1. Clone repository
git clone https://github.com/sisilabsai/lineage.git
cd lineage

# 2. Build project
cargo build --release

# 3. See it in action
cargo run --example trust_score_dashboard --release

# 4. Run tests
cargo test

# 5. Explore examples
cargo run --example lifecycle_demo
```

---

## 🙏 Acknowledgments

- **Created by**: Wilson Ecaat, Founder and Lead Developer at [Sisi Labs](https://github.com/sisilabsai)
- Built in **Rust** for type safety and reliability
- Inspired by **ontological systems** and permanent consequence
- Designed for **philosophical exploration** of software identity
- Created for **researchers, builders, and thinkers**

---

**Created**: January 29, 2026  
**Status**: ✅ Production Ready  
**Maintained by**: [Sisi Labs](https://github.com/sisilabsai) - Wilson Ecaat  
**Community**: Welcome

---

### 🚀 Ready to explore permanent consequence?

[⭐ Star this project](https://github.com/sisilabsai/lineage) • [👥 Join discussions](https://github.com/sisilabsai/lineage/discussions) • [📖 Read docs](DOCTRINE.md)
