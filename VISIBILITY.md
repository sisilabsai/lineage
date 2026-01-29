# Lineage Visibility Guide

## Making the Lineage System Visible

The Lineage system is now fully demonstrable through three runnable executables that visualize an agent's complete lifecycle from birth through death.

### Run Any of These to Watch Lineage in Action

```bash
# Primary demonstration - shows complete lifecycle phases
cargo run --bin demo

# Comprehensive lifecycle with detailed phases
cargo run --example lifecycle_demo

# Dramatic death demonstration
cargo run --example mortality
```

All three visualize:
- **Energy consumption** (finite, irreversible)
- **Damage accumulation** (permanent scars)
- **Capacity degradation** (effects of accumulated damage)
- **Complete lifecycle** (birth → operation → decay → death/termination)

## What Happens When You Run These

Each demo watches an agent:

1. **Birth** with a fixed energy budget (250-500 units)
2. **Early Success** with low-cost tasks 
3. **Mounting Failures** that accumulate damage
4. **Capacity Loss** as scars degrade its ability to work
5. **Energy Crisis** when costs exceed available energy
6. **Death or Survival** - the terminal state

### Real Output Example

```
┌──────────────────────────────────────────────────────┐
│ Status: 🟢 ALIVE                                 │
├──────────────────────────────────────────────────────┤
│ Energy:  ███████████░░░░░░░░░ 140/250
│ Damage:  ▓▓▓░░░░░░░░░░░░░░░░░ 15/100
│ Capacity: ▰▰▰▰▰▰▰▰▰▰▱▱▱▱▱ 85/100
├──────────────────────────────────────────────────────┤
│ Completed:  15 | Failed:   8                       │
└──────────────────────────────────────────────────────┘
```

This shows:
- **140 energy remaining** from 250 initial
- **15 damage score** (scars from failures)
- **85 capacity** (down from initial 100, reduced by damage)
- **15 successful, 8 failed tasks** (each failure costs energy AND inflicts damage)

## The Lifecycle Mechanics

### Energy System
- **Finite**: Agent starts with fixed energy budget
- **Consumed**: Every task attempt uses energy (success or failure)
- **Irreversible**: Energy never returns
- **Death trigger**: When energy hits zero

### Damage & Scars
- **Inflicted by failures**: Task errors create permanent damage
- **Permanent**: Scars never heal
- **Multiplicative**: Each point of damage increases future task costs by 1%
- **Capacity limit**: Damage > 100 means capacity = 0 (agent immobilized)

### Cost Escalation
```
actual_cost = base_cost × (1.0 + damage_score / 100.0)

Example:
- Task costs 50 energy normally
- With 50 damage: 50 × 1.50 = 75 energy
- With 100 damage: 50 × 2.00 = 100 energy
```

This creates a **death spiral**:
1. Failure inflicts damage
2. Damage increases all future costs
3. Higher costs drain energy faster
4. Energy exhaustion forces more failures
5. More failures = more damage
6. Inevitable death

## Key Insights From Watching These

1. **No Recovery**: Unlike resilient systems, Lineage has no healing. Scars are permanent.

2. **Energy is Time**: Energy isn't just a resource—it's how much the agent can exist.

3. **Failure is Expensive**: A failed task consumes energy AND inflicts damage (double cost).

4. **Early Damage Matters**: Damage accumulated early multiplies costs for the entire lifetime.

5. **Success Prolongs Death**: Good execution just delays the inevitable—entropy wins.

6. **Ontological Honesty**: The system truthfully models how real systems degrade over time.

## For Developers

To use this in your code:

```rust
use lineage::{TaskAgent, Task, TaskOutcome};

// Create an agent with 500 energy units
let mut agent = TaskAgent::create(500);

// Check it's alive
assert!(agent.is_alive());
assert_eq!(agent.energy(), 500);
assert_eq!(agent.damage_score(), 0);

// Execute a successful task
let task = Task::new("Work".to_string(), 100);
agent.execute_task(task, TaskOutcome::Success);
// Now energy = 400

// Execute a task that fails
let task = Task::new("Hard work".to_string(), 80);
agent.execute_task(task, TaskOutcome::SevereFailure {
    reason: "Out of memory".to_string(),
});
// Energy = 320 (cost increased by damage penalty)
// Damage = 20 (severe failure inflicts 20 damage)
// All future tasks cost 20% more
```

See [EXAMPLES.md](./EXAMPLES.md) for more detailed information.

## Philosophy

Lineage inverts typical software design principles:

| Aspect | Traditional Systems | Lineage |
|--------|-------------------|---------|
| **Errors** | Trigger recovery | Inflict permanent scars |
| **Retries** | Unlimited | Limited by finite energy |
| **State Reset** | Possible | Forbidden (immutable history) |
| **System Model** | Resilient | Mortal |
| **Goal** | Optimization | Ontological honesty |

The core idea: software systems are mortal entities that experience consequences. Design should reflect that truth.

## Run the Demos Now

```bash
# See a complete agent lifecycle
cargo run --bin demo

# Or one of the examples:
cargo run --example lifecycle_demo
cargo run --example mortality
```

Watch as an agent is born, operates, accumulates damage, and faces entropy. 
The lineage is now visible.
