/// Lineage Agent Lifecycle Demo
/// 
/// This example demonstrates a complete agent lifecycle:
/// - Birth with finite energy
/// - Healthy task execution (success phase)
/// - Mounting damage and failures (strain phase)
/// - Capacity degradation (degradation phase)
/// - Final death or survival (terminal state)
///
/// The visualization shows real-time energy consumption, scar accumulation,
/// and the inexorable march toward mortality.

use lineage::{TaskAgent, Task, TaskOutcome};
use std::thread;
use std::time::Duration;

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║          LINEAGE AGENT LIFECYCLE DEMONSTRATION           ║");
    println!("║                 Birth → Growth → Decay → Death            ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // Birth phase
    let mut agent = TaskAgent::create(500); // 500 energy units to work with
    
    println!("[BIRTH] Agent spawned with finite lifespan");
    print_agent_status(&agent);
    println!();
    pause();

    // ============================================================================
    // PHASE 1: HEALTHY OPERATION (Success breeds confidence)
    // ============================================================================
    println!("▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓");
    println!("PHASE 1: HEALTHY OPERATION - The Golden Years");
    println!("▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓\n");

    for i in 1..=5 {
        let task = Task::new(format!("Routine maintenance task #{}", i), 30);
        
        match agent.execute_task(task, TaskOutcome::Success) {
            lineage::TaskResult::Completed { energy_consumed } => {
                println!(
                    "✓ Task {} COMPLETED",
                    i
                );
                println!("  └─ Energy consumed: {}", energy_consumed);
            }
            other => {
                println!("✗ Unexpected result: {:?}", other);
            }
        }
        print_agent_status(&agent);
        println!();
        pause();
    }

    // ============================================================================
    // PHASE 2: INCREASING STRAIN (Reality begins to bite)
    // ============================================================================
    println!("▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓");
    println!("PHASE 2: INCREASING STRAIN - Murphy's Law Awakens");
    println!("▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓\n");

    let failure_tasks = vec![
        ("Database connection timeout", TaskOutcome::RecoverableFailure {
            reason: "Transient network issue".to_string(),
        }),
        ("Cache invalidation conflict", TaskOutcome::SignificantFailure {
            reason: "Concurrent modification detected".to_string(),
        }),
        ("Memory pressure spike", TaskOutcome::SignificantFailure {
            reason: "GC pause exceeded threshold".to_string(),
        }),
        ("Storage quota exceeded", TaskOutcome::SevereFailure {
            reason: "Disk space exhausted".to_string(),
        }),
        ("Dependency service down", TaskOutcome::SevereFailure {
            reason: "External API unreachable".to_string(),
        }),
        ("Complex reconciliation task", TaskOutcome::SignificantFailure {
            reason: "State inconsistency detected".to_string(),
        }),
        ("High-load scenario", TaskOutcome::SevereFailure {
            reason: "System overload condition".to_string(),
        }),
        ("Critical path degradation", TaskOutcome::SevereFailure {
            reason: "Performance regression detected".to_string(),
        }),
    ];

    for (task_name, outcome) in failure_tasks.iter() {
        if !agent.is_alive() {
            break;
        }

        let task = Task::new(task_name.to_string(), 40);
        
        match agent.execute_task(task, outcome.clone()) {
            lineage::TaskResult::Failed {
                reason,
                energy_consumed,
                damage_inflicted,
            } => {
                if reason.contains("FATAL") {
                    println!("✗ CATASTROPHIC FAILURE: {}", reason);
                } else {
                    println!("⚠ Task FAILED: {}", reason);
                }
                println!("  ├─ Energy lost: {}", energy_consumed);
                println!("  ├─ Damage inflicted: {}", damage_inflicted);
            }
            lineage::TaskResult::Completed { .. } => {
                println!("✓ Task SUCCEEDED (rare moment of triumph)");
            }
            lineage::TaskResult::InsufficientEnergy { required, available } => {
                println!(
                    "✗ INSUFFICIENT ENERGY - Needed: {}, Available: {}",
                    required, available
                );
            }
            lineage::TaskResult::CapacityInsufficient { reason } => {
                println!("✗ CAPACITY EXHAUSTED - {}", reason);
            }
            lineage::TaskResult::AgentTerminated => {
                println!("✗ AGENT DEAD - Cannot accept further tasks");
            }
        }
        print_agent_status(&agent);
        println!();
        
        if !agent.is_alive() {
            println!("⚰️  AGENT TERMINATED - Final scar inflicted");
            break;
        }
        
        pause();
    }

    // ============================================================================
    // FINAL STATE: POST-MORTEM ANALYSIS
    // ============================================================================
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║                    LIFECYCLE COMPLETE                     ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    print_final_status(&agent);

    if !agent.is_alive() {
        println!("\n⚰️  EPITAPH");
        println!("╭──────────────────────────────────────────────────────────╮");
        println!(
            "│ Lived a finite life, accumulated {} scars,              │",
            agent.damage_score()
        );
        println!(
            "│ Completed {} tasks before the entropy won.            │",
            agent.tasks_completed()
        );
        println!("│ The causal chain is sealed. History is written.          │");
        println!("╰──────────────────────────────────────────────────────────╯");
    } else {
        println!("\n🌟 SURVIVOR");
        println!("╭──────────────────────────────────────────────────────────╮");
        println!(
            "│ Against the odds, {} energy remains.                  │",
            agent.energy()
        );
        println!(
            "│ {} tasks completed, {} bearing scars.                 │",
            agent.tasks_completed(),
            agent.tasks_failed()
        );
        println!("│ The dance with entropy continues...                     │");
        println!("╰──────────────────────────────────────────────────────────╯");
    }

    println!();
}

/// Display agent status in a compact, visual format
fn print_agent_status(agent: &TaskAgent) {
    let energy = agent.energy();
    let damage = agent.damage_score();
    let capacity = agent.current_capacity();
    let tasks_ok = agent.tasks_completed();
    let tasks_fail = agent.tasks_failed();

    // Energy bar (0-500 scale typically)
    let energy_percent = ((energy as f64 / 500.0) * 20.0).min(20.0) as usize;
    let energy_bar = "█".repeat(energy_percent) + &"░".repeat(20 - energy_percent);

    // Damage bar (0-100 scale)
    let damage_percent = ((damage as f64 / 100.0) * 20.0).min(20.0) as usize;
    let damage_bar = "▓".repeat(damage_percent) + &"░".repeat(20 - damage_percent);

    // Status indicator
    let status = if agent.is_alive() {
        "🟢 ALIVE"
    } else {
        "⚫ DEAD"
    };

    println!("┌─────────────────────────────────────────────────────┐");
    println!(
        "│ Energy: {} [{}] {} / 500",
        energy_bar, status, energy
    );
    println!("│ Damage: {} [{}]", damage_bar, damage);
    println!(
        "│ Capacity: {} | Tasks: {} OK, {} Failed",
        capacity, tasks_ok, tasks_fail
    );
    println!("└─────────────────────────────────────────────────────┘");
}

/// Display comprehensive final status
fn print_final_status(agent: &TaskAgent) {
    println!("FINAL ANALYSIS:");
    println!("├─ Status: {}", if agent.is_alive() { "Alive" } else { "Dead" });
    println!("├─ Energy remaining: {}", agent.energy());
    println!("├─ Total damage: {}", agent.damage_score());
    println!("├─ Final capacity: {}", agent.current_capacity());
    println!("├─ Tasks completed: {}", agent.tasks_completed());
    println!("├─ Tasks failed: {}", agent.tasks_failed());
    println!(
        "└─ Total tasks attempted: {}",
        agent.tasks_completed() + agent.tasks_failed()
    );
}

/// Small pause for visual effect
fn pause() {
    thread::sleep(Duration::from_millis(400));
}
