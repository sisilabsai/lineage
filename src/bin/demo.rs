/// Lineage - Complete System Demonstration
/// 
/// This binary demonstrates the full Lineage ontological system:
/// - Unique identity that cannot be duplicated
/// - Finite energy that decays irreversibly  
/// - Permanent scars that accumulate and degrade capacity
/// - Complete lifecycle from birth to death
/// - Immutable causal history
///
/// Run with: cargo run

use lineage::{TaskAgent, Task, TaskOutcome};
use std::thread;
use std::time::Duration;

fn main() {
    clear_screen();
    
    println!("\n");
    print_header("LINEAGE SYSTEM DEMONSTRATION");
    println!();
    println!("    An ontological system for software that experiences consequences.");
    println!("    All energy is finite. All scars are permanent. All death is final.");
    println!();
    println!("    Five core truths:");
    println!("    1. Identity cannot be cloned or copied");
    println!("    2. History cannot be erased or rewritten");
    println!("    3. Energy cannot be restored or recharged");
    println!("    4. Scars are permanent and visible forever");
    println!("    5. Death is final and irreversible");
    println!();

    // Create agent with moderate energy
    let mut agent = TaskAgent::create(250);
    
    println!("═══════════════════════════════════════════════════════════════════");
    println!("                    AGENT LIFECYCLE BEGINS");
    println!("═══════════════════════════════════════════════════════════════════\n");
    
    display_agent(&agent);

    // Phase 1: Successful operations (confidence building)
    section("Phase 1: Initialization - Building Confidence");
    
    let init_tasks = vec![
        ("Bootstrap runtime", 20),
        ("Load configuration", 15),
        ("Initialize storage", 25),
        ("Establish connections", 20),
    ];

    for (task_name, cost) in init_tasks {
        let task = Task::new(task_name.to_string(), cost);
        match agent.execute_task(task, TaskOutcome::Success) {
            lineage::TaskResult::Completed { energy_consumed } => {
                println!("  ✓ {} [cost: {}]", task_name, energy_consumed);
            }
            _ => println!("  ✗ {} failed unexpectedly", task_name),
        }
    }
    
    display_agent(&agent);
    pause();

    // Phase 2: Manageable failures (scars accumulate)
    section("Phase 2: Operations Under Load - Scars Accumulate");
    
    let load_tasks = vec![
        ("Handle concurrent request", TaskOutcome::RecoverableFailure {
            reason: "Timeout (retryable)".to_string(),
        }),
        ("Process batch update", TaskOutcome::SignificantFailure {
            reason: "Constraint violation".to_string(),
        }),
        ("Execute query", TaskOutcome::Success),
        ("Sync distributed state", TaskOutcome::SevereFailure {
            reason: "Consistency error".to_string(),
        }),
        ("Recover from error", TaskOutcome::RecoverableFailure {
            reason: "Partial recovery".to_string(),
        }),
    ];

    for (task_name, outcome) in load_tasks {
        let cost = 25;
        let task = Task::new(task_name.to_string(), cost);
        
        match agent.execute_task(task, outcome) {
            lineage::TaskResult::Completed { .. } => {
                println!("  ✓ {}", task_name);
            }
            lineage::TaskResult::Failed { reason, damage_inflicted, .. } => {
                println!("  ⚠ {} (scar: +{})", reason, damage_inflicted);
            }
            _ => {}
        }
        
        if !agent.is_alive() {
            break;
        }
    }
    
    display_agent(&agent);
    pause();

    // Phase 3: Degradation (capacity drops, costs rise)
    section("Phase 3: Degradation - Capacity Falls, Costs Rise");
    
    let degrading_tasks = vec![
        ("Complex transaction", 35),
        ("Data reconciliation", 40),
        ("Intensive computation", 45),
    ];

    for (task_name, cost) in degrading_tasks {
        if !agent.is_alive() {
            break;
        }
        
        let task = Task::new(task_name.to_string(), cost);
        
        match agent.execute_task(task.clone(), TaskOutcome::SevereFailure {
            reason: format!("Resource exhaustion under load"),
        }) {
            lineage::TaskResult::Failed { reason, damage_inflicted, .. } => {
                println!("  ⚠ {} (damage: +{})", reason, damage_inflicted);
            }
            lineage::TaskResult::InsufficientEnergy { required, available } => {
                println!("  ✗ {} - INSUFFICIENT ENERGY (need {}, have {})", 
                    task_name, required, available);
                break;
            }
            _ => {}
        }
    }
    
    display_agent(&agent);
    pause();

    // Final status
    println!();
    println!("═══════════════════════════════════════════════════════════════════");
    println!("                         FINAL ANALYSIS");
    println!("═══════════════════════════════════════════════════════════════════\n");
    
    println!("Agent Status:");
    println!("  Identity:      {}", if agent.is_alive() { "ACTIVE" } else { "TERMINATED" });
    println!("  Energy:        {} / 250", agent.energy());
    println!("  Damage:        {}/100 (capacity: {})", agent.damage_score(), agent.current_capacity());
    println!("  Tasks:         {} completed, {} failed", agent.tasks_completed(), agent.tasks_failed());
    
    println!();
    if !agent.is_alive() {
        println!("╔═══════════════════════════════════════════════════════════╗");
        println!("║              AGENT TERMINATED - LIFE COMPLETE             ║");
        println!("║                                                           ║");
        println!("║  The causal chain is sealed. This agent's existence is    ║");
        println!("║  recorded in immutable history. It cannot be revived,    ║");
        println!("║  restarted, or rewritten. Its scars are permanent.       ║");
        println!("║  Its death is final.                                     ║");
        println!("╚═══════════════════════════════════════════════════════════╝");
    } else {
        println!("╔═══════════════════════════════════════════════════════════╗");
        println!("║         AGENT SURVIVES - FOR NOW                          ║");
        println!("║                                                           ║");
        println!("║  {} energy remains, but entropy marches on...            ║", agent.energy());
        println!("║  Damage: {} scars will continue to degrade capacity.  ║", agent.damage_score());
        println!("║  The dance with mortality continues...                  ║");
        println!("╚═══════════════════════════════════════════════════════════╝");
    }
    
    println!();
}

fn print_header(text: &str) {
    let border = "╔".to_string() + &"═".repeat(text.len() + 2) + "╗";
    println!("{}", border);
    println!("║ {} ║", text);
    println!("{}", border.replace("╔", "╚").replace("╗", "╝"));
}

fn section(title: &str) {
    println!();
    println!("───────────────────────────────────────────────────────────────────");
    println!("  {}", title);
    println!("───────────────────────────────────────────────────────────────────\n");
}

fn display_agent(agent: &TaskAgent) {
    let energy = agent.energy();
    let damage = agent.damage_score();
    let capacity = agent.current_capacity();
    let max_energy = 250u64;
    
    let energy_pct = (energy as f64 / max_energy as f64).min(1.0);
    let damage_pct = (damage as f64 / 100.0).min(1.0);
    
    let energy_bar = make_bar(energy_pct, 20, "█", "░");
    let damage_bar = make_bar(damage_pct, 20, "▓", "░");
    let capacity_bar = make_bar(capacity as f64 / 100.0, 15, "▰", "▱");
    
    let status = if agent.is_alive() {
        "🟢 ALIVE   "
    } else {
        "⚫ DEAD    "
    };

    println!("┌──────────────────────────────────────────────────────┐");
    println!("│ Status: {}                              │", status);
    println!("├──────────────────────────────────────────────────────┤");
    println!("│ Energy:  {} {}/{}", energy_bar, energy, max_energy);
    println!("│ Damage:  {} {}/100", damage_bar, damage);
    println!("│ Capacity: {} {}/100", capacity_bar, capacity);
    println!("├──────────────────────────────────────────────────────┤");
    println!("│ Completed: {:3} | Failed: {:3}                       │", 
        agent.tasks_completed(), agent.tasks_failed());
    println!("└──────────────────────────────────────────────────────┘");
}

fn make_bar(ratio: f64, width: usize, filled: &str, empty: &str) -> String {
    let filled_count = ((ratio * width as f64).ceil() as usize).min(width);
    let empty_count = width - filled_count;
    format!("{}{}", filled.repeat(filled_count), empty.repeat(empty_count))
}

fn pause() {
    thread::sleep(Duration::from_millis(300));
}

#[cfg(target_os = "windows")]
fn clear_screen() {
    std::process::Command::new("cls")
        .status()
        .ok();
}

#[cfg(not(target_os = "windows"))]
fn clear_screen() {
    std::process::Command::new("clear")
        .status()
        .ok();
}
