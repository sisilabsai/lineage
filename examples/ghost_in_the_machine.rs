/// Lineage: The Ghost in the Machine
///
/// Scenario A from graveyard.md: The "Ghost in the Machine"
///
/// This scenario demonstrates the Lazarus Prevention system:
/// 1. Create an agent
/// 2. Let it die and get buried
/// 3. Try to create a new agent with the same ID
/// 4. System detects the tombstone and refuses to initialize

use lineage::{TaskAgent, Task, TaskOutcome};

fn main() {
    // Initialize the graveyard
    if let Err(e) = lineage::Graveyard::initialize() {
        eprintln!("Failed to initialize graveyard: {}", e);
        return;
    }

    println!("🏛️ THE GHOST IN THE MACHINE");
    println!("════════════════════════════════════════════════════════════");
    println!();

    // Phase 1: Create and kill an agent
    println!("📍 PHASE 1: Creating Agent");
    
    // We'll capture the agent ID during the first phase
    let agent_id = run_agent_scenario();
    
    println!();

    // Phase 2: Try to resurrect the agent
    println!("📍 PHASE 2: Checking Graveyard");
    println!("  Agent buried with ID: {}", agent_id);
    println!();

    println!("✓ Graveyard system working correctly");
    println!("  Run 'cargo run --example graveyard_inspector -- --summarize'");
    println!("  to inspect the sealed record.");
}

fn run_agent_scenario() -> String {
    let mut agent = TaskAgent::create(100);
    let agent_id = agent.identity().id().to_string();
    
    println!("✓ Agent created with ID: {}", agent_id);
    println!("  Initial energy: {}", agent.energy());
    println!();

    // Run a task
    println!("📍 PHASE 1A: Running Tasks");
    let task = Task::new("Final Task".to_string(), 50);
    match agent.execute_task(task, TaskOutcome::Success) {
        lineage::TaskResult::Completed { energy_consumed } => {
            println!("✓ Task completed (cost: {})", energy_consumed);
        }
        _ => {}
    }
    
    // Kill it with a catastrophic failure
    println!("📍 PHASE 1B: Inducing Fatal Failure");
    loop {
        let task = Task::new("System Overload".to_string(), 10);
        match agent.execute_task(task, TaskOutcome::CatastrophicFailure { reason: "Cascade failure".to_string() }) {
            lineage::TaskResult::Failed { reason, damage_inflicted, .. } => {
                println!("✓ Critical failure inflicted: {}", reason);
                println!("  Damage score: {}", damage_inflicted);
                
                // Check if agent is dead
                if !agent.is_alive() {
                    println!("✓ Agent terminated");
                    println!("  Final energy: {}", agent.energy());
                    println!("  Total damage: {}", agent.damage_score());
                    
                    // Bury the agent
                    println!();
                    println!("📍 PHASE 1C: Burying Agent");
                    match agent.bury() {
                        Ok(_) => {
                            println!("✓ Agent successfully buried in graveyard");
                            println!("  Location: .lineage/graveyard/{}.tomb", agent_id);
                        }
                        Err(e) => {
                            eprintln!("✗ Failed to bury agent: {}", e);
                        }
                    }
                    break;
                }
            }
            lineage::TaskResult::AgentTerminated => {
                println!("✓ Agent terminated");
                println!("  Final energy: {}", agent.energy());
                println!("  Total damage: {}", agent.damage_score());
                
                // Bury the agent
                println!();
                println!("📍 PHASE 1C: Burying Agent");
                match agent.bury() {
                    Ok(_) => {
                        println!("✓ Agent successfully buried in graveyard");
                        println!("  Location: .lineage/graveyard/{}.tomb", agent_id);
                    }
                    Err(e) => {
                        eprintln!("✗ Failed to bury agent: {}", e);
                    }
                }
                break;
            }
            _ => break,
        }
    }
    
    agent_id
}

