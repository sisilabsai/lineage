# Pressure Log: Autonomous Agent System

**Domain**: Finite-lifespan task execution agents  
**Date**: 2026-01-22  
**Status**: Doctrine extraction from completed system

---

## A. Where We Wanted to Cheat

### 1. "Tasks should retry with exponential backoff"

**Temptation**: If task fails, wait 1s, 2s, 4s, 8s... then retry automatically until success.

**Why it felt necessary**: Networks are unreliable. Services have transient failures. Temporary errors shouldn't kill the agent.

**Why it felt reasonable**: Every production system does this. Retry-with-backoff is engineering 101. Not including it feels negligent.

**Reality**: Automatic retry consumes energy without forcing reconsideration. If an operation is worth trying five times, it should cost five times the energy up front. Retry hides the true cost of failure.

### 2. "Agents should learn from repeated failures"

**Temptation**: After failing the same task type three times, adjust strategy. Pattern recognition improves success rate.

**Why it felt necessary**: Smart agents adapt. Doing the same thing repeatedly expecting different results is insanity. Learning is what separates agents from scripts.

**Why it felt reasonable**: Machine learning exists. Agents should use their error history to improve. That's literally the point of having memory.

**Reality**: Learning implies damage reverses. If the agent "learned" from errors, scars should decrease future costs, not increase them. Learning is incompatible with permanent damage accumulation.

### 3. "Agents should checkpoint state and recover"

**Temptation**: Every N operations, save state to disk. On failure, restore from last checkpoint.

**Why it felt necessary**: Losing all progress due to one late-stage error is wasteful. Checkpoints minimize rework.

**Why it felt reasonable**: Databases do this. Operating systems do this. Fault tolerance requires snapshots.

**Reality**: Checkpointing erases causal history. If you can restore to "before the error," the error didn't happen in the timeline. The agent's identity becomes disconnected from its actual experiences.

### 4. "Dead agents should be restartable"

**Temptation**: When agent dies, spawn new agent with same mission and remaining tasks.

**Why it felt necessary**: The mission still needs completion. Letting work die with the agent wastes organizational resources.

**Why it felt reasonable**: Kubernetes does this. Process supervisors do this. High availability requires restarts.

**Reality**: Restarting is creating a new identity with a fabricated history. The new agent didn't experience the failures of the old one. It's identity laundering for systems.

### 5. "Agents need energy reserves for critical operations"

**Temptation**: Partition energy into "general" and "emergency reserve" pools. Critical operations can tap reserves.

**Why it felt necessary**: What if the agent runs out of energy right before a crucial final task? Some operations are non-optional.

**Why it felt reasonable**: Reserve capacity is standard practice. You don't want to strand an agent one operation away from success.

**Reality**: Reserves communicate that some energy is "more real" than other energy. If reserves can be tapped, they're just regular energy with extra steps. True scarcity has no reserves.

### 6. "Failed tasks should be skippable"

**Temptation**: If task fails, mark it skipped and proceed to next task. Come back to it if energy permits.

**Why it felt necessary**: One blocking failure shouldn't halt the entire workflow. Agents should make progress where possible.

**Why it felt reasonable**: Dependency graphs allow parallel execution. Skip failed branches and continue others.

**Reality**: Skipping is consequence avoidance. The task failed, consumed energy, inflicted damage—and now we pretend it doesn't block progress? Failure has to matter structurally, not just energetically.

### 7. "Damage should decrease over time without activity"

**Temptation**: If agent performs no operations for T seconds, reduce damage score by 1 per interval.

**Why it felt necessary**: "Healing" through rest is natural. Systems recover when load decreases.

**Why it felt reasonable**: Biological systems heal. Rate limiting allows recovery. Downtime has value.

**Reality**: Time-based healing implies damage is a temporary state, not a permanent scar. If scars fade, they weren't real consequences—they were cooldowns.

---

## B. What Lineage Forced Instead

### 1. Task sequencing must account for mortality

We couldn't add retry-with-backoff because each attempt consumes energy irreversibly. There's no "free retries."

**Decision that disappeared**: Default retry behavior. We can't make retry automatic because it would drain agents invisibly.

**Upstream change**: Task sequences must be designed with failure budgets. If a task might fail 3 times, the agent needs 3× energy allocated. Planners must think about failure cost, not just success cost.

### 2. Energy must be front-loaded for critical paths

We couldn't add reserves because all energy is accessible via `energy()`. There's no partitioning mechanism.

**Decision that disappeared**: Energy management strategies. Can't have "save 100 for final task" logic because there's no enforcement.

**Upstream change**: Mission-critical operations must happen first, while energy is abundant. Low-priority tasks happen later, when death risk is acceptable. Priority is enforced by sequence, not by reserves.

### 3. Failure must be embedded in workflows

We couldn't make tasks skippable because `execute_task()` consumes energy regardless of outcome. Failure isn't free.

**Decision that disappeared**: Graceful degradation. Can't have "core tasks succeed, optional tasks fail gracefully" because all failures inflict scars and consume energy.

**Upstream change**: Workflows must include failure paths explicitly. "If task fails, what's the degraded but acceptable outcome?" becomes a design requirement, not a runtime recovery strategy.

### 4. Mission completion is not guaranteed

We couldn't add agent restart because Lineage identity is unique and non-transferrable. Dead agents stay dead.

**Decision that disappeared**: Completion guarantees. We can't promise "mission will finish eventually" because agents can die mid-mission.

**Upstream change**: Missions must be scoped to energy budgets. If budget is insufficient, mission is impossible—this is detected at design time, not runtime. We fail fast by refusing to start, not by restarting repeatedly.

### 5. Agent pools replace individual resilience

We couldn't add checkpointing because Memory is append-only and cannot be "restored to previous state."

**Decision that disappeared**: Stateful recovery. Can't save and reload agent state.

**Upstream change**: Instead of making agents resilient, spawn multiple agents with overlapping capabilities. Redundancy at the population level, mortality at the individual level. Accept that some agents will die without completing their work.

### 6. Damage is designed into cost calculations

We couldn't add learning because there's no mechanism to reduce future costs based on past failures. `actual_cost = base_cost * damage_penalty` is immutable logic.

**Decision that disappeared**: Adaptive optimization. Can't have agents "get better" at tasks through practice.

**Upstream change**: Tasks must be designed assuming agents get worse over time. High-risk operations happen early, low-risk operations happen late. The natural progression is capability degradation, not improvement.

---

## C. What This Reveals About Software Culture

### 1. We design for infinite attempts, not finite resources

**Conventional agent systems**: Include retry loops, exponential backoff, circuit breakers—all assuming attempts are essentially free.

**What Lineage exposes**: Every attempt costs something permanent. Retry-until-success is an infinite resource assumption disguised as error handling.

**Modern agent systems lie**: By presenting retry as "fault tolerance" when it's really "we're pretending resources are infinite."

### 2. We treat failure as temporary state, not permanent event

**Conventional agent systems**: Failures are exceptions that can be caught, handled, and moved past. The system returns to "normal."

**What Lineage exposes**: Failure is an event in causal history. It happened. It consumed resources. It cannot be undone by catching an exception.

**Modern agent systems lie**: By using language like "recovered from error" when nothing was actually recovered—resources were spent and history was written.

### 3. We assume progress is inevitable if we keep trying

**Conventional agent systems**: Tasks will eventually succeed if we retry enough times. "Eventually consistent" architectures.

**What Lineage exposes**: Progress is not guaranteed. Finite resources + permanent damage = possible mission failure before completion.

**Modern agent systems lie**: By implying that persistence guarantees success, when reality includes unrecoverable failure states.

### 4. We build resilience theater instead of accepting mortality

**Conventional agent systems**: Agents that restart, retry, recover—simulating resilience through replacement.

**What Lineage exposes**: Restarting is creating new identity. If an agent dies and "restarts," a different entity with a clean slate is performing the work. This isn't resilience, it's replacement.

**Modern agent systems lie**: By calling replacement "recovery" and treating identity as transferrable.

### 5. We hide failure cost through abstraction

**Conventional agent systems**: Retry logic is buried in libraries. Developers call `fetch_with_retry()` and don't see the five network attempts.

**What Lineage exposes**: Each attempt has explicit energy cost. You cannot hide that you tried five times—it's in the energy consumption and the event log.

**Modern agent systems lie**: By making failure invisible through convenient abstractions that hide resource consumption.

### 6. We use "smart agents" as cover for mortality denial

**Conventional agent systems**: Agents that "learn" from errors and "adapt" strategy are portrayed as intelligent.

**What Lineage exposes**: Learning implies past damage can be overcome. If an agent truly learned, previous errors wouldn't penalize current operations. "Smart agents" are just agents that pretend their history doesn't matter.

**Modern agent systems lie**: By framing learning as intelligence when it's really consequence erasure.

### 7. We design task systems assuming agents are immortal

**Conventional agent systems**: Task queues, workflow engines, orchestrators—all assume agents will eventually drain the queue.

**What Lineage exposes**: Agents die. Task queues persist, but workers don't. Designing for "eventually all tasks complete" requires either infinite agents or immortal agents. Real systems have neither.

**Modern agent systems lie**: By scheduling work without accounting for worker mortality, then acting surprised when "hung workers" need manual cleanup.

---

## D. One Sentence of Doctrine

**Retry mechanisms exist to deny that failure is terminal and attempts are finite.**

---

## Signature

This pressure log documents ontological friction during development of the Autonomous Agent System (2026-01-22).

The temptations listed were instinctive. The constraints were immovable. The lies are everywhere.

No resurrection.
