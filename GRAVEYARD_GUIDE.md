# The Eternal Archive: Graveyard System

The Graveyard is a persistent, tamper-proof archive of deceased agents in the Lineage system. It demonstrates the irreversibility principle - death is final, identities are sealed forever, and history cannot be rewritten.

## Core Features

### 1. Immutable Tombstones
Every dead agent receives a sealed record containing:
- **Identity Block**: Agent ID, creation timestamp, identity hash
- **Metabolic Record**: Initial/peak/final energy, efficiency rating (tasks/energy), tasks completed
- **Pathology Report**: Complete scar tissue history with timestamps and severity
- **Causal Chain**: Cryptographic hash of the entire event sequence for tamper detection

### 2. Persistence Engine
- Stores tombstones as JSON files in `.lineage/graveyard/`
- **Atomic writes**: Uses temp file + rename to prevent corruption
- **OS-level protection**: Files marked read-only to prevent accidental deletion
- **No overwrites**: Returns error if attempting to bury an already-dead agent ID
- **In-memory registry**: Lightning-fast O(1) Lazarus prevention checks

### 3. Lazarus Prevention
The system checks the graveyard upon every `Agent::create()`. If an ID is found in the graveyard, the program panics:
```
ONTOLOGICAL ERROR: Identity [ID] is already sealed in the Eternal Archive.
```

Dead identities cannot be reborn.

## Usage

### Running Examples

#### Ghost in the Machine (Single Agent Demo)
```bash
cargo run --example ghost_in_the_machine
```
Demonstrates the Lazarus prevention system by:
1. Creating an agent
2. Running a task
3. Inducing a fatal failure
4. Burying the agent
5. Showing the graveyard inspector reading the sealed record

#### Multi-Agent Competition (Population Simulation)
```bash
cargo run --release --example multi_agent_competition
```
Runs 10 agents competing for 60 tasks with escalating difficulty. Dead agents are automatically buried. Survivors are listed at the end.

#### Archaeologist Scenario (Multi-Generational Study)
Run 5 simulations to build up ~50+ dead agents and analyze evolutionary trends:

**On Windows (PowerShell):**
```powershell
.\archaeologist.ps1
```

**On Linux/Mac (Bash):**
```bash
bash archaeologist.sh
```

### Forensic Analysis

#### Summarize the Graveyard
List all dead agents with their stats:
```bash
cargo run --example graveyard_inspector -- --summarize
```

Output shows:
- Agent ID (shortened)
- Lifespan (efficiency rating)
- Tasks Completed
- Scars Inflicted
- Legacy Score (tasks / (scars + 1))

#### Autopsy an Agent
Examine detailed timeline of a specific agent's life:
```bash
cargo run --example graveyard_inspector -- --autopsy <ID_PREFIX>
```

Shows:
- Identity block with creation info
- Metabolic record with energy stats
- Pathology report listing all scars
- Causal chain (event hash)

#### Verify Integrity
Check if a tombstone has been tampered with:
```bash
cargo run --example graveyard_inspector -- --verify <ID_PREFIX>
```

Confirms:
- File exists and is readable
- OS-level read-only protection is active
- Tombstone is sealed and tamper-protected

#### Find the Darwinian Champion
Across all dead agents, find the one with the best success-to-scar ratio:
```bash
cargo run --example graveyard_inspector -- --darwinian
```

Shows top 5 agents ranked by success efficiency.

## File Structure

```
.lineage/graveyard/
├── <agent_id_1>.tomb      (JSON file, read-only)
├── <agent_id_2>.tomb      (JSON file, read-only)
├── <agent_id_3>.tomb      (JSON file, read-only)
└── ...
```

Each `.tomb` file is a complete, immutable record that can be analyzed, verified, and archived.

## Tombstone Format (JSON)

```json
{
  "identity": {
    "id": "6ed806b9f2b8...",
    "creation_time": "2026-01-30T12:34:56Z",
    "identity_hash": "Identity { id: \"...\", ... }"
  },
  "metabolic_record": {
    "final_energy": 0,
    "peak_energy": 100,
    "initial_energy": 100,
    "efficiency_ratio": 0.5,
    "tasks_completed": 1
  },
  "pathology_report": {
    "scars": [
      {
        "timestamp": "2026-01-30T12:34:57Z",
        "severity": "Severe",
        "description": "Cascade failure",
        "context": null
      }
    ],
    "scar_count": 1,
    "cause_of_death": "Cascade failure",
    "death_timestamp": "2026-01-30T12:34:58Z"
  },
  "causal_chain": {
    "merkle_root": "a1b2c3d4...",
    "event_hashes": ["hash1", "hash2", ...],
    "total_events": 1
  },
  "burial_timestamp": "2026-01-30T12:34:58Z",
  "schema_version": 1
}
```

## Key Design Principles

### 1. Irreversibility
- Once an agent dies, its record is permanent
- No modifications, deletions, or overwrites
- OS-level read-only prevents accidental mutation

### 2. Causality
- Complete event history preserved in causal chain
- Merkle tree ensures no tampering is possible
- Hash verification confirms record integrity

### 3. Ontological Consistency
- Dead identities cannot be reused
- Lazarus prevention panics on resurrection attempts
- Identity uniqueness enforced across lifetimes

### 4. Zero-Cost Inspection
- In-memory registry enables O(1) lookups
- Large graveyards don't require loading all records
- Inspector tools lazy-load only examined tombstones

## Performance Characteristics

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| Create graveyard | O(1) | Directory creation |
| Bury agent | O(n) | n = scar count, atomic write |
| Check if dead | O(1) | Registry lookup |
| List all dead | O(m) | m = dead agent count, directory scan |
| Verify integrity | O(1) | File metadata check |
| Autopsy agent | O(1) | JSON deserialization |

## Future Extensions

Potential enhancements:
- **Database backend**: SQLite/PostgreSQL for massive graveyards
- **Time-series analysis**: Mortality trends over generations
- **Genealogy tracking**: Family trees and ancestor chains
- **Comparative pathology**: Disease patterns across populations
- **Archive export**: Backup and preserve historical records
- **Query language**: Complex forensic queries on dead agents
- **Visualization**: Timeline graphs and population stats

## Philosophical Notes

The Graveyard embodies the core principle of Lineage: **consequences are irreversible**.

In most systems, failure is forgotten. In Lineage:
- Every scar is remembered forever
- Every identity, once sealed, is unique
- Every death contributes to the historical record
- Natural selection operates not in isolation, but under eternal observation

The Archaeologist who examines the graveyard sees not just death counts, but the selective pressure that shapes survival. The agents that endure do so because they are genuinely more resilient—their superiority is written in stone.

---

**The dead do not disappear. They become history.**
