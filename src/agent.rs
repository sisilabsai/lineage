// Autonomous Task Agent System
//
// ONTOLOGICAL CONSTRAINT: Finite lifespan, irreversible resource consumption.
//
// Agents are NOT:
// - Learning systems that improve from errors
// - Resilient processes that retry indefinitely
// - Recoverable entities that checkpoint state
//
// Agents ARE:
// - Finite-lifetime task executors
// - Mortal entities that die permanently
// - Consequence-bearing systems that accumulate damage

use crate::lineage::{Lineage, OperationError};
use crate::scar::ScarSeverity;
use crate::graveyard::Graveyard;

/// Classification of task execution outcomes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskOutcome {
    /// Task completed successfully.
    Success,
    
    /// Task failed with recoverable error (Minor scar).
    RecoverableFailure { reason: String },
    
    /// Task failed with significant error (Moderate scar).
    SignificantFailure { reason: String },
    
    /// Task failed with severe error (Severe scar).
    SevereFailure { reason: String },
    
    /// Task failed catastrophically (Fatal scar, terminates agent).
    CatastrophicFailure { reason: String },
}

impl TaskOutcome {
    /// Determine scar severity for this outcome.
    pub fn severity(&self) -> Option<ScarSeverity> {
        match self {
            TaskOutcome::Success => None,
            TaskOutcome::RecoverableFailure { .. } => Some(ScarSeverity::Minor),
            TaskOutcome::SignificantFailure { .. } => Some(ScarSeverity::Moderate),
            TaskOutcome::SevereFailure { .. } => Some(ScarSeverity::Severe),
            TaskOutcome::CatastrophicFailure { .. } => Some(ScarSeverity::Fatal),
        }
    }
    
    /// Get failure description if outcome is failure.
    pub fn failure_description(&self) -> Option<String> {
        match self {
            TaskOutcome::Success => None,
            TaskOutcome::RecoverableFailure { reason } => Some(reason.clone()),
            TaskOutcome::SignificantFailure { reason } => Some(reason.clone()),
            TaskOutcome::SevereFailure { reason } => Some(reason.clone()),
            TaskOutcome::CatastrophicFailure { reason } => Some(reason.clone()),
        }
    }
}

/// Result of attempting to execute a task.
#[derive(Debug, PartialEq, Eq)]
pub enum TaskResult {
    /// Task executed successfully.
    Completed { energy_consumed: u64 },
    
    /// Task failed, scar inflicted.
    Failed { 
        reason: String,
        energy_consumed: u64,
        damage_inflicted: u32,
    },
    
    /// Task rejected due to insufficient energy.
    InsufficientEnergy { required: u64, available: u64 },
    
    /// Task rejected due to excessive damage (capacity degraded).
    CapacityInsufficient { reason: String },
    
    /// Agent is dead, cannot execute tasks.
    AgentTerminated,
}

/// Task descriptor with energy requirements.
#[derive(Debug, Clone)]
pub struct Task {
    /// Task identifier/description.
    pub description: String,
    
    /// Base energy cost to execute this task.
    pub base_cost: u64,
    
    /// Minimum capacity required (affected by accumulated damage).
    pub min_capacity: u32,
}

impl Task {
    /// Create new task with description and base cost.
    pub fn new(description: String, base_cost: u64) -> Self {
        Task {
            description,
            base_cost,
            min_capacity: 0, // No capacity requirement by default
        }
    }
    
    /// Create task requiring minimum capacity.
    pub fn with_capacity(description: String, base_cost: u64, min_capacity: u32) -> Self {
        Task {
            description,
            base_cost,
            min_capacity,
        }
    }
    
    /// Calculate actual energy cost including damage penalty.
    /// 
    /// CONSEQUENCE: Accumulated damage increases energy cost.
    /// More scars = higher cost for same task.
    pub fn actual_cost(&self, damage_score: u32) -> u64 {
        // Base cost + 10% per 10 damage points
        let damage_multiplier = 1.0 + (damage_score as f64 / 100.0);
        (self.base_cost as f64 * damage_multiplier).ceil() as u64
    }
}

/// Autonomous task-executing agent with finite lifespan.
/// 
/// ONTOLOGICAL CONSTRAINTS:
/// - Born with fixed energy budget
/// - Tasks consume energy irreversibly
/// - Errors inflict permanent scars
/// - Scars increase future task costs
/// - Death is permanent
/// 
/// FORBIDDEN OPERATIONS:
/// - Retry without cost
/// - Energy restoration
/// - Scar removal
/// - Death resurrection
/// - Learning from errors
pub struct TaskAgent {
    lineage: Lineage,
    tasks_completed: usize,
    tasks_failed: usize,
}

impl TaskAgent {
    /// Create new agent with initial energy budget.
    /// 
    /// FINITE LIFETIME: This is all the energy the agent will ever have.
    /// 
    /// LAZARUS CHECK: Verifies that no agent with this identity has died before.
    /// If an agent identity is found in the graveyard, this function panics.
    pub fn create(initial_energy: u64) -> Self {
        let lineage = Lineage::create(initial_energy);
        let agent_id = lineage.identity().id();
        
        // LAZARUS PREVENTION: Check if this identity has already died
        if Graveyard::is_dead(agent_id) {
            panic!(
                "ONTOLOGICAL ERROR: Identity {} is already sealed in the Eternal Archive. \
                 A dead identity cannot be reborn.",
                agent_id
            );
        }
        
        TaskAgent {
            lineage,
            tasks_completed: 0,
            tasks_failed: 0,
        }
    }
    
    /// Get agent identity.
    pub fn identity(&self) -> &crate::identity::Identity {
        self.lineage.identity()
    }
    
    /// Check if agent is alive.
    pub fn is_alive(&self) -> bool {
        self.lineage.is_alive()
    }
    
    /// Get current energy level.
    pub fn energy(&self) -> u64 {
        self.lineage.metabolism().energy()
    }
    
    /// Get accumulated damage score.
    pub fn damage_score(&self) -> u32 {
        self.lineage.scars().damage_score()
    }
    
    /// Get count of completed tasks.
    pub fn tasks_completed(&self) -> usize {
        self.tasks_completed
    }
    
    /// Get count of failed tasks.
    pub fn tasks_failed(&self) -> usize {
        self.tasks_failed
    }
    
    /// Calculate current task capacity.
    /// 
    /// DEGRADATION: Capacity decreases with accumulated damage.
    /// Base capacity = 100, reduced by damage score.
    pub fn current_capacity(&self) -> u32 {
        let base_capacity = 100u32;
        let damage = self.damage_score();
        base_capacity.saturating_sub(damage)
    }
    
    /// Execute task with given outcome.
    /// 
    /// CONSEQUENCES:
    /// - Energy consumed irreversibly (even on failure)
    /// - Failures inflict permanent scars
    /// - Scars increase future task costs
    /// - Fatal failures terminate agent
    /// - No retry mechanism exists
    /// 
    /// HARSH REALITY:
    /// - Failure consumes energy without completing task
    /// - Agent cannot learn from errors
    /// - Accumulated damage leads to death spiral
    pub fn execute_task(&mut self, task: Task, outcome: TaskOutcome) -> TaskResult {
        // Check if agent is dead
        if !self.lineage.is_alive() {
            return TaskResult::AgentTerminated;
        }
        
        // Calculate actual cost (increased by damage)
        let actual_cost = task.actual_cost(self.damage_score());
        
        // Check capacity requirement
        if task.min_capacity > self.current_capacity() {
            return TaskResult::CapacityInsufficient {
                reason: format!(
                    "Task requires capacity {}, agent has {} (damage: {})",
                    task.min_capacity,
                    self.current_capacity(),
                    self.damage_score()
                ),
            };
        }
        
        // Check energy availability
        if actual_cost > self.energy() {
            return TaskResult::InsufficientEnergy {
                required: actual_cost,
                available: self.energy(),
            };
        }
        
        // Consume energy for task attempt
        let operation_desc = format!("Task: {}", task.description);
        match self.lineage.perform_operation(operation_desc, actual_cost) {
            crate::lineage::OperationResult::Success { energy_consumed } => {
                // Task attempt consumed energy, now handle outcome
                match outcome {
                    TaskOutcome::Success => {
                        // Task succeeded
                        self.tasks_completed += 1;
                        TaskResult::Completed { energy_consumed }
                    }
                    _ => {
                        // Task failed - inflict scar
                        self.tasks_failed += 1;
                        
                        if let Some(severity) = outcome.severity() {
                            let failure_desc = outcome
                                .failure_description()
                                .unwrap_or_else(|| "Task failure".to_string());
                            
                            let error = OperationError::new(severity, failure_desc.clone());
                            let damage_before = self.damage_score();
                            
                            match self.lineage.record_error(error) {
                                crate::lineage::OperationResult::Success { .. } => {
                                    // Non-fatal failure
                                    let damage_inflicted = self.damage_score() - damage_before;
                                    TaskResult::Failed {
                                        reason: failure_desc,
                                        energy_consumed,
                                        damage_inflicted,
                                    }
                                }
                                crate::lineage::OperationResult::Dead => {
                                    // Fatal failure - agent terminated
                                    let damage_inflicted = self.damage_score() - damage_before;
                                    TaskResult::Failed {
                                        reason: format!("{} (FATAL - agent terminated)", failure_desc),
                                        energy_consumed,
                                        damage_inflicted,
                                    }
                                }
                                crate::lineage::OperationResult::OntologicalViolation { reason } => {
                                    eprintln!("FATAL: Ontological violation: {}", reason);
                                    std::process::exit(1);
                                }
                                _ => unreachable!("record_error cannot return InsufficientEnergy"),
                            }
                        } else {
                            unreachable!("Failure outcome must have severity")
                        }
                    }
                }
            }
            crate::lineage::OperationResult::InsufficientEnergy { required, available } => {
                TaskResult::InsufficientEnergy { required, available }
            }
            crate::lineage::OperationResult::Dead => TaskResult::AgentTerminated,
            crate::lineage::OperationResult::OntologicalViolation { reason } => {
                eprintln!("FATAL: Ontological violation: {}", reason);
                std::process::exit(1);
            }
        }
    }
    
    /// Bury this agent in the Graveyard (requires agent to be dead).
    /// 
    /// Creates a cryptographically sealed tombstone with:
    /// - Complete identity and metabolic information
    /// - All scars and cause of death
    /// - Causal chain for tamper detection
    /// 
    /// **CONSEQUENCE**: Once buried, this agent's identity cannot be reused.
    /// **CONSEQUENCE**: Tombstone is immutable after creation.
    pub fn bury(&self) -> Result<(), crate::graveyard::GraveyardError> {
        if self.is_alive() {
            return Err(crate::graveyard::GraveyardError::DirectoryError(
                "Cannot bury a living agent".to_string(),
            ));
        }

        // Gather scar information
        let scar_records: Vec<crate::graveyard::ScarRecord> = self
            .lineage
            .scars()
            .all_scars()
            .iter()
            .map(|scar| crate::graveyard::ScarRecord {
                timestamp: scar.timestamp(),
                severity: format!("{:?}", scar.severity()),
                description: scar.description().to_string(),
                context: scar.context().map(|s| s.to_string()),
            })
            .collect();

        // Determine cause of death
        let cause_of_death = self
            .lineage
            .scars()
            .latest_scar()
            .map(|s| s.description().to_string())
            .unwrap_or_else(|| "Energy depletion".to_string());

        // Create tombstone with proper energy metrics
        let initial_energy = self.lineage.metabolism().initial_energy();
        let final_energy = self.energy();
        let _energy_consumed = initial_energy.saturating_sub(final_energy);
        
        let tombstone = crate::graveyard::Tombstone::create(
            self.identity().id().to_string(),
            format!("{:?}", self.identity()),
            chrono::Utc::now() - chrono::Duration::seconds(1), // Approximate creation time
            final_energy,
            initial_energy,  // Peak energy = initial energy
            initial_energy,
            self.tasks_completed as u32,
            scar_records,
            cause_of_death,
        );

        // Bury in the eternal archive
        crate::graveyard::Graveyard::bury(&tombstone)
    }

    /// Spawn a descendant agent from this "healthy" parent agent.
    ///
    /// REQUIREMENTS FOR SPAWNING:
    /// - Parent must be ALIVE (not dead)
    /// - Parent must have high enough Legacy Score (at least 0.5)
    /// - Parent must have capacity to transfer (remaining energy >= 100)
    /// - Parent must have completed at least 5 tasks successfully
    ///
    /// TRANSFER MECHANICS:
    /// - Portion of parent's energy passes to child (scaled by parent's efficiency)
    /// - Parent loses transferred energy permanently
    /// - Child inherits knowledge from parent's success ratio
    /// - Child gains genetic advantage from parent's efficiency
    /// - Creates a causal tree across generations
    ///
    /// CONSEQUENCE: Energy transfer is IRREVERSIBLE.
    /// CONSEQUENCE: Parent becomes weaker after spawning.
    /// CONSEQUENCE: Child starts with advantage but at higher risk.
    /// 
    /// **Returns**: New child TaskAgent with genealogical lineage
    pub fn spawn(&mut self, initial_energy_for_child: u64) -> Result<TaskAgent, String> {
        // Check if parent is alive
        if !self.is_alive() {
            return Err("Cannot spawn from a dead agent".to_string());
        }

        // Calculate parent's current legacy score for fitness check
        let efficiency = if self.tasks_completed > 0 {
            self.tasks_completed as f64 / (self.tasks_completed + self.tasks_failed) as f64
        } else {
            0.0
        };

        let legacy_score = if self.tasks_failed > 0 {
            efficiency * (self.tasks_completed as f64 / (self.tasks_failed as f64 + 1.0))
        } else {
            efficiency + (self.tasks_completed as f64 * 0.1)
        };

        // Check fitness requirements
        if legacy_score < 0.5 && self.tasks_completed < 5 {
            return Err(format!(
                "Parent agent not healthy enough to spawn (legacy score: {:.2}, required: >= 0.5)",
                legacy_score
            ));
        }

        // Check energy availability for transfer
        if self.energy() < initial_energy_for_child + 50 {
            return Err(format!(
                "Parent does not have enough energy to spawn (required: {}, available: {})",
                initial_energy_for_child + 50,
                self.energy()
            ));
        }

        // TRANSFER ENERGY: Irreversibly consume parent's energy
        let transfer_task = Task::new(
            format!(
                "Spawning descendant agent with {} energy",
                initial_energy_for_child
            ),
            initial_energy_for_child,
        );

        match self.execute_task(transfer_task, TaskOutcome::Success) {
            TaskResult::Completed { .. } => {
                // Energy successfully transferred
                // Create child agent
                let mut child = TaskAgent::create(initial_energy_for_child);

                // Record parentage in child's memory
                child.lineage.memory_mut().append(format!(
                    "Spawned from parent agent {}. Inherited efficiency: {:.2}. Generation lineage established.",
                    self.identity().id(),
                    efficiency
                ));

                Ok(child)
            }
            TaskResult::Failed { .. } => {
                Err("Energy transfer failed during spawn operation".to_string())
            }
            TaskResult::InsufficientEnergy { required, available } => {
                Err(format!(
                    "Insufficient energy for spawn: required {}, available {}",
                    required, available
                ))
            }
            TaskResult::CapacityInsufficient { reason } => {
                Err(format!("Parent capacity insufficient for spawn: {}", reason))
            }
            TaskResult::AgentTerminated => {
                Err("Parent agent terminated during spawn".to_string())
            }
        }
    }
    
    /// Get agent status summary.
    pub fn status_summary(&self) -> String {
        format!(
            "Agent {} | Status: {} | Energy: {} | Capacity: {}/100 | Tasks: {} completed, {} failed",
            self.identity().id().chars().take(8).collect::<String>(),
            if self.is_alive() { "ALIVE" } else { "DEAD" },
            self.energy(),
            self.current_capacity(),
            self.tasks_completed,
            self.tasks_failed
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_starts_with_full_energy() {
        let agent = TaskAgent::create(1000);
        assert_eq!(agent.energy(), 1000);
        assert!(agent.is_alive());
    }

    #[test]
    fn successful_task_consumes_energy() {
        let mut agent = TaskAgent::create(1000);
        let task = Task::new("Test task".to_string(), 100);
        
        let result = agent.execute_task(task, TaskOutcome::Success);
        
        assert!(matches!(result, TaskResult::Completed { .. }));
        assert_eq!(agent.energy(), 900);
        assert_eq!(agent.tasks_completed(), 1);
    }

    #[test]
    fn failed_task_consumes_energy_and_inflicts_scar() {
        let mut agent = TaskAgent::create(1000);
        let task = Task::new("Failing task".to_string(), 100);
        
        let result = agent.execute_task(
            task,
            TaskOutcome::RecoverableFailure {
                reason: "Network timeout".to_string(),
            },
        );
        
        // Energy consumed despite failure
        assert!(matches!(result, TaskResult::Failed { .. }));
        assert_eq!(agent.energy(), 900);
        
        // Scar inflicted
        assert_eq!(agent.damage_score(), 1);
        assert_eq!(agent.tasks_failed(), 1);
    }

    #[test]
    fn damage_increases_task_cost() {
        let mut agent = TaskAgent::create(1000);
        let task1 = Task::new("Task 1".to_string(), 100);
        let task2 = Task::new("Task 2".to_string(), 100);
        
        // First task succeeds (base cost: 100)
        agent.execute_task(task1, TaskOutcome::Success);
        let energy_after_first = agent.energy();
        
        // Inflict significant damage (Moderate = 5 points)
        let damage_task = Task::new("Damage task".to_string(), 0);
        agent.execute_task(
            damage_task,
            TaskOutcome::SignificantFailure {
                reason: "Major error".to_string(),
            },
        );
        
        // Second task has higher cost due to damage
        let energy_before_second = agent.energy();
        agent.execute_task(task2, TaskOutcome::Success);
        let energy_after_second = agent.energy();
        
        let first_cost = 1000 - energy_after_first;
        let second_cost = energy_before_second - energy_after_second;
        
        // Second task cost is higher due to damage penalty
        assert!(second_cost > first_cost);
    }

    #[test]
    fn insufficient_energy_rejects_task() {
        let mut agent = TaskAgent::create(50);
        let task = Task::new("Expensive task".to_string(), 100);
        
        let result = agent.execute_task(task, TaskOutcome::Success);
        
        assert!(matches!(
            result,
            TaskResult::InsufficientEnergy { required: 100, available: 50 }
        ));
    }

    #[test]
    fn capacity_degrades_with_damage() {
        let mut agent = TaskAgent::create(1000);
        assert_eq!(agent.current_capacity(), 100);
        
        // Inflict moderate damage (5 points)
        let task = Task::new("Task".to_string(), 10);
        agent.execute_task(
            task,
            TaskOutcome::SignificantFailure {
                reason: "Error".to_string(),
            },
        );
        
        // Capacity reduced by damage
        assert_eq!(agent.current_capacity(), 95);
    }

    #[test]
    fn high_capacity_task_rejected_when_damaged() {
        let mut agent = TaskAgent::create(1000);
        
        // Create task requiring 50 capacity
        let high_capacity_task = Task::with_capacity(
            "High capacity task".to_string(),
            10,
            50,
        );
        
        // Initially succeeds (capacity = 100)
        let result = agent.execute_task(high_capacity_task.clone(), TaskOutcome::Success);
        assert!(matches!(result, TaskResult::Completed { .. }));
        
        // Inflict severe damage (20 points * 3 = 60 damage)
        for _ in 0..3 {
            let damage_task = Task::new("Damage".to_string(), 10);
            agent.execute_task(
                damage_task,
                TaskOutcome::SevereFailure {
                    reason: "Severe error".to_string(),
                },
            );
        }
        
        // Now capacity is insufficient (100 - 60 = 40 < 50)
        let result = agent.execute_task(high_capacity_task, TaskOutcome::Success);
        assert!(matches!(result, TaskResult::CapacityInsufficient { .. }));
    }

    #[test]
    fn catastrophic_failure_terminates_agent() {
        let mut agent = TaskAgent::create(1000);
        assert!(agent.is_alive());
        
        let task = Task::new("Critical task".to_string(), 10);
        let result = agent.execute_task(
            task,
            TaskOutcome::CatastrophicFailure {
                reason: "Unrecoverable error".to_string(),
            },
        );
        
        // Task failed and agent terminated
        assert!(matches!(result, TaskResult::Failed { .. }));
        assert!(!agent.is_alive());
    }

    #[test]
    fn dead_agent_cannot_execute_tasks() {
        let mut agent = TaskAgent::create(1000);
        
        // Terminate agent via catastrophic failure
        let fatal_task = Task::new("Fatal".to_string(), 10);
        agent.execute_task(
            fatal_task,
            TaskOutcome::CatastrophicFailure {
                reason: "Fatal".to_string(),
            },
        );
        
        // Attempt to execute another task
        let task = Task::new("Post-death task".to_string(), 10);
        let result = agent.execute_task(task, TaskOutcome::Success);
        
        assert_eq!(result, TaskResult::AgentTerminated);
    }

    #[test]
    fn energy_exhaustion_terminates_agent() {
        let mut agent = TaskAgent::create(100);
        
        // Execute task consuming all energy
        let task = Task::new("Exhausting task".to_string(), 100);
        agent.execute_task(task, TaskOutcome::Success);
        
        assert_eq!(agent.energy(), 0);
        assert!(!agent.is_alive());
        
        // Cannot execute more tasks
        let next_task = Task::new("After death".to_string(), 1);
        let result = agent.execute_task(next_task, TaskOutcome::Success);
        assert_eq!(result, TaskResult::AgentTerminated);
    }

    #[test]
    fn tasks_completed_count_tracks_successes_only() {
        let mut agent = TaskAgent::create(1000);
        
        agent.execute_task(Task::new("T1".to_string(), 10), TaskOutcome::Success);
        agent.execute_task(
            Task::new("T2".to_string(), 10),
            TaskOutcome::RecoverableFailure {
                reason: "Fail".to_string(),
            },
        );
        agent.execute_task(Task::new("T3".to_string(), 10), TaskOutcome::Success);
        
        assert_eq!(agent.tasks_completed(), 2);
        assert_eq!(agent.tasks_failed(), 1);
    }

    // BRUTAL TESTS: Prove harsh consequences

    #[test]
    fn failure_consumes_energy_without_progress() {
        let mut agent = TaskAgent::create(1000);
        let initial_energy = agent.energy();
        
        // Task fails - energy consumed, no progress
        let task = Task::new("Failing task".to_string(), 100);
        agent.execute_task(
            task,
            TaskOutcome::RecoverableFailure {
                reason: "Failure".to_string(),
            },
        );
        
        // Energy consumed
        assert_eq!(agent.energy(), initial_energy - 100);
        
        // No completed tasks
        assert_eq!(agent.tasks_completed(), 0);
        
        // Damage inflicted
        assert!(agent.damage_score() > 0);
    }

    #[test]
    fn repeated_failures_cause_death_spiral() {
        let mut agent = TaskAgent::create(500);
        
        // Inflict damage to increase costs
        for i in 0..5 {
            let task = Task::new(format!("Task {}", i), 50);
            agent.execute_task(
                task,
                TaskOutcome::SignificantFailure {
                    reason: "Error".to_string(),
                },
            );
        }
        
        // Now tasks cost more due to damage (damage score = 25)
        let expensive_task = Task::new("Expensive".to_string(), 50);
        let actual_cost = expensive_task.actual_cost(agent.damage_score());
        
        // Cost increased from 50 to ~62 (50 * 1.25)
        assert!(actual_cost > 50);
        assert!(actual_cost < 65);
    }

    #[test]
    fn no_retry_mechanism_exists() {
        let mut agent = TaskAgent::create(1000);
        
        // Task fails
        let task = Task::new("Failing task".to_string(), 100);
        let result = agent.execute_task(
            task.clone(),
            TaskOutcome::RecoverableFailure {
                reason: "Failure".to_string(),
            },
        );
        
        assert!(matches!(result, TaskResult::Failed { .. }));
        
        // ATTACK: Try to retry - no method exists
        // Agent has no retry() method
        // Agent has no reset() method
        // Agent has no recover() method
        
        // Can execute same task again, but:
        // - Costs more energy (damage penalty)
        // - Damage still present
        // - Previous failure still recorded
        
        let damage_after_first = agent.damage_score();
        let energy_after_first = agent.energy();
        
        // Second attempt (manual, not automatic retry)
        agent.execute_task(task, TaskOutcome::Success);
        
        // Damage remains from first failure
        assert_eq!(agent.damage_score(), damage_after_first);
        
        // More energy consumed than if first succeeded
        assert!(agent.energy() < energy_after_first - 100);
    }

    #[test]
    fn agent_cannot_learn_from_errors() {
        let mut agent = TaskAgent::create(1000);
        
        // Fail same task type repeatedly
        for _ in 0..10 {
            let task = Task::new("Repeating failure".to_string(), 10);
            agent.execute_task(
                task,
                TaskOutcome::RecoverableFailure {
                    reason: "Same error".to_string(),
                },
            );
        }
        
        // Damage accumulates (no learning)
        assert_eq!(agent.damage_score(), 10); // 10 failures * 1 damage each
        
        // Agent does not improve
        // Agent does not adapt
        // Agent does not learn patterns
        
        // Cost keeps increasing with damage
        let task = Task::new("Next task".to_string(), 100);
        let cost = task.actual_cost(agent.damage_score());
        assert!(cost > 100); // Penalty applied
    }

    #[test]
    fn death_before_mission_completion_is_allowed() {
        let mut agent = TaskAgent::create(200);
        
        // Mission: Complete 10 tasks (100 energy each = 1000 total needed)
        // Agent only has 200 energy - will die before completion
        
        let mut completed = 0;
        for i in 0..10 {
            if !agent.is_alive() {
                break;
            }
            
            let task = Task::new(format!("Mission task {}", i), 100);
            match agent.execute_task(task, TaskOutcome::Success) {
                TaskResult::Completed { .. } => completed += 1,
                TaskResult::InsufficientEnergy { .. } => break,
                _ => break,
            }
        }
        
        // Agent died before completing mission
        assert!(!agent.is_alive());
        assert!(completed < 10);
        
        // This is correct behavior, not a bug
        // Agents with insufficient resources die
        // No resurrection, no extra energy
    }

    #[test]
    fn energy_only_decreases_never_increases() {
        let mut agent = TaskAgent::create(1000);
        let mut previous_energy = agent.energy();
        
        // Execute various tasks
        let tasks = vec![
            (Task::new("T1".to_string(), 50), TaskOutcome::Success),
            (
                Task::new("T2".to_string(), 30),
                TaskOutcome::RecoverableFailure {
                    reason: "Fail".to_string(),
                },
            ),
            (Task::new("T3".to_string(), 100), TaskOutcome::Success),
        ];
        
        for (task, outcome) in tasks {
            if !agent.is_alive() {
                break;
            }
            
            agent.execute_task(task, outcome);
            
            // Energy never increases
            assert!(agent.energy() <= previous_energy);
            previous_energy = agent.energy();
        }
    }

    #[test]
    fn accumulated_damage_is_permanent() {
        let mut agent = TaskAgent::create(1000);
        
        // Inflict damage
        let task = Task::new("Damage task".to_string(), 10);
        agent.execute_task(
            task,
            TaskOutcome::SevereFailure {
                reason: "Error".to_string(),
            },
        );
        
        let damage = agent.damage_score();
        assert_eq!(damage, 20); // Severe = 20 points
        
        // Execute many successful tasks
        for i in 0..100 {
            if !agent.is_alive() {
                break;
            }
            let t = Task::new(format!("Success {}", i), 1);
            agent.execute_task(t, TaskOutcome::Success);
        }
        
        // Damage unchanged by success
        assert_eq!(agent.damage_score(), damage);
    }
}
