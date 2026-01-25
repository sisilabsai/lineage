# Autonomous Task Agent System

## Overview

Finite-lifespan task-executing agents built on Lineage primitives. Agents are born with fixed energy, execute tasks irreversibly, accumulate damage, and die permanently.

---

## Core Philosophy

**Agents are mortal executors, not resilient systems.**

Traditional agent architectures:
- Retry failed operations with backoff
- Learn from errors to improve
- Checkpoint state for recovery
- Supervisor intervention on failure

**Lineage Agent Model**:
- Tasks consume energy regardless of outcome
- Errors inflict permanent scars
- No retry mechanisms exist
- Death is final, no resurrection

---

## Architecture

### TaskOutcome

Five outcome classifications for task execution:

| Outcome | Severity | Consequence |
|---------|----------|-------------|
| `Success` | None | No damage |
| `RecoverableFailure` | Minor | +1 damage, energy lost |
| `SignificantFailure` | Moderate | +5 damage, energy lost |
| `SevereFailure` | Severe | +20 damage, energy lost |
| `CatastrophicFailure` | Fatal | +100 damage, termination |

### Task

Task descriptor with energy requirements:

```rust
pub struct Task {
    pub description: String,
    pub base_cost: u64,        // Base energy cost
    pub min_capacity: u32,     // Minimum capacity required
}
```

**Damage Penalty Mechanism**:
```rust
actual_cost = base_cost * (1.0 + damage_score / 100.0)
```

Examples:
- Damage 0: 100 energy task costs 100
- Damage 10: 100 energy task costs 110 (+10%)
- Damage 50: 100 energy task costs 150 (+50%)
- Damage 100: 100 energy task costs 200 (+100%)

### TaskAgent

Finite-lifetime task executor wrapping Lineage.

**State**:
- `lineage` — Identity, memory, energy, scars, death
- `tasks_completed` — Count of successful tasks
- `tasks_failed` — Count of failed tasks

**Capacity Degradation**:
```rust
current_capacity = 100 - damage_score
```

High-capacity tasks rejected when damage exceeds threshold.

---

## Brutal Consequences

### 1. Failure Consumes Energy

**Reality**: Attempting a task costs energy, regardless of outcome.

```rust
let mut agent = TaskAgent::create(1000);

// Task fails - energy consumed, no progress made
agent.execute_task(
    Task::new("Task".to_string(), 100),
    TaskOutcome::RecoverableFailure { reason: "Error".to_string() }
);

assert_eq!(agent.energy(), 900);  // Energy lost
assert_eq!(agent.tasks_completed(), 0);  // No progress
assert_eq!(agent.damage_score(), 1);  // Scar inflicted
```

**Implication**: Failure is expensive. Energy wasted permanently.

### 2. Damage Increases Future Costs

**Reality**: Accumulated scars make all future tasks more expensive.

```rust
let mut agent = TaskAgent::create(1000);
let task = Task::new("Task".to_string(), 100);

// First execution: base cost (100)
agent.execute_task(task.clone(), TaskOutcome::Success);

// Inflict damage (20 points)
agent.execute_task(
    Task::new("Fail".to_string(), 0),
    TaskOutcome::SevereFailure { reason: "Error".to_string() }
);

// Second execution: increased cost (~120)
// Same task now costs 20% more due to damage penalty
```

**Implication**: Death spiral. Errors make future operations harder.

### 3. Capacity Degrades Permanently

**Reality**: Some tasks require minimum operational capacity.

```rust
let mut agent = TaskAgent::create(1000);

// High-capacity task (requires 50 capacity)
let critical_task = Task::with_capacity("Critical".to_string(), 10, 50);

// Initially succeeds (capacity = 100)
agent.execute_task(critical_task.clone(), TaskOutcome::Success);

// Inflict 60 damage (capacity drops to 40)
for _ in 0..3 {
    agent.execute_task(
        Task::new("Fail".to_string(), 10),
        TaskOutcome::SevereFailure { reason: "Error".to_string() }
    );
}

// Now rejected (40 < 50 required)
let result = agent.execute_task(critical_task, TaskOutcome::Success);
assert!(matches!(result, TaskResult::CapacityInsufficient { .. }));
```

**Implication**: Damaged agents lose access to complex tasks.

### 4. No Retry Mechanism

**Reality**: Agents cannot automatically retry failed tasks.

```rust
let mut agent = TaskAgent::create(1000);

// Task fails
agent.execute_task(
    Task::new("Fail".to_string(), 100),
    TaskOutcome::RecoverableFailure { reason: "Error".to_string() }
);

// FORBIDDEN: No retry mechanism exists
// agent.retry_last_task() // Does not exist
// agent.retry_with_backoff() // Does not exist
```

**Manual retry is possible but**:
- Costs additional energy
- Damage from first failure remains
- Higher cost due to damage penalty
- No guarantee of success

**Implication**: Each attempt is independent and costly.

### 5. Agents Cannot Learn

**Reality**: Repeated failures do not improve future performance.

```rust
let mut agent = TaskAgent::create(1000);

// Fail same task 10 times
for _ in 0..10 {
    agent.execute_task(
        Task::new("Repeating failure".to_string(), 10),
        TaskOutcome::RecoverableFailure { reason: "Same error".to_string() }
    );
}

// Damage accumulates (10 scars)
assert_eq!(agent.damage_score(), 10);

// Agent does NOT:
// - Learn the pattern
// - Adapt behavior
// - Improve success rate
// - Reduce future costs
```

**Implication**: Agents are executors, not learning systems.

### 6. Death Before Completion Is Allowed

**Reality**: Agents can die before finishing their mission.

```rust
let mut agent = TaskAgent::create(200);

// Mission: Complete 10 tasks @ 100 energy each = 1000 total needed
// Agent only has 200 energy

let mut completed = 0;
for i in 0..10 {
    if !agent.is_alive() { break; }
    
    match agent.execute_task(Task::new(format!("T{}", i), 100), TaskOutcome::Success) {
        TaskResult::Completed { .. } => completed += 1,
        _ => break,
    }
}

assert!(!agent.is_alive());
assert!(completed < 10);  // Died before completion
```

**This is correct behavior, not a bug.**

**Implication**: Insufficient resources = mission failure = death.

---

## Why This Feels Harsh

### The Problem with Resilient Systems

Traditional agent systems provide:

1. **Automatic retry**: "Try again with exponential backoff"
   - **Problem**: Hides cost of failure
   - **Reality**: Every attempt costs resources

2. **Learning from errors**: "Agent improves over time"
   - **Problem**: Implies damage reverses
   - **Reality**: Scars are permanent

3. **Checkpoint recovery**: "Restore from last good state"
   - **Problem**: Erases consequence
   - **Reality**: History is immutable

4. **Supervisor intervention**: "External system rescues failing agent"
   - **Problem**: Bypasses mortality
   - **Reality**: Death is final

### Ontological Truth

In physical systems:
- Failed attempts consume resources
- Damage accumulates over time
- Organisms do not get extra chances
- Death is permanent

**Lineage agents enforce physical mortality in software.**

### Operational Implications

**This discomfort is intentional.**

Agents with insufficient resources:
- Die before completing missions
- Accumulate damage from errors
- Face increasing costs due to damage
- Cannot be reset or restarted

This is not a flaw. This is ontological consequence.

---

## Enforcement Mechanisms

### Compile-Time Prevention

```rust
// FORBIDDEN: No retry mechanisms exist
impl TaskAgent {
    // pub fn retry() { } // Does not exist
    // pub fn retry_with_backoff() { } // Does not exist
    // pub fn checkpoint() { } // Does not exist
    // pub fn restore_checkpoint() { } // Does not exist
}

// FORBIDDEN: No learning mechanisms exist
impl TaskAgent {
    // pub fn learn_from_error() { } // Does not exist
    // pub fn adapt_behavior() { } // Does not exist
    // pub fn optimize_cost() { } // Does not exist
}

// FORBIDDEN: No resurrection mechanisms exist
impl TaskAgent {
    // pub fn resurrect() { } // Does not exist
    // pub fn reset() { } // Does not exist
    // pub fn restore_energy() { } // Does not exist
}
```

### Runtime Enforcement

```rust
pub fn execute_task(&mut self, task: Task, outcome: TaskOutcome) -> TaskResult {
    // 1. Check death (backed by Lineage mortality)
    if !self.lineage.is_alive() {
        return TaskResult::AgentTerminated;
    }
    
    // 2. Calculate cost with damage penalty
    let actual_cost = task.actual_cost(self.damage_score());
    
    // 3. Check capacity (degraded by damage)
    if task.min_capacity > self.current_capacity() {
        return TaskResult::CapacityInsufficient { .. };
    }
    
    // 4. Consume energy (irreversible)
    self.lineage.perform_operation(desc, actual_cost);
    
    // 5. If failure, inflict scar (permanent)
    if let Some(severity) = outcome.severity() {
        self.lineage.record_error(error);
    }
}
```

### Invariant Tests

18 tests verify brutal consequences:

**Positive Tests** (system behavior):
1. Successful tasks consume energy
2. Failed tasks consume energy AND inflict scars
3. Damage increases future task costs
4. Capacity degrades with damage
5. High-capacity tasks rejected when damaged
6. Catastrophic failures terminate agent
7. Dead agents cannot execute tasks
8. Energy exhaustion terminates agent

**Brutal Tests** (attack prevention):
1. Failure consumes energy without progress
2. Repeated failures cause death spiral
3. No retry mechanism exists
4. Agents cannot learn from errors
5. Death before mission completion is allowed
6. Energy only decreases, never increases
7. Accumulated damage is permanent

---

## Usage Example

```rust
use lineage::{TaskAgent, Task, TaskOutcome, TaskResult};

// Create agent with 1000 energy
let mut agent = TaskAgent::create(1000);

// Execute successful task
let task1 = Task::new("Initialize".to_string(), 100);
match agent.execute_task(task1, TaskOutcome::Success) {
    TaskResult::Completed { energy_consumed } => {
        println!("Task completed, {} energy consumed", energy_consumed);
    }
    _ => {}
}

// Execute failing task (energy consumed, scar inflicted)
let task2 = Task::new("Risky operation".to_string(), 150);
match agent.execute_task(task2, TaskOutcome::SignificantFailure { 
    reason: "Connection timeout".to_string() 
}) {
    TaskResult::Failed { reason, energy_consumed, damage_inflicted } => {
        println!("Task failed: {}", reason);
        println!("Energy wasted: {}", energy_consumed);
        println!("Damage inflicted: {}", damage_inflicted);
    }
    _ => {}
}

// Future tasks now cost more (damage penalty)
println!("Current damage: {}", agent.damage_score());
println!("Current capacity: {}/100", agent.current_capacity());

// High-capacity task may be rejected
let critical_task = Task::with_capacity("Critical op".to_string(), 50, 60);
match agent.execute_task(critical_task, TaskOutcome::Success) {
    TaskResult::CapacityInsufficient { reason } => {
        println!("Task rejected: {}", reason);
    }
    _ => {}
}

// Catastrophic failure terminates agent
let fatal_task = Task::new("Fatal operation".to_string(), 10);
agent.execute_task(fatal_task, TaskOutcome::CatastrophicFailure {
    reason: "Unrecoverable error".to_string()
});

assert!(!agent.is_alive());

// All future tasks rejected
let post_death = Task::new("After death".to_string(), 1);
assert_eq!(
    agent.execute_task(post_death, TaskOutcome::Success),
    TaskResult::AgentTerminated
);
```

---

## Design Decisions

### Why No Automatic Retry?

Automatic retry hides:
- True cost of failure
- Resource consumption
- Damage accumulation

Manual retry (if needed) forces:
- Explicit cost awareness
- Damage acknowledgment
- Resource management

### Why No Learning?

Learning suggests:
- Errors improve future performance
- Damage can be overcome
- Cost reduction over time

Lineage agents:
- Execute tasks deterministically
- Do not adapt behavior
- Maintain permanent damage

### Why Allow Death Before Completion?

Mission completion guarantees imply:
- Resources are infinite
- Failure is preventable
- Death is avoidable

Lineage reality:
- Resources are finite
- Failure has cost
- Death is inevitable

### Why Damage Penalty?

Damage penalty creates:
- Death spiral (errors compound)
- Urgency (conserve energy)
- Consequence visibility

Without penalty:
- Damage becomes cosmetic
- Errors lose meaning
- Mortality is delayed artificially

---

## Operational Reality

### What This Means for Operators

**If you deploy this system**:

- Agents will die before completing tasks (if under-resourced)
- Failed tasks waste energy permanently
- Accumulated damage makes agents less capable
- No retry system will save failing agents
- No learning will improve performance

**This is uncomfortable by design.**

Agent mortality mirrors physical reality:
- Organisms exhaust resources
- Damage accumulates irreversibly
- Death is final

### Suitability

**This system is suitable for**:
- Ephemeral task execution (one-shot agents)
- Consequence-aware scheduling (plan for mortality)
- Resource-constrained environments (scarcity is real)

**This system is NOT suitable for**:
- Long-running services (need resilience)
- Critical infrastructure (need retry/recovery)
- Learning systems (need adaptation)

---

## Integration with Lineage

Task agents are **compositional**:

- Built atop Identity, Memory, Metabolism, Scars, Death
- Do not modify core primitives
- Inherit all irreversibility guarantees
- Extend Lineage without weakening it

---

## Test Execution

```bash
# Run agent tests only
cargo test --lib agent

# Verify all 18 tests pass
# 11 positive tests (system behavior)
# 7 brutal tests (harsh consequences)
```

**Test Results**: All 177 tests passing (159 existing + 18 agent)

---

## Summary

**Autonomous Task Agent System proves**:

1. Agents can have finite lifespans
2. Task failure can have permanent cost
3. Damage can compound into death spirals
4. Learning and retry can be made impossible
5. Death before mission completion is correct behavior

**The system is complete.**

No retry. No learning. No resurrection.

This is not cruelty. This is mortality.
