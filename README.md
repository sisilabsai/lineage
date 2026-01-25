# Lineage

Ontological software system enforcing irreversible consequence.

## System Definition

Lineage is a Rust implementation of consequential software where:

- Identity cannot be cloned or copied
- History cannot be erased or rewritten  
- Energy cannot be restored or recharged
- Scars are permanent
- Death is final
- Termination seals all state

These are enforced constraints, not features.

---

## Core Principles

### 1. Identity (Unique and Irreversible)

**IRREVERSIBLE**: Identity is created once at birth via SHA-256 hash of timestamp + entropy.

Identity constraints:

- No Clone trait (compile-time prevention)
- No Copy trait (compile-time prevention)
- No PartialEq trait (identities cannot be compared for equality)
- Cannot be recreated from hash
- Cannot be reassigned

```rust
use lineage::Lineage;

let lineage = Lineage::create(1000);
println!("Identity: {}", lineage.identity().id());

// FORBIDDEN - will not compile:
// let cloned = lineage.clone();
// Identity::from_hash("...");
```

### 2. Memory (Append-Only Causal Chain)

**IRREVERSIBLE**: All operations are recorded in immutable event log.

Memory constraints:

- Events form strict causal chain (sequence → previous)
- Event count is monotonically increasing
- Event sequences are immutable
- Event descriptions are immutable
- Memory can be terminated (sealed permanently)

**TERMINATION**: When lineage dies, memory is terminated. Termination:
- Records TERMINATION event in causal chain
- Sets is_terminated flag
- Prevents all future appends (panics if attempted)
- Is irreversible (cannot terminate twice)

```rust
lineage.perform_operation("Initialize".to_string(), 100);
lineage.perform_operation("Process".to_string(), 200);

for event in lineage.memory().history() {
    println!("[{}] {}", event.sequence(), event.description());
}

// FORBIDDEN - methods do not exist:
// memory.clear()
// memory.delete_event(index)
// memory.rollback()
// memory.modify_event(index, new_description)
```

### 3. Metabolism (Finite Energy)

**IRREVERSIBLE**: Energy is monotonically decreasing.

Metabolism constraints:

- Energy starts at initial value
- Energy can only decrease (no addition methods exist)
- Energy reaches zero → death is mandatory
- Death state is permanent
- consume() accepts only u64 (no negative costs possible)

**DEATH TRIGGER**: When energy reaches zero, lineage:
- Calls metabolism.die()
- Terminates memory (seals event log)
- Returns OperationResult::Dead for all future operations

```rust
use lineage::OperationResult;

let mut lineage = Lineage::create(500);

match lineage.perform_operation("Expensive task".to_string(), 600) {
    OperationResult::InsufficientEnergy { required, available } => {
        println!("Operation rejected: need {}, have {}", required, available);
    }
    _ => {}
}

// FORBIDDEN - methods do not exist:
// metabolism.add_energy(amount)
// metabolism.recharge()
// metabolism.restore()
// metabolism.revive()
// metabolism.set_energy(new_value)
```

### 4. Scars (Permanent Consequences)

**IRREVERSIBLE**: Scars are permanent records of error.

Scar constraints:

- Scar count is monotonically increasing
- Damage score is monotonically increasing  
- Scar descriptions are immutable
- Scar severity is immutable
- Fatal scars cause immediate death and termination

Severity scale: Minor=1, Moderate=5, Severe=20, Fatal=100 (damage score)

**DEATH TRIGGER**: ScarSeverity::Fatal causes:
- metabolism.die()
- Memory termination
- Return of OperationResult::Dead

```rust
use lineage::{OperationError};
use lineage::scar::ScarSeverity;

let error = OperationError::new(
    ScarSeverity::Moderate,
    "Network failure".to_string()
);

lineage.record_error(error);

println!("Scars: {}", lineage.scars().scar_count());
println!("Damage: {}", lineage.scars().damage_score());

// FORBIDDEN - methods do not exist:
// scars.remove(index)
// scars.heal()
// scars.clear()
// scars.reduce_severity()
// scar.set_description(new_desc)
```

### 5. Behavior (Consequential Actions)

**IRREVERSIBLE**: Contract violations inflict permanent consequences.

System includes one behavior: PulseBehavior

Behavior constraints:
- Base cost: 10 energy
- Threshold contract: energy ≥ 30 during pulse
- Contract checked BEFORE pulse consumes energy
- Violation inflicts strain scar (ScarSeverity::Moderate)
- Each strain scar increases cost by +5 permanently
- Pulse cost is monotonically increasing

```rust
use lineage::behavior::PulseBehavior;

let behavior = PulseBehavior::new();
let mut lineage = Lineage::create(1000);

// Contract satisfied
let output = behavior.execute_pulse(&mut lineage);
assert!(output.is_strong);

lineage.perform_operation("Heavy work".to_string(), 975);

// Contract violated (25 < 30)
let output = behavior.execute_pulse(&mut lineage);
assert!(output.strain_occurred);

println!("Cost: {} → {}", 10, behavior.current_pulse_cost(&lineage));  // 10 → 15
```

Strain consequence mechanism:
1. Weak pulse detected (energy < threshold)
2. Strain scar inflicted via record_error()
3. Scar description contains "Pulse strain"
4. count_strain_scars() filters on description
5. Cost = base_cost + (strain_count * 5)

**DEATH SPIRAL**: Accumulated strain → higher cost → lower energy margin → more strain → death

```rust
// FORBIDDEN - methods do not exist:
// behavior.reset_cost()
// behavior.ignore_scars()
// behavior.set_base_cost_only()
```

### 6. Death (Final and Irreversible)

**IRREVERSIBLE**: Death is permanent state transition.

Death triggers:
1. Energy depleted to zero → metabolism.die() → terminate("Energy depleted")
2. Fatal scar inflicted → metabolism.die() → terminate("Fatal scar inflicted")

Death consequences:
- metabolism.is_dead() returns true permanently
- metabolism.energy() returns 0
- memory.terminate() called (seals event log)
- memory.is_terminated() returns true
- All future perform_operation() calls return OperationResult::Dead
- All future record_error() calls return OperationResult::Dead

```rust
lineage.perform_operation("Final task".to_string(), remaining_energy);

assert!(!lineage.is_alive());
assert!(lineage.memory().is_terminated());

match lineage.perform_operation("After death".to_string(), 1) {
    OperationResult::Dead => {
        // Only possible outcome
    }
    _ => unreachable!()
}
```

```rust
// FORBIDDEN - methods do not exist:
// lineage.revive()
// lineage.resurrect()
// metabolism.restore_life()
// memory.unseal()
```

---

## Usage

### Creating a Lineage

```rust
use lineage::Lineage;

// Create with 1000 energy units
let mut lineage = Lineage::create(1000);
```

### Performing Operations

```rust
use lineage::OperationResult;

match lineage.perform_operation("My task".to_string(), 150) {
    OperationResult::Success { energy_consumed } => {
        println!("Success! Used {} energy", energy_consumed);
    }
    OperationResult::InsufficientEnergy { required, available } => {
        println!("Failed: need {}, have {}", required, available);
    }
    OperationResult::Dead => {
        println!("Lineage is dead");
    }
    OperationResult::OntologicalViolation { reason } => {
        eprintln!("FATAL: {}", reason);
        std::process::exit(1);
    }
}
```

### Recording Errors

```rust
use lineage::{OperationError};
use lineage::scar::ScarSeverity;

// Non-fatal error
let error = OperationError::new(
    ScarSeverity::Minor,
    "Cache miss".to_string()
);
lineage.record_error(error);

// Fatal error (causes immediate death)
let fatal = OperationError::new(
    ScarSeverity::Fatal,
    "Unrecoverable corruption".to_string()
);
lineage.record_error(fatal);

assert!(!lineage.is_alive());
```

### Checking Status

```rust
let status = lineage.status();
println!("{}", status);

// Output:
// === Lineage Status ===
// Identity: 9d004d21170cfc19...
// Birth Time: 1769037701430050300
// Status: ALIVE
// Energy: 750/1000 (25.0% consumed)
// Events: 5
// Scars: 2 (damage score: 6)
```

---

## Building and Running

### Build

```bash
cargo build
```

### Run Demonstration

```bash
cargo run
```

This runs a comprehensive demonstration showing:
- Identity creation
- **Consequential behavior loop** (pulse with strain)
- Normal operations
- Scar accumulation
- Energy exhaustion
- Death
- Attempted violations

### Run Tests

```bash
cargo test
```

All 141 tests (70 lib + 70 bin + 1 doc) verify system invariants:
- Identity cannot be cloned
- Memory cannot be cleared
- Energy cannot be restored
- Scars are permanent
- Death is final
- **Behavior contracts cause consequences**

---

## Architectural Guarantees

### Type System Enforcement

Lineage uses Rust's type system to enforce ontological correctness:

- **No Clone trait**: Identity and Lineage cannot be duplicated
- **No Copy trait**: Values cannot be implicitly copied
- **No PartialEq on Identity**: Identities are never "equal"
- **Immutable history**: Events are stored in append-only Vec
- **Monotonic energy**: Only decrement operations exist

### Compile-Time Prevention

Forbidden operations that will not compile:

```rust
let lineage = Lineage::create(1000);

// FORBIDDEN: No Clone trait
let copy = lineage.clone();

// FORBIDDEN: No Copy trait  
let copy = lineage;

// FORBIDDEN: Methods do not exist
lineage.metabolism().add_energy(100);
lineage.metabolism().recharge();
lineage.scars().clear();
lineage.scars().remove(0);
lineage.memory().delete_event(0);
lineage.memory().clear();
Identity::from_hash("...");
```

### Runtime Verification

Invariant checks on every operation:

```rust
pub fn verify_invariants(&self) -> Result<(), String> {
    // Memory causal chain integrity
    if !self.memory.verify_integrity() {
        return Err("Memory corruption: causal chain is broken".to_string());
    }
    
    // Death state consistency  
    if self.metabolism.is_dead() && self.metabolism.energy() != 0 {
        return Err("Metabolism corruption: dead but has energy".to_string());
    }
    
    // Fatal scar consistency
    if self.scars.has_fatal_scars() && !self.metabolism.is_dead() {
        return Err("Scar corruption: fatal scar exists but not dead".to_string());
    }
    
    Ok(())
}
```

**TERMINATION ON VIOLATION**: If verify_invariants() returns Err:
- Operation returns OperationResult::OntologicalViolation
- Caller must terminate process (std::process::exit(1))
- No recovery is possible

---

## Design Constraints

### No Bypass Mechanisms

System provides no:

- Debug modes
- Override flags
- Emergency resets
- Developer bypasses
- Configuration options to disable constraints

Constraints are absolute and non-configurable.

### Incompatible Use Cases

Lineage cannot be used for systems requiring:

- State rollback or undo
- Entity cloning or duplication
- Resource restoration or recharge
- Error recovery or healing
- Death resurrection or reset

These operations are prevented at compile-time and runtime.

---

## System Classification

Lineage is:

- Reference implementation of irreversible consequence
- Demonstration of ontological constraints in software
- Experimental system for research

Lineage is not:

- Production infrastructure
- General-purpose library
- Database or persistence layer
- Logging or error handling framework

---

## Contributing

Before contributing, read [`prompt.md`](prompt.md) - the canonical development constitution.

Any contribution that:

- Introduces Clone or Copy on core types
- Adds energy restoration mechanisms
- Allows history modification
- Enables scar removal
- Provides death bypass

**Will be rejected immediately.**

---

## License

This project is a philosophical exploration. Use it to learn, to think, to build experiments.

If you build something with these ideas, I'd love to hear about it.

---

## System Properties

Lineage enforces:

- Unique identity (cannot clone)
- Permanent memory (cannot erase)  
- Finite resources (cannot restore)
- Lasting consequences (cannot undo)
- Mortal existence (cannot resurrect)
- Sealed termination (cannot unseal)

These constraints are maintained via Rust type system and runtime checks.
