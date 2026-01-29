# Lineage Examples & Demonstrations

This directory contains runnable demonstrations of the Lineage system - an ontological framework for software that experiences irreversible consequences.

## Quick Start

Run any of these to watch an agent lifecycle unfold:

```bash
# Main binary with detailed visualization
cargo run --bin demo

# Lifecycle with comprehensive energy/damage tracking
cargo run --example lifecycle_demo

# Dramatic agent death demonstration
cargo run --example mortality
```

## What They Show

### `demo` (Main Binary)
The primary demonstration showing the complete agent lifecycle:
- Birth with finite energy (250 units)
- Successful early operations (confidence building)
- Accumulating scars and failures
- Degraded capacity from accumulated damage
- Final energy exhaustion

Shows how:
- Tasks consume energy irreversibly
- Failures inflict permanent damage
- Damage multiplier increases future task costs
- Capacity is degraded by accumulated scar tissue

### `lifecycle_demo` (Example)
Complete lifecycle demonstration with visual bars:
- Phase 1: Healthy operation (5 successful tasks)
- Phase 2: Increasing strain (8 failure scenarios)
- Demonstrates energy depletion and damage accumulation
- Shows realistic failure modes (network, memory, consistency issues)

Key insight: Failures don't just fail—they leave permanent marks that make future tasks harder.

### `mortality` (Example)
Shows the harsh reality that **every agent dies**:
- Early optimism (3 successful tasks)
- Cascading failures (each worse than the last)
- Inevitable energy exhaustion
- Either dramatic catastrophic failure or starvation

Key insight: There is no recovery. The system inexorably moves toward death.

## Core Mechanics Demonstrated

### Energy System
- Agents start with a fixed energy budget
- Every task attempt costs energy (success or failure)
- Damage increases the cost of all future tasks
- When energy reaches zero, the agent is dead

### Damage & Capacity
- Task failures inflict scars of varying severity:
  - **Minor** (1 damage): Recoverable errors
  - **Moderate** (5 damage): Significant failures
  - **Severe** (20 damage): Major system errors
  - **Fatal**: Immediate termination
- Base capacity is 100
- Capacity degrades by damage score: `capacity = 100 - damage`
- Tasks can require minimum capacity to execute
- Low capacity eventually blocks all operations

### Cost Multiplier
```
Actual Cost = Base Cost × (1.0 + damage_score / 100.0)
```
- A task with 50 damage costs 50% more energy
- This creates a death spiral: damage makes tasks more expensive, leading to more damage

### Irreversibility
- **Energy cannot be restored** - it only ever decreases
- **Scars cannot be healed** - damage is permanent
- **Death is final** - terminated agents cannot be revived or reset
- **History is immutable** - all events are recorded forever

## Understanding the Output

Each demonstration shows:
- **Energy bar**: How much energy remains (fills left-to-right)
- **Damage bar**: Accumulated scar tissue (fills left-to-right)
- **Capacity**: Current task capacity (100 minus damage)
- **Task counts**: Successful vs failed operations
- **Status**: 🟢 ALIVE or ⚫ DEAD

## What You'll Learn

These demos illustrate the core principle: **consequences are permanent**.

- A system with finite resources faces inevitable entropy
- Errors aren't failures to recover from—they're wounds that never heal
- Each failure makes the next task harder
- No amount of success changes the fundamental trajectory toward death
- Perfect success early just delays the inevitable

## The Philosophy

Lineage inverts common software design:

| Traditional Software | Lineage |
|---|---|
| Errors trigger recovery | Errors inflict scars |
| Infinite retries | Finite energy |
| State can be reset | History is immutable |
| Systems are resilient | Systems are mortal |
| Learning from mistakes | Bearing the cost of mistakes |

This isn't designed for happy-path optimization. It's designed for **ontological honesty** about what happens to systems over time.

## Running Your Own Experiments

To create your own agent lifecycle, use the `TaskAgent` API:

```rust
use lineage::{TaskAgent, Task, TaskOutcome};

let mut agent = TaskAgent::create(500);  // 500 energy units

// Execute a successful task
let task = Task::new("Work".to_string(), 50);
agent.execute_task(task, TaskOutcome::Success);

// Execute a task that fails and causes damage
let failing_task = Task::new("Hard work".to_string(), 75);
agent.execute_task(failing_task, TaskOutcome::SevereFailure {
    reason: "Resource exhaustion".to_string(),
});

println!("Energy: {}, Damage: {}, Alive: {}", 
    agent.energy(), 
    agent.damage_score(),
    agent.is_alive()
);
```

See [src/agent.rs](../src/agent.rs) for the full API.
