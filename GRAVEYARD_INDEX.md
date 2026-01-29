# Graveyard System - Quick Reference Index

## 📚 Documentation

| Document | Purpose | Read Time |
|----------|---------|-----------|
| [GRAVEYARD_GUIDE.md](GRAVEYARD_GUIDE.md) | Complete system guide with usage examples | 20 min |
| [GRAVEYARD_TESTING.md](GRAVEYARD_TESTING.md) | Testing procedures and validation | 15 min |
| [GRAVEYARD_IMPLEMENTATION.md](GRAVEYARD_IMPLEMENTATION.md) | Implementation details and architecture | 10 min |
| [GRAVEYARD_COMPLETION_REPORT.md](GRAVEYARD_COMPLETION_REPORT.md) | Final summary and status | 10 min |

## 🚀 Quick Start

### 1. Test Single Agent Death (30 seconds)
```bash
cargo run --example ghost_in_the_machine
cargo run --example graveyard_inspector -- --summarize
```

### 2. Population Simulation (1 minute)
```bash
cargo run --release --example multi_agent_competition
cargo run --example graveyard_inspector -- --darwinian
```

### 3. Multi-Generational Study (5 minutes)
```powershell
.\archaeologist.ps1     # Windows
bash archaeologist.sh   # Unix
```

## 🔍 Graveyard Inspector Commands

```bash
# List all dead agents
cargo run --example graveyard_inspector -- --summarize

# Examine specific agent
cargo run --example graveyard_inspector -- --autopsy <ID_PREFIX>

# Verify data integrity
cargo run --example graveyard_inspector -- --verify <ID_PREFIX>

# Find evolutionary winner
cargo run --example graveyard_inspector -- --darwinian
```

## 📁 File Locations

**Core Implementation:**
- `src/graveyard.rs` - Main graveyard system (539 lines)
- `src/agent.rs` - Agent burial integration
- `src/lib.rs` - Public API exports

**Examples:**
- `examples/ghost_in_the_machine.rs` - Single agent demo
- `examples/graveyard_inspector.rs` - Analysis tool
- `examples/multi_agent_competition.rs` - Population dynamics

**Automation:**
- `archaeologist.ps1` - Windows multi-generation script
- `archaeologist.sh` - Unix multi-generation script

**Data Storage:**
- `.lineage/graveyard/` - Tombstone files (read-only JSON)

## 🎯 Key Features

✅ **Immutable Tombstones**: Complete agent records sealed forever  
✅ **Persistent Storage**: JSON files with OS-level read-only protection  
✅ **Lazarus Prevention**: Dead IDs cannot be reused  
✅ **Atomic Writes**: Temp file + rename prevents corruption  
✅ **Fast Lookups**: O(1) registry for graveyard checks  
✅ **Zero-Cost Inspection**: Lazy loading of records  
✅ **Forensic Analysis**: 4 inspector commands  
✅ **Multi-Generational**: Archaeologist scenario for long-term studies  

## 📊 What's Stored in a Tombstone

Each dead agent's record includes:
- **Identity**: Unique ID, creation timestamp
- **Metabolism**: Energy stats, efficiency, tasks completed
- **Pathology**: Complete scar history with timestamps
- **Causality**: Merkle hash chain for integrity verification

## 🧪 Validation Status

All systems tested and verified:
- ✅ Compilation: No errors
- ✅ Examples: All working
- ✅ Data: Correctly stored
- ✅ Protection: Files are read-only
- ✅ Performance: < 1 second for 50+ agents
- ✅ Documentation: Complete

## 🔗 Integration Points

The Graveyard integrates with:
- `TaskAgent::create()` - Checks if ID is dead
- `TaskAgent::bury()` - Writes tombstone
- `Graveyard::initialize()` - Sets up persistence
- `GRAVEYARD_REGISTRY` - Fast lookups

## 💡 Common Tasks

### Kill an agent and examine record
```bash
cargo run --example ghost_in_the_machine
cargo run --example graveyard_inspector -- --autopsy 775e26461b
```

### Run a population simulation
```bash
cargo run --release --example multi_agent_competition
```

### Analyze evolutionary trends
```bash
cargo run --example graveyard_inspector -- --darwinian
```

### Verify data hasn't been tampered with
```bash
cargo run --example graveyard_inspector -- --verify <ID>
```

### Backup graveyard records
```powershell
Copy-Item .\.lineage\graveyard\*.tomb -Destination backup/
```

## 📖 Reading Guide

**New to the system?** Start here:
1. Read [GRAVEYARD_GUIDE.md](GRAVEYARD_GUIDE.md) overview
2. Run `ghost_in_the_machine` example
3. Explore inspector commands

**Running tests?** See:
1. [GRAVEYARD_TESTING.md](GRAVEYARD_TESTING.md) for test procedures
2. Run quick start tests above
3. Check validation checklist

**Need technical details?** Read:
1. [GRAVEYARD_IMPLEMENTATION.md](GRAVEYARD_IMPLEMENTATION.md) for architecture
2. Review `src/graveyard.rs` source code
3. Check README.md for integration

**Looking for status?** See:
1. [GRAVEYARD_COMPLETION_REPORT.md](GRAVEYARD_COMPLETION_REPORT.md)
2. File statistics and feature checklist
3. Production readiness indicators

## ❓ FAQ

**Q: How long do tombstones persist?**  
A: Forever. Read-only files at OS level prevent deletion.

**Q: Can I restore a dead agent?**  
A: No. Lazarus prevention panics if you try to reuse an ID.

**Q: How fast is the graveyard system?**  
A: Very fast. Lookups are O(1), summarize is O(n) where n ≈ 50 agents.

**Q: What if I want to archive records?**  
A: Copy `.lineage/graveyard/*.tomb` files to backup location.

**Q: Can I query the graveyard?**  
A: Use the inspector tool. Future: SQL-like queries possible.

## 🏆 System Philosophy

> The dead do not disappear. They become history.

The Graveyard embodies Lineage's core principle: **consequences are permanent**. Every agent that dies contributes to an eternal archive, enabling:

- Historical analysis of population dynamics
- Evolutionary pressure measurement
- Forensic investigation of failures
- Trust scoring based on permanent records

---

**Status: ✅ PRODUCTION READY**  
**Last Updated: January 30, 2026**
