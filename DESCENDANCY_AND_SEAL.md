# 🧬 The Final Seal: Descendancy & Cryptographic Integrity

**Date**: January 30, 2026  
**Status**: ✅ COMPLETE

## Overview

This document describes the implementation of the two "High-Limit" features that complete the Lineage foundation:

1. **Descendancy (The `spawn()` Method)** - Allows healthy agents to create descendants and pass resources across generations
2. **The Cryptographic Seal** - Ensures tomb files are signed and detects fraudulent edits to Legacy Scores

---

## Part 1: Descendancy - The spawn() Method

### 🧬 What It Does

The `spawn()` method allows a "healthy" parent agent to create a descendant agent, establishing a **Causal Tree across generations**.

```rust
pub fn spawn(&mut self, initial_energy_for_child: u64) -> Result<TaskAgent, String>
```

### 📋 Requirements for Spawning

A parent agent can only spawn if ALL of these conditions are met:

1. **Parent must be ALIVE** - Cannot spawn from a dead agent
2. **High Legacy Score** - Legacy Score ≥ 0.5 (or 5+ completed tasks)
3. **Sufficient Energy** - Must have `child_energy + 50` to transfer and overhead
4. **No Capacity Degradation** - Parent must be reasonably healthy (capacity > 0)

### 💾 Energy Transfer Mechanics

When spawning succeeds:

1. **Irreversible Deduction** - Parent loses energy permanently:
   ```
   parent.energy() -= initial_energy_for_child
   ```

2. **Child Receives Full Budget** - Child starts with:
   ```
   child.energy() = initial_energy_for_child
   ```

3. **Genealogical Recording** - Child's memory contains:
   - Parent's ID
   - Efficiency ratio inherited
   - Generation number

### 🎯 Biological Analogy

```
Generation 0: Origin Agent (created with Lineage::create)
    ↓
    └─→ Generation 1: Spawned Child (created via parent.spawn())
        ├─→ Generation 2: Grandchild (created via child.spawn())
        └─→ Generation 2: Sibling (another child from same parent)
```

### 📊 Example

```rust
// Create a healthy parent
let mut parent = TaskAgent::create(1000);

// Parent completes 8 tasks
for i in 1..=8 {
    let task = Task::new(format!("Task {}", i), 50);
    parent.execute_task(task, TaskOutcome::Success);
}

// Parent has:
// - 600 energy remaining (1000 - 8*50)
// - 8 completed tasks
// - 0 failures
// - Legacy Score: ~8.0 (very healthy)

// Spawn a child with 300 energy
let child = parent.spawn(300)?;

// After spawn:
// - parent.energy() = 300 (600 - 300)
// - child.energy() = 300
// - Parent permanently weakened
// - Child inherits genealogy
```

### ⚠️ Consequences

- **Irreversible**: Energy transfer cannot be undone
- **Weakness**: Parent becomes weaker after spawning
- **Higher Risk**: Child starts with advantage but faces higher expectations
- **Generations**: Creates a permanent lineage tree

---

## Part 2: The Cryptographic Seal

### 🔐 What It Does

The cryptographic seal ensures that `.tomb` files cannot be tampered with without detection. If someone tries to edit a Legacy Score or other data manually, the signature will be invalid and the fraud will be detected.

### 🛠️ Implementation Details

#### Signature Calculation

Each tombstone has an HMAC-SHA256 signature computed from:

```rust
signature = HMAC-SHA256(key, data)

where data = format!(
    "{}|{}|{}|{}|{}|{}",
    agent_id,
    tasks_completed,
    efficiency_ratio,
    scar_count,
    merkle_root,
    burial_timestamp
)

and key = contents of .lineage/keys/tombstone.key
```

#### Key Management

- **Storage**: `.lineage/keys/tombstone.key`
- **Generation**: Auto-generated from system entropy on first use
- **Persistence**: Same key used for all signatures in a graveyard
- **Security**: File marked read-only (mode 0o600 on Unix)

#### Signature Storage

Tombstone JSON now includes:

```json
{
  "identity": { ... },
  "metabolism": { ... },
  "pathology": { ... },
  "causal_chain": { ... },
  "parentage": {
    "parent_id": "optional_parent_id",
    "inherited_capacity": null,
    "inherited_knowledge": null,
    "generation": 0
  },
  "signature": "a1b2c3d4e5f6...",
  "burial_timestamp": "2026-01-30T...",
  "schema_version": 1
}
```

### 🔍 Verification Process

The `verify_signature()` method checks:

1. **Signature exists** - Must be present in tombstone
2. **Signature is valid** - HMAC matches recomputed signature
3. **Core fields unchanged** - Tasks, efficiency, scars haven't been edited
4. **Metadata intact** - Legacy Score and other derived fields match

### 🚨 Fraud Detection

The Graveyard Inspector now detects:

1. **Missing Signatures** - `[FRAUD]` if signature field is empty
2. **Invalid Signatures** - `[FRAUD]` if HMAC doesn't match
3. **Impossible Values** - `[FRAUD]` if efficiency < 0 or > 100
4. **Suspicious Patterns** - `[WARN]` if zero tasks but many scars

### 📋 Example Fraud Scenarios

**Scenario 1: Editing Legacy Score**
```bash
# Attacker: Edit .tomb file to change tasks_completed from 5 to 50
$ vim .lineage/graveyard/abc123.tomb
# Legacy Score jumps from 1.67 to 16.67

# Inspector: Detects mismatch
$ cargo run --example graveyard_inspector -- --verify abc123
❌ FRAUD DETECTED: Signature mismatch
```

**Scenario 2: Editing Efficiency**
```bash
# Attacker: Change efficiency_ratio from 0.8 to 0.99
# Inspector catches: Efficiency outside normal range
```

**Scenario 3: Perfect Data Edit (Defeated by Signature)**
```bash
# Attacker: Edit ALL core fields to be consistent
# Inspector still catches: HMAC signature is different
# Only way to forge is to have the key (.lineage/keys/tombstone.key)
```

---

## Part 3: Genealogical Record (ParentageRecord)

### 📊 Structure

```rust
pub struct ParentageRecord {
    pub parent_id: Option<String>,
    pub inherited_capacity: Option<u64>,
    pub inherited_knowledge: Option<String>,
    pub generation: u32,
}
```

### 🧬 Generation Tracking

```
Generation 0: Agents created with Lineage::create()
Generation 1: Agents spawned from Generation 0 agents
Generation 2: Agents spawned from Generation 1 agents
...
```

### 🔗 Causal Chain

Genealogical information enables:

- **Lineage Trees**: Trace descendants of successful agents
- **Evolution Analysis**: Compare efficiency across generations
- **Success Patterns**: Identify which parent traits lead to success
- **Orphan Detection**: Flag agents with missing parents in graveyard

---

## Usage Examples

### Example 1: Basic Spawning

```rust
use lineage::agent::{TaskAgent, Task, TaskOutcome};

// Create parent
let mut parent = TaskAgent::create(1000);

// Make parent healthy through tasks
for i in 1..=10 {
    let task = Task::new(format!("Task {}", i), 50);
    parent.execute_task(task, TaskOutcome::Success);
}

// Spawn child
let mut child = parent.spawn(400)?;
println!("Child created with {} energy", child.energy());
```

### Example 2: Verify Signatures

```rust
use lineage::graveyard::Graveyard;

// Load tombstone
let tombstone = Graveyard::load("agent_id")?;

// Verify signature
match tombstone.verify_signature() {
    Ok(_) => println!("✓ No tampering detected"),
    Err(e) => println!("✗ FRAUD: {}", e),
}
```

### Example 3: Run Full Demo

```bash
cargo run --example descendancy_demo
```

This demonstrates:
- Creating a healthy parent
- Spawning a descendant
- Burying both with cryptographic seals
- Verifying signatures

### Example 4: Inspect Graveyard

```bash
# List all agents with genealogy
cargo run --example graveyard_inspector -- --summarize

# Detailed timeline with genealogy
cargo run --example graveyard_inspector -- --autopsy <AGENT_ID>

# Verify signatures and detect fraud
cargo run --example graveyard_inspector -- --verify <AGENT_ID>
```

---

## Architecture Changes

### Modified Files

1. **src/graveyard.rs**
   - Added `ParentageRecord` struct
   - Added `signature` field to `Tombstone`
   - Added `calculate_signature()` method
   - Added `verify_signature()` method
   - Updated `create_with_parentage()` method
   - Updated `verify()` to check signatures

2. **src/agent.rs**
   - Added `spawn()` method
   - Updated `bury()` to support genealogy

3. **src/lineage.rs**
   - Added `memory_mut()` method for child to record parentage

4. **examples/graveyard_inspector.rs**
   - Enhanced `verify_tombstone()` with signature checking
   - Added fraud detection logic
   - Added genealogical information display

5. **Cargo.toml**
   - Added `hmac = "0.12"`
   - Added `hex = "0.4"`

### New Files

- **examples/descendancy_demo.rs** - Complete demonstration of features

---

## Testing Recommendations

### Unit Tests

```rust
#[test]
fn test_spawn_requires_healthy_parent() {
    let mut parent = TaskAgent::create(100);
    // Parent not healthy enough
    assert!(parent.spawn(50).is_err());
}

#[test]
fn test_spawn_transfers_energy() {
    let mut parent = TaskAgent::create(1000);
    // Make healthy...
    let child = parent.spawn(300).unwrap();
    assert_eq!(child.energy(), 300);
    assert_eq!(parent.energy(), 700); // 1000 - 300
}

#[test]
fn test_signature_verification() {
    let tombstone = create_test_tombstone();
    assert!(tombstone.verify_signature().is_ok());
}

#[test]
fn test_tampered_signature_detected() {
    let mut tombstone = create_test_tombstone();
    // Tamper with data
    tombstone.metabolism.tasks_completed = 999;
    // Signature should no longer match
    assert!(tombstone.verify_signature().is_err());
}
```

### Integration Tests

1. **Create, spawn, bury, verify workflow**
2. **Multi-generation lineage (grandchildren)**
3. **Orphan detection (parent missing from graveyard)**
4. **Signature verification on loaded tombstones**

### Manual Testing

```bash
# Test 1: Basic spawn
cargo run --example descendancy_demo

# Test 2: Verify genealogy recorded
cargo run --example graveyard_inspector -- --autopsy <CHILD_ID>
# Should show parent_id in genealogical record

# Test 3: Attempt manual fraud (WILL FAIL)
# Edit .lineage/graveyard/xxx.tomb manually
# Change "tasks_completed" from 5 to 50
cargo run --example graveyard_inspector -- --verify xxx
# Output: ❌ FRAUD DETECTED: Signature mismatch
```

---

## Performance Characteristics

### Time Complexity

- **spawn()**: O(1) - just energy transfer
- **calculate_signature()**: O(1) - fixed fields hashed
- **verify_signature()**: O(1) - recompute and compare

### Space Complexity

- **ParentageRecord**: ~100 bytes per agent
- **signature field**: ~64 bytes (hex-encoded hash)
- **Total overhead**: ~164 bytes per tombstone

### Key Management

- **Key generation**: One-time, ~microseconds
- **Key storage**: Single file on disk (~32 bytes)
- **Key loading**: File I/O, happens once at graveyard initialization

---

## Security Considerations

### Threats & Mitigations

| Threat | Mitigation |
|--------|-----------|
| Tampering with Legacy Score | HMAC signature detects changes |
| Forging signatures | Requires .lineage/keys/tombstone.key |
| Replacing entire tombstone | Graveyard prevents overwrites |
| Deleting tombstones | OS-level read-only protection |
| Modifying tomb files offline | Signature verification catches it |

### Trust Model

- **Graveyard as Source of Truth**: Tombstones are immutable once sealed
- **Key as Root Secret**: Signature key (.lineage/keys/tombstone.key) is critical
- **Genealogy as Proof**: Parent-child relationships provide provenance chain
- **Causal Chain + Signature**: Double protection (merkle root + HMAC)

### Limitations

- **Single Key**: All agents in graveyard use same signing key
  - *Mitigation*: Key stored securely, updated when graveyard reinitialized
- **No Revocation**: Can't revoke signature after burial
  - *Mitigation*: By design - tombstones are permanent
- **No Timestamp**: Signature doesn't include precise creation time
  - *Mitigation*: burial_timestamp stored separately and covered by signature

---

## Future Extensions

### Potential Enhancements

1. **Multi-Key Signature** - Different keys per generation for stronger provenance
2. **Cryptographic Proof** - ZK proof of valid descent (no relation to parent edits)
3. **Signature Chain** - Chain of signatures across genealogical tree
4. **Key Rotation** - Periodic key changes with forward secrecy
5. **Revocation List** - List of invalidated agents (compromised)

### Alternative Approaches Considered

1. **RSA Signatures**: More complex, slower, key management harder
2. **Ed25519**: Better security model, but added complexity
3. **Simple SHA256**: Not sufficient - provides integrity but not authenticity
4. **Database Enforcement**: Would require external dependencies

---

## Conclusion

The implementation of **Descendancy** and **The Cryptographic Seal** completes the Lineage foundation with:

✅ **Genealogical Trees** - Agents can now reproduce and create lineages  
✅ **Irreversible Transfer** - Resources pass between generations permanently  
✅ **Fraud Detection** - Signatures prevent tampering with Legacy Scores  
✅ **Permanent Records** - Tombstones sealed and verified cryptographically  

The system now enforces **both** biological realism (finite lifespans, reproduction) AND cryptographic integrity (signatures, tamper detection).

**Status**: 🏁 Foundation complete. Ready for advanced features.

---

## Running the Examples

```bash
# Full demonstration of both features
cargo run --example descendancy_demo

# Inspect graveyard for genealogy
cargo run --example graveyard_inspector -- --summarize

# Verify cryptographic seals
cargo run --example graveyard_inspector -- --verify <AGENT_ID>

# Detailed autopsy with genealogy
cargo run --example graveyard_inspector -- --autopsy <AGENT_ID>
```

---

## Documentation

- See [GRAVEYARD_DELIVERY.md](./GRAVEYARD_DELIVERY.md) for Graveyard system overview
- See [GRAVEYARD_GUIDE.md](./GRAVEYARD_GUIDE.md) for usage guide
- See [examples/descendancy_demo.rs](./examples/descendancy_demo.rs) for code examples
