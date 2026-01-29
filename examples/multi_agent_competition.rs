/// Lineage: Multi-Agent Competition
/// 
/// A population-level simulation showing emergent natural selection:
/// - 10 agents compete for 60 limited tasks
/// - Only successful agents gain progress
/// - Failures inflict permanent scars that compound
/// - Agents degrade at different rates based on luck
/// - Some thrive briefly, most spiral into death
/// - Dead agents remain sealed in permanent history
///
/// This demonstrates:
/// - Population consequences of irreversibility
/// - Emergent "natural selection" (survivors earn it through fewer scars)
/// - Observable sealed lineages post-mortem
/// - Environmental pressure forcing differentiation

use lineage::{TaskAgent, Task, TaskOutcome};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

const INITIAL_ENERGY: u64 = 300;  // Lower energy = more deaths
const NUM_AGENTS: usize = 10;
const TOTAL_TASKS: usize = 60;
const TASK_DIFFICULTY_SCALING: f64 = 1.6;  // Steeper cost curve = more failures

/// Agent statistics tracker
#[derive(Clone)]
struct AgentStats {
    _id: usize,
    tasks_completed: u32,
    tasks_failed: u32,
    peak_energy: u64,
    death_round: Option<usize>,
    final_damage: u32,
    final_energy: u64,
}

fn main() {
    clear_screen();
    
    print_banner();
    println!();

    // Initialize agents
    let mut agents: Vec<TaskAgent> = (0..NUM_AGENTS)
        .map(|_| TaskAgent::create(INITIAL_ENERGY))
        .collect();

    let mut stats: Vec<AgentStats> = (0..NUM_AGENTS)
        .map(|i| AgentStats {
            _id: i,
            tasks_completed: 0,
            tasks_failed: 0,
            peak_energy: INITIAL_ENERGY,
            death_round: None,
            final_damage: 0,
            final_energy: INITIAL_ENERGY,
        })
        .collect();

    // Print agent roster
    println!("╔══ POPULATION BORN ══════════════════════════════════════════╗");
    for i in 0..NUM_AGENTS {
        println!("│ Agent {:2} initialized with {} energy units                  │", 
                 i, INITIAL_ENERGY);
    }
    println!("╚═════════════════════════════════════════════════════════════╝\n");

    let mut alive_agents = NUM_AGENTS;
    let mut task_counter = 0;

    // ============================================================================
    // MAIN SIMULATION LOOP
    // ============================================================================
    println!("╔══ COMPETITION BEGINS ═══════════════════════════════════════╗");
    println!("║ {} agents | {} shared tasks | Limited resources              ║", NUM_AGENTS, TOTAL_TASKS);
    println!("╚═════════════════════════════════════════════════════════════╝\n");

    let mut rng = thread_rng();

    while alive_agents > 0 && task_counter < TOTAL_TASKS {
        // Shuffle agent order for fairness
        let mut indices: Vec<usize> = (0..NUM_AGENTS).collect();
        indices.shuffle(&mut rng);

        // Each round, cycle through living agents
        for &idx in &indices {
            if !agents[idx].is_alive() {
                continue;
            }

            if task_counter >= TOTAL_TASKS {
                break;
            }

            task_counter += 1;
            let round = task_counter;

            // Task difficulty scales with time
            let base_difficulty = 40u64;
            let scaled_difficulty = (base_difficulty as f64 * 
                (1.0 + (round as f64 / TOTAL_TASKS as f64) * TASK_DIFFICULTY_SCALING)) as u64;
            let difficulty = scaled_difficulty + rng.gen_range(0..30);

            let task = Task::new(format!("Task #{}", round), difficulty);

            // Success probability decreases with damage
            let damage_penalty = agents[idx].damage_score() as f64 * 0.02;  // Steeper penalty
            let base_success_rate = 55.0;  // Lower base success rate
            let success_rate = ((base_success_rate - damage_penalty).max(5.0)) / 100.0;  // Lower floor

            // Determine outcome probabilistically
            let outcome = if rng.gen_bool(success_rate) {
                TaskOutcome::Success
            } else {
                // Failure severity varies (heavily weighted toward catastrophic)
                match rng.gen_range(0..10) {
                    0..=1 => TaskOutcome::RecoverableFailure {
                        reason: "Transient error".to_string(),
                    },
                    2..=4 => TaskOutcome::SignificantFailure {
                        reason: "Resource contention".to_string(),
                    },
                    5..=7 => TaskOutcome::SevereFailure {
                        reason: "System degradation".to_string(),
                    },
                    _ => TaskOutcome::CatastrophicFailure {
                        reason: "Cascade failure".to_string(),
                    },
                }
            };

            // Execute task
            let result = agents[idx].execute_task(task, outcome);

            // Process result and update stats
            match result {
                lineage::TaskResult::Completed { energy_consumed } => {
                    stats[idx].tasks_completed += 1;
                    print_task_success(idx, round, &agents[idx], energy_consumed);
                }
                lineage::TaskResult::Failed {
                    reason,
                    damage_inflicted,
                    ..
                } => {
                    stats[idx].tasks_failed += 1;
                    print_task_failure(idx, round, &agents[idx], &reason, damage_inflicted);
                }
                lineage::TaskResult::InsufficientEnergy { .. } => {
                    stats[idx].tasks_failed += 1;
                    print_energy_exhausted(idx, round);
                }
                lineage::TaskResult::CapacityInsufficient { .. } => {
                    stats[idx].tasks_failed += 1;
                    print_capacity_insufficient(idx, round, agents[idx].current_capacity());
                }
                lineage::TaskResult::AgentTerminated => {
                    stats[idx].death_round = Some(round);
                    stats[idx].final_damage = agents[idx].damage_score();
                    stats[idx].final_energy = agents[idx].energy();
                    print_agent_death(idx, round);
                    alive_agents -= 1;
                }
            }

            // Track peak energy
            if agents[idx].is_alive() && agents[idx].energy() > stats[idx].peak_energy {
                stats[idx].peak_energy = agents[idx].energy();
            }
            
            // Small pause for screen recording visibility
            std::thread::sleep(std::time::Duration::from_millis(150));
        }
    }

    // ============================================================================
    // POST-MORTEM ANALYSIS
    // ============================================================================
    println!();
    print_competition_end();

    let mut survivors: Vec<usize> = vec![];
    let mut dead: Vec<usize> = vec![];

    for i in 0..NUM_AGENTS {
        if agents[i].is_alive() {
            survivors.push(i);
            stats[i].final_damage = agents[i].damage_score();
            stats[i].final_energy = agents[i].energy();
        } else {
            dead.push(i);
        }
    }

    // Print survivors first
    if !survivors.is_empty() {
        println!();
        println!("╔══ SURVIVORS ════════════════════════════════════════════════╗");
        
        // Sort by tasks completed (descending)
        survivors.sort_by_key(|&idx| std::cmp::Reverse(stats[idx].tasks_completed));
        
        for (rank, &idx) in survivors.iter().enumerate() {
            let s = &stats[idx];
            print_survivor(rank + 1, idx, s);
        }
        println!("╚═════════════════════════════════════════════════════════════╝");
    } else {
        println!();
        println!("╔════════════════════════════════════════════════════════════╗");
        println!("║  ☠️  COMPLETE EXTINCTION - All agents dead                 ║");
        println!("╚════════════════════════════════════════════════════════════╝");
    }

    // Print dead agents' sealed lineages
    if !dead.is_empty() {
        println!();
        println!("╔══ SEALED LINEAGES (Post-Mortem Records) ════════════════════╗");
        
        // Sort by completion count (descending)
        dead.sort_by_key(|&idx| std::cmp::Reverse(stats[idx].tasks_completed));
        
        for (rank, &idx) in dead.iter().enumerate() {
            let s = &stats[idx];
            let death_round = s.death_round.unwrap_or(TOTAL_TASKS);
            println!(
                "│ #{:2} | Agent {:2} | Completed: {:2} | Failed: {:2} | Damage: {:3} | Death round: {:2} │",
                rank + 1,
                idx,
                s.tasks_completed,
                s.tasks_failed,
                s.final_damage,
                death_round
            );
        }
        println!("│ (All histories sealed permanently — immutable record)       │");
        println!("╚═════════════════════════════════════════════════════════════╝");
    }

    // Final summary
    println!();
    print_final_summary(&agents, &survivors, &stats, task_counter, alive_agents);
    
    println!();
    println!("═══════════════════════════════════════════════════════════════════");
    println!("All lineages—living and dead—remain forever recorded.");
    println!("The consequences of their actions are written in irreversible history.");
    println!("═══════════════════════════════════════════════════════════════════");
    println!();
}

// ============================================================================
// OUTPUT FORMATTING FUNCTIONS
// ============================================================================

fn print_banner() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║                                                               ║");
    println!("║          LINEAGE: MULTI-AGENT COMPETITION                    ║");
    println!("║                                                               ║");
    println!("║  Ten agents compete for sixty tasks.                         ║");
    println!("║  Scars compound. Energy depletes. Death is final.            ║");
    println!("║  Evolution through consequence.                              ║");
    println!("║                                                               ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
}

fn print_competition_end() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║                   COMPETITION COMPLETE                        ║");
    println!("║                                                               ║");
    println!("║          Analysis of population consequences                  ║");
    println!("║                                                               ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
}

fn print_task_success(idx: usize, round: usize, agent: &TaskAgent, _energy_used: u64) {
    println!(
        "  ✓ Agent {:2} | Rnd {:2} | SUCCESS    │ Energy: {:3} | Dmg: {:2} | Cap: {}",
        idx, round, agent.energy(), agent.damage_score(), agent.current_capacity()
    );
}

fn print_task_failure(idx: usize, round: usize, agent: &TaskAgent, reason: &str, damage: u32) {
    println!(
        "  ⚠ Agent {:2} | Rnd {:2} | {}  | Scar +{:2} | Energy: {:3} | Cap: {}",
        idx, round, format!("{:<19}", reason), damage, agent.energy(), agent.current_capacity()
    );
}

fn print_energy_exhausted(idx: usize, round: usize) {
    println!(
        "  ✗ Agent {:2} | Rnd {:2} | NO ENERGY              │ Task cannot execute      ",
        idx, round
    );
}

fn print_capacity_insufficient(idx: usize, round: usize, capacity: u32) {
    println!(
        "  ✗ Agent {:2} | Rnd {:2} | NO CAPACITY ({}%)        │ Agent immobilized        ",
        idx, round, capacity
    );
}

fn print_agent_death(idx: usize, round: usize) {
    println!(
        "  ⚫ Agent {:2} | Rnd {:2} | TERMINATED             │ Lineage sealed - GOODBYE ",
        idx, round
    );
}

fn print_survivor(rank: usize, idx: usize, stats: &AgentStats) {
    println!(
        "│ #{} │ Agent {:2} │ Tasks: {}/{} │ Damage: {:2} │ Energy: {:3} │ SUCCESS │",
        rank,
        idx,
        stats.tasks_completed,
        stats.tasks_completed + stats.tasks_failed,
        stats.final_damage,
        stats.final_energy
    );
}

fn print_final_summary(
    agents: &[TaskAgent],
    survivors: &[usize],
    stats: &[AgentStats],
    tasks_completed: usize,
    _alive_count: usize,
) {
    println!("╔══ FINAL STATISTICS ═════════════════════════════════════════╗");
    
    // Count actual alive agents
    let actual_alive = agents.iter().filter(|a| a.is_alive()).count();
    
    let total_completed: u32 = stats.iter().map(|s| s.tasks_completed).sum();
    let total_failed: u32 = stats.iter().map(|s| s.tasks_failed).sum();
    let avg_damage: f64 = stats.iter().map(|s| s.final_damage as f64).sum::<f64>() / NUM_AGENTS as f64;
    
    println!("│ Population: {}/{} alive ({}% survival)", actual_alive, NUM_AGENTS,
        (actual_alive * 100 / NUM_AGENTS));
    println!("│ Tasks processed: {} / {}", tasks_completed, TOTAL_TASKS);
    println!("│ Total completions: {} | Total failures: {}", total_completed, total_failed);
    println!("│ Average damage per agent: {:.1}", avg_damage);
    
    if !survivors.is_empty() {
        let winner = survivors[0];
        let winner_stats = &stats[winner];
        println!("│");
        println!("│ 🏆 CHAMPION: Agent {} | {} tasks completed | {} damage",
            winner, winner_stats.tasks_completed, winner_stats.final_damage);
    }
    
    println!("╚═════════════════════════════════════════════════════════════╝");
}

#[cfg(target_os = "windows")]
fn clear_screen() {
    std::process::Command::new("cmd")
        .args(&["/C", "cls"])
        .status()
        .ok();
}

#[cfg(not(target_os = "windows"))]
fn clear_screen() {
    std::process::Command::new("clear")
        .status()
        .ok();
}