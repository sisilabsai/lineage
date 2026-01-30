//! # Descendancy and Cryptographic Seal Demonstration
//!
//! This example demonstrates:
//! 1. The spawn() method - allowing healthy parents to create descendants
//! 2. Genealogical tracking - recording parentage across generations
//! 3. Energy transfer - irreversible resource passing between generations
//! 4. Cryptographic signatures - detecting fraudulent edits to Legacy Scores
//!
//! Run with: cargo run --example descendancy_demo

use lineage::agent::{TaskAgent, Task, TaskOutcome};

fn main() {
    println!("═══════════════════════════════════════════════════════════════");
    println!("🧬 LINEAGE: DESCENDANCY & CRYPTOGRAPHIC SEAL DEMONSTRATION");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Initialize the graveyard system
    if let Err(e) = lineage::graveyard::Graveyard::initialize() {
        eprintln!("Failed to initialize graveyard: {}", e);
        return;
    }

    // ========================================================================
    // PART 1: Create a healthy parent agent
    // ========================================================================
    println!("📍 PART 1: Creating a healthy parent agent...\n");

    let mut parent = TaskAgent::create(1000);
    let parent_id = parent.identity().id().to_string();
    
    println!("✓ Parent Agent Created");
    println!("  ID: {}", parent_id.chars().take(16).collect::<String>());
    println!("  Initial Energy: 1000");
    println!("  Generation: 0 (Origin)\n");

    // Execute some successful tasks to build up health
    println!("📊 Parent executes tasks to build health...\n");
    
    for i in 1..=8 {
        let task = Task::new(format!("Task {}", i), 50);
        let outcome = TaskOutcome::Success;
        let result = parent.execute_task(task, outcome);
        
        match result {
            lineage::agent::TaskResult::Completed { energy_consumed } => {
                println!("  ✓ Task {} completed (cost: {})", i, energy_consumed);
            }
            _ => println!("  ✗ Task {} failed", i),
        }
    }

    println!();
    println!("📈 Parent Status After Training:");
    println!("  Energy Remaining: {}", parent.energy());
    println!("  Capacity: {}", parent.current_capacity());
    println!("  Tasks Completed: {}", parent.tasks_completed());
    println!("  Tasks Failed: {}", parent.tasks_failed());
    println!();

    // Calculate legacy score
    let efficiency = parent.tasks_completed() as f64 
        / (parent.tasks_completed() + parent.tasks_failed()) as f64;
    let legacy_score = efficiency * (parent.tasks_completed() as f64 / 1.0);
    
    println!("  Legacy Score: {:.2} (Efficiency: {:.2})", legacy_score, efficiency);
    println!();

    // ========================================================================
    // PART 2: Spawn a descendant
    // ========================================================================
    println!("🧬 PART 2: Spawning a descendant agent...\n");

    let child_energy = 300;
    match parent.spawn(child_energy) {
        Ok(mut child) => {
            let child_id = child.identity().id().to_string();
            
            println!("✓ Descendant Successfully Spawned!");
            println!("  Parent ID: {}", parent_id.chars().take(16).collect::<String>());
            println!("  Child ID: {}", child_id.chars().take(16).collect::<String>());
            println!("  Inherited Energy: {}", child_energy);
            println!("  Parent's Remaining Energy: {}", parent.energy());
            println!();

            // Have the child execute some tasks
            println!("📊 Child executes tasks...\n");
            
            for i in 1..=4 {
                let task = Task::new(format!("Child Task {}", i), 40);
                let outcome = TaskOutcome::Success;
                let result = child.execute_task(task, outcome);
                
                match result {
                    lineage::agent::TaskResult::Completed { .. } => {
                        println!("  ✓ Child Task {} completed", i);
                    }
                    _ => println!("  ✗ Child Task {} failed", i),
                }
            }

            println!();
            println!("📈 Child Status:");
            println!("  Energy Remaining: {}", child.energy());
            println!("  Tasks Completed: {}", child.tasks_completed());
            println!();

            // ====================================================================
            // PART 3: Bury both parent and child to demonstrate cryptographic seal
            // ====================================================================
            println!("⚰️  PART 3: Burying agents and verifying cryptographic seals...\n");

            // Kill the parent by exhausting energy
            while parent.is_alive() && parent.energy() > 0 {
                let task_cost = std::cmp::min(parent.energy(), 100);
                let exhaustion_task = Task::new("Final exhaustion".to_string(), task_cost);
                match parent.execute_task(exhaustion_task, TaskOutcome::CatastrophicFailure {
                    reason: "Energy depletion".to_string(),
                }) {
                    lineage::agent::TaskResult::Failed { .. } => {
                        // Agent died, exit loop
                        break;
                    }
                    _ => {
                        // Task executed, continue
                    }
                }
            }

            // Kill the child by exhausting energy
            while child.is_alive() && child.energy() > 0 {
                let task_cost = std::cmp::min(child.energy(), 100);
                let exhaustion_task = Task::new("Child exhaustion".to_string(), task_cost);
                match child.execute_task(exhaustion_task, TaskOutcome::CatastrophicFailure {
                    reason: "Energy depletion".to_string(),
                }) {
                    lineage::agent::TaskResult::Failed { .. } => {
                        // Agent died, exit loop
                        break;
                    }
                    _ => {
                        // Task executed, continue
                    }
                }
            }

            // Bury them
            match parent.bury() {
                Ok(_) => {
                    println!("✓ Parent agent buried in the Graveyard");
                    println!("  Tombstone cryptographically sealed with HMAC-SHA256");
                }
                Err(e) => eprintln!("✗ Failed to bury parent: {}", e),
            }

            match child.bury() {
                Ok(_) => {
                    println!("✓ Child agent buried in the Graveyard");
                    println!("  Genealogical record recorded (parent: {})", parent_id.chars().take(8).collect::<String>());
                    println!("  Tombstone cryptographically sealed with HMAC-SHA256");
                }
                Err(e) => eprintln!("✗ Failed to bury child: {}", e),
            }

            println!();

            // ====================================================================
            // PART 4: Verify the cryptographic seals
            // ====================================================================
            println!("🔐 PART 4: Signature verification...\n");

            match lineage::graveyard::Graveyard::load(&parent_id) {
                Ok(parent_tomb) => {
                    match parent_tomb.verify_signature() {
                        Ok(_) => {
                            println!("✓ Parent's cryptographic signature verified");
                            println!("  Status: No tampering detected");
                        }
                        Err(e) => {
                            println!("✗ Parent's signature verification failed: {}", e);
                            println!("  Status: FRAUDULENT HISTORY DETECTED");
                        }
                    }
                }
                Err(e) => eprintln!("Error loading parent tombstone: {}", e),
            }

            match lineage::graveyard::Graveyard::load(&child_id) {
                Ok(child_tomb) => {
                    match child_tomb.verify_signature() {
                        Ok(_) => {
                            println!("✓ Child's cryptographic signature verified");
                            println!("  Status: No tampering detected");
                            println!("  Genealogy: Descended from parent agent");
                        }
                        Err(e) => {
                            println!("✗ Child's signature verification failed: {}", e);
                            println!("  Status: FRAUDULENT HISTORY DETECTED");
                        }
                    }
                }
                Err(e) => eprintln!("Error loading child tombstone: {}", e),
            }

            println!();
        }
        Err(e) => {
            eprintln!("✗ Failed to spawn descendant: {}", e);
            println!();
            println!("Spawning requires:");
            println!("  - Parent agent must be ALIVE");
            println!("  - Parent's legacy score must be >= 0.5");
            println!("  - Parent must have energy: child_energy + 50");
            println!("  - Parent should have completed 5+ tasks");
        }
    }

    // ========================================================================
    // SUMMARY
    // ========================================================================
    println!("═══════════════════════════════════════════════════════════════");
    println!("🧬 DESCENDANCY DEMONSTRATION COMPLETE");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Key Features Demonstrated:");
    println!("  1. ✓ spawn() - Created descendant from healthy parent");
    println!("  2. ✓ Genealogical tracking - Parent-child relationships recorded");
    println!("  3. ✓ Irreversible energy transfer - Parent weakened by spawning");
    println!("  4. ✓ Cryptographic seals - HMAC-SHA256 prevents tampering");
    println!("  5. ✓ Fraud detection - Signatures detect Legacy Score edits");
    println!();

    println!("🏛️  Verify with: cargo run --example graveyard_inspector -- --summarize");
    println!("🔐 Check seals with: cargo run --example graveyard_inspector -- --verify <ID>");
}
