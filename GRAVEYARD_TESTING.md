# Graveyard System: Testing & Validation Guide

This document describes how to thoroughly test and validate the Graveyard system implementation.

## Quick Start Testing

### 1. Ghost in the Machine (5 minutes)

Tests the Lazarus prevention system with a single agent lifecycle:

```bash
cargo run --example ghost_in_the_machine
```

**Expected output:**
- Agent creation with unique ID
- Task execution (1 success)
- Fatal failure induction
- Agent burial in graveyard
- Confirmation that graveyard system works

**Validates:**
- ✓ Agent death detection
- ✓ Tombstone creation
- ✓ File writing to `.lineage/graveyard/`
- ✓ Lazarus prevention initialization

### 2. Multi-Agent Competition (2 minutes)

Simulates population-level dynamics with competitive pressure:

```bash
cargo run --release --example multi_agent_competition
```

**Expected output:**
- 10 agents competing for 60 tasks
- Tasks with escalating difficulty
- Some agents succeed, others fail
- Dead agents are buried automatically
- Post-mortem analysis showing survivors and sealed lineages

**Validates:**
- ✓ Multiple agent deaths
- ✓ Automatic burial on fatal failures
- ✓ Graveyard populated with 4+ tombstones
- ✓ Correct damage and energy calculations

### 3. Graveyard Inspector (1 minute each)

Examine the sealed records created by the competition:

#### Summarize all dead agents
```bash
cargo run --example graveyard_inspector -- --summarize
```

**Expected output:**
- Table of all dead agents
- Agent ID (shortened)
- Tasks Completed, Scars Inflicted
- Legacy Score calculation
- Total agent count

#### Autopsy a specific agent
```bash
cargo run --example graveyard_inspector -- --autopsy <ID_PREFIX>
```

Example:
```bash
cargo run --example graveyard_inspector -- --autopsy b0c4d2b22c
```

**Expected output:**
- Agent ID
- Created timestamp
- Metabolic record (energy stats)
- Pathology report (scar list)
- Causal chain (hash)

#### Verify integrity
```bash
cargo run --example graveyard_inspector -- --verify <ID_PREFIX>
```

**Expected output:**
- File path verification
- Read-only protection status
- Integrity confirmation

#### Find Darwinian champion
```bash
cargo run --example graveyard_inspector -- --darwinian
```

**Expected output:**
- Top 5 agents by success-to-scar ratio
- Winner highlighted
- Comparative statistics

## Comprehensive Testing Workflow

### Test 1: Fresh Start (All Systems)
```powershell
# Clear any old graveyard
Remove-Item .\.lineage\graveyard\* -Force -ErrorAction SilentlyContinue

# Run ghost example
cargo run --example ghost_in_the_machine

# Check graveyard was created
Get-ChildItem .\.lineage\graveyard\ | Measure-Object

# Inspect the agent
cargo run --example graveyard_inspector -- --summarize
```

**Expected results:**
- ✓ Exactly 1 agent in graveyard
- ✓ ID matches the one from ghost example
- ✓ Summarize shows it with 0.00 legacy score

### Test 2: Population Dynamics
```powershell
# Clear graveyard
Remove-Item .\.lineage\graveyard\* -Force -ErrorAction SilentlyContinue

# Run 2 competitions
cargo run --release --example multi_agent_competition
cargo run --release --example multi_agent_competition

# Check graveyard size
$count = @(Get-ChildItem .\.lineage\graveyard\*.tomb).Count
Write-Host "Total dead agents: $count"

# Analyze
cargo run --example graveyard_inspector -- --darwinian
```

**Expected results:**
- ✓ 6-8 total dead agents
- ✓ Darwinian champion identified
- ✓ No duplicate IDs in graveyard

### Test 3: Archaeologist Scenario (Full Historical Analysis)
```powershell
# Run the comprehensive script
.\archaeologist.ps1
```

This runs 5 generations of multi-agent competition and analyzes:
- Total agents in graveyard
- Population-level statistics
- Evolutionary trends
- Darwinian selection pressure

**Expected results:**
- ✓ 20-40+ dead agents total
- ✓ Clear champion with best success ratio
- ✓ Graveyard shows realistic population dynamics

### Test 4: Data Integrity
```powershell
# Create a tombstone
cargo run --example ghost_in_the_machine

# Get the tombstone file
$tomb = Get-ChildItem .\.lineage\graveyard\*.tomb | Select-Object -First 1

# Check read-only status
$file = Get-Item $tomb.FullName
$file.Attributes -contains "ReadOnly"  # Should be True

# Try to delete (should fail)
Remove-Item $tomb.FullName -Force 2>&1 | Select-String "Access is denied"
```

**Expected results:**
- ✓ File is read-only
- ✓ Cannot delete or modify tombstone
- ✓ Access denied on write attempts

### Test 5: Autopsy Deep Dive
```powershell
# Get a specific agent ID
$id = Get-ChildItem .\.lineage\graveyard\*.tomb | Select-Object -First 1 -ExpandProperty BaseName

# Full autopsy
cargo run --example graveyard_inspector -- --autopsy $id

# Verify integrity
cargo run --example graveyard_inspector -- --verify $id
```

**Expected output:**
- ✓ Complete identity, metabolic, and pathology records
- ✓ Read-only protection confirmed
- ✓ No missing or corrupted data

## Edge Cases & Stress Tests

### Test 6: Large Population (Optional)
```powershell
# Modify multi_agent_competition.rs:
# Change NUM_AGENTS from 10 to 100
# Change TOTAL_TASKS from 60 to 500

# Recompile and run
cargo run --release --example multi_agent_competition

# Inspector should still perform well
cargo run --example graveyard_inspector -- --summarize
cargo run --example graveyard_inspector -- --darwinian
```

**Performance expectations:**
- ✓ Build completes in < 5 seconds
- ✓ Competition runs in < 10 seconds
- ✓ Inspector summarize in < 1 second
- ✓ No memory issues with 50-100 tombstones

### Test 7: Concurrent Graveyard Access (Advanced)
```rust
// In a test file:
#[tokio::test]
async fn test_concurrent_burial() {
    Graveyard::initialize().unwrap();
    
    let handles: Vec<_> = (0..10)
        .map(|i| {
            tokio::spawn(async move {
                let agent = TaskAgent::create(100);
                // ... kill agent ...
                agent.bury().unwrap()
            })
        })
        .collect();
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    // All 10 should be buried without conflicts
    let graveyard = load_tombstones();
    assert_eq!(graveyard.len(), 10);
}
```

## Validation Checklist

### Core Functionality
- [ ] Ghost example runs without panic
- [ ] Agent is properly buried
- [ ] Graveyard directory is created
- [ ] Tombstone files are created with correct naming

### Data Integrity
- [ ] Tombstone files are read-only
- [ ] Cannot delete or modify buried agents
- [ ] JSON structure is valid
- [ ] All required fields are present

### Inspector Commands
- [ ] `--summarize` lists all agents
- [ ] `--autopsy` shows agent details
- [ ] `--verify` confirms integrity
- [ ] `--darwinian` identifies champion
- [ ] Help message displays correctly

### Population Dynamics
- [ ] Multiple runs accumulate dead agents
- [ ] No duplicate IDs appear
- [ ] Legacy scores calculated correctly
- [ ] Darwinian champion has best ratio

### Performance
- [ ] Summarize completes in < 1 second
- [ ] Autopsy completes in < 100ms
- [ ] Verify completes in < 50ms
- [ ] Can handle 100+ tombstones

### Error Handling
- [ ] Invalid agent ID gracefully fails
- [ ] Non-existent graveyard is created
- [ ] Overwrites correctly rejected
- [ ] File I/O errors are handled

## Success Criteria

The Graveyard system is complete when:

1. ✅ All examples compile without errors
2. ✅ Ghost scenario demonstrates Lazarus prevention
3. ✅ Multi-agent competition generates graveyard data
4. ✅ Inspector tools read and analyze tombstones
5. ✅ Files are read-only and tamper-proof
6. ✅ System handles multiple generations
7. ✅ Darwinian analysis identifies realistic champions
8. ✅ Performance is acceptable for 50+ agents
9. ✅ No data corruption on interruption
10. ✅ Documentation is complete and clear

---

**All tests passing? The Graveyard system is ready for production use.**
