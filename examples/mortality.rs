/// Lineage Agent - Inevitable Mortality Demo
/// 
/// This variant demonstrates the harsh reality: every agent eventually dies.
/// This version intensifies the strain to guarantee a complete lifecycle,
/// showing the irreversible march from life to death and the sealed causal chain.

use lineage::{TaskAgent, Task, TaskOutcome};

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║           LINEAGE: ENTROPY AND INEVITABLE DEATH          ║");
    println!("║              Watch an agent face mortality                 ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // Birth phase - smaller initial energy to ensure termination
    let mut agent = TaskAgent::create(300);
    
    println!("[INITIALIZATION] Agent spawned");
    print_status(&agent);
    println!();

    // Early successes build false hope
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("EARLY OPTIMISM: First tasks go smoothly");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let early_tasks = vec![
        Task::new("Initialization".to_string(), 25),
        Task::new("Configuration setup".to_string(), 25),
        Task::new("Service discovery".to_string(), 30),
    ];

    for task in early_tasks {
        if !agent.is_alive() {
            break;
        }
        match agent.execute_task(task.clone(), TaskOutcome::Success) {
            lineage::TaskResult::Completed { .. } => {
                println!("✓ {}: SUCCESS", task.description);
            }
            _ => {
                println!("✗ {}: UNEXPECTED RESULT", task.description);
            }
        }
    }
    print_status(&agent);
    println!();

    // Things begin to go wrong
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("REALITY SETS IN: Cascading failures begin");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let failure_scenarios = vec![
        ("Network timeout", TaskOutcome::RecoverableFailure { reason: "Retry-able error".to_string() }),
        ("Memory leak detected", TaskOutcome::SignificantFailure { reason: "Resource exhaustion beginning".to_string() }),
        ("GC pause critical", TaskOutcome::SevereFailure { reason: "Pause time exceeded limits".to_string() }),
        ("Cascading failure", TaskOutcome::SevereFailure { reason: "System becoming unstable".to_string() }),
        ("Core service down", TaskOutcome::SevereFailure { reason: "Dependency failure".to_string() }),
        ("State corruption", TaskOutcome::SevereFailure { reason: "Data integrity compromised".to_string() }),
        ("Recovery attempt 1", TaskOutcome::SignificantFailure { reason: "Recovery failed".to_string() }),
        ("Recovery attempt 2", TaskOutcome::SignificantFailure { reason: "Damage too severe".to_string() }),
        ("Critical threshold", TaskOutcome::SevereFailure { reason: "System integrity failing".to_string() }),
        ("Final degradation", TaskOutcome::CatastrophicFailure { reason: "Irreversible damage - system terminating".to_string() }),
    ];

    let mut phase = 1;
    for (scenario_name, outcome) in failure_scenarios {
        if !agent.is_alive() {
            println!("\n⚰️  AGENT EXPIRED - No further processing possible\n");
            break;
        }

        // Increase difficulty as we go - cost grows rapidly
        let cost = 40 + (phase as u64 * 15);
        let task = Task::new(scenario_name.to_string(), cost);

        match agent.execute_task(task, outcome.clone()) {
            lineage::TaskResult::Failed { reason, energy_consumed, damage_inflicted } => {
                let severity_indicator = if reason.contains("FATAL") { "🔥 CATASTROPHIC" } else { "⚠️ FAILURE" };
                println!("{} - {} [Energy: {}, Damage: {}]", 
                    severity_indicator, reason, energy_consumed, damage_inflicted);
            }
            lineage::TaskResult::Completed { .. } => {
                println!("✓ {} - SUCCEEDED against odds", scenario_name);
            }
            lineage::TaskResult::InsufficientEnergy { available, required } => {
                println!("✗ {} - INSUFFICIENT ENERGY (need {}, have {})", 
                    scenario_name, required, available);
                break;
            }
            lineage::TaskResult::CapacityInsufficient { reason } => {
                println!("✗ {} - {}", scenario_name, reason);
            }
            lineage::TaskResult::AgentTerminated => {
                println!("✗ {} - AGENT ALREADY DEAD", scenario_name);
                break;
            }
        }

        print_status(&agent);
        println!();
        phase += 1;
    }

    // Post-mortem
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║                      END OF LIFE REPORT                   ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    print_final_report(&agent);

    if agent.is_alive() {
        println!("\n🔴 UNUSUAL SURVIVAL - Agent persists despite injuries");
        println!("   It limps onward with {} energy and {} accumulated damage", 
            agent.energy(), agent.damage_score());
    } else {
        println!("\n⚫ PERMANENT TERMINATION CONFIRMED");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("The causal chain is sealed. This agent's existence is recorded");
        println!("in immutable history. It cannot be revived, restarted, or ");
        println!("rewritten. Its scars are permanent. Its death is final.");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }

    println!();
}

fn print_status(agent: &TaskAgent) {
    let energy_pct = (agent.energy() as f64 / 300.0).min(1.0);
    let damage_pct = (agent.damage_score() as f64 / 100.0).min(1.0);
    
    let energy_bar = create_bar(energy_pct, 20, "█", "░");
    let damage_bar = create_bar(damage_pct, 20, "▓", "░");

    let status = if agent.is_alive() { "🟢 ALIVE" } else { "⚫ DEAD" };

    println!("┌─────────────────────────────────────────────────────┐");
    println!("│ Energy: {} [{}] {}/300", energy_bar, status, agent.energy());
    println!("│ Damage: {} [{}]", damage_bar, agent.damage_score());
    println!("│ Capacity remaining: {}", agent.current_capacity());
    println!("│ Completed: {} | Failed: {}", agent.tasks_completed(), agent.tasks_failed());
    println!("└─────────────────────────────────────────────────────┘");
}

fn print_final_report(agent: &TaskAgent) {
    println!("Status Report:");
    println!("├─ Final condition: {}", if agent.is_alive() { "ALIVE" } else { "DEAD" });
    println!("├─ Energy remaining: {}", agent.energy());
    println!("├─ Total accumulated damage: {}", agent.damage_score());
    println!("├─ Final capacity: {}", agent.current_capacity());
    println!("├─ Tasks successfully completed: {}", agent.tasks_completed());
    println!("├─ Tasks that failed: {}", agent.tasks_failed());
    println!("└─ Total task attempts: {}", agent.tasks_completed() + agent.tasks_failed());
}

fn create_bar(ratio: f64, width: usize, filled: &str, empty: &str) -> String {
    let filled_count = ((ratio * width as f64).ceil() as usize).min(width);
    let empty_count = width - filled_count;
    format!("{}{}", filled.repeat(filled_count), empty.repeat(empty_count))
}
