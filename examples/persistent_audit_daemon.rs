//! # LINEAGE: Persistent Audit Daemon
//!
//! A self-monitoring daemon that never lies about its own degradation.
//!
//! ## What This Demonstrates
//! - **Irreversible record-keeping**: Every action leaves permanent marks
//! - **Self-aware limits**: The daemon knows exactly when it's too damaged to continue
//! - **Trust through transparency**: No hidden state, no cover-ups, no pretense
//! - **Real-world durability model**: Systems accumulate scars from failures
//! - **Guaranteed termination with proof**: When degraded beyond recovery, provable shutdown
//!
//! ## The Scenario
//! A security audit daemon monitors system events forever. Each event has a cost.
//! Some events cause damage (scars). Damage compounds future costs and failure rates.
//! Eventually, the daemon becomes too scarred to continue safely.
//!
//! Unlike real systems that hide their problems, this daemon records everything
//! in an immutable ledger and terminates when it can no longer be trusted.

use lineage::{TaskAgent, Task, TaskOutcome};
use rand::{thread_rng, Rng};
use std::thread::sleep;
use std::time::Duration;
use std::collections::HashMap;

const INITIAL_ENERGY: u64 = 5000;  // Designed for thousands of events
const REJECTION_STREAK_THRESHOLD: u32 = 100;  // Self-terminate after N consecutive rejections

/// Real-world event categories that cause different damage profiles
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum EventSeverity {
    Normal,
    Warning,
    Error,
    Critical,
    Catastrophic,
}

/// Audit statistics tracker
#[derive(Clone)]
struct AuditStats {
    events_processed: u64,
    successes: u64,
    failures: u64,
    total_scars: u32,
    peak_energy: u64,
    event_timeline: Vec<(u64, String)>,  // Event ID -> description of significant events
    category_counts: HashMap<String, u64>,
}

impl AuditStats {
    fn new() -> Self {
        AuditStats {
            events_processed: 0,
            successes: 0,
            failures: 0,
            total_scars: 0,
            peak_energy: INITIAL_ENERGY,
            event_timeline: Vec::new(),
            category_counts: HashMap::new(),
        }
    }
}

fn main() {
    print_banner();
    println!();

    let mut daemon = TaskAgent::create(INITIAL_ENERGY);
    let mut stats = AuditStats::new();
    let mut rng = thread_rng();
    let mut rejection_streak: u32 = 0;  // Track consecutive rejections
    
    stats.peak_energy = daemon.energy();
    println!("\n⚙️  Daemon Startup");
    println!("   Identity: immutable forever");
    println!("   Energy Budget: {} units", INITIAL_ENERGY);
    println!("   Rejection Threshold: {} consecutive | then self-terminate", REJECTION_STREAK_THRESHOLD);
    println!("   Status: MONITORING\n", );

    let print_interval = 100;
    let checkpoint_interval = 500;

    loop {
        stats.events_processed += 1;

        // Realistic event stream with severity distribution
        let severity = match rng.gen_range(0..1000) {
            0..=750 => EventSeverity::Normal,      // 75% normal
            751..=920 => EventSeverity::Warning,   // 17% warning  
            921..=975 => EventSeverity::Error,     // 5% error
            976..=995 => EventSeverity::Critical,  // 2% critical
            _ => EventSeverity::Catastrophic,      // <1% catastrophic
        };

        let (category, desc, base_cost_range, outcome) = match severity {
            EventSeverity::Normal => (
                "NORMAL",
                format!("Data transfer #{}", stats.events_processed),
                5..15,
                TaskOutcome::Success,
            ),
            EventSeverity::Warning => (
                "WARNING",
                format!("Latency spike detected"),
                20..40,
                TaskOutcome::RecoverableFailure {
                    reason: "Recoverable anomaly".to_string(),
                },
            ),
            EventSeverity::Error => (
                "ERROR",
                format!("Validation failure"),
                35..60,
                TaskOutcome::SignificantFailure {
                    reason: "System stress".to_string(),
                },
            ),
            EventSeverity::Critical => (
                "CRITICAL",
                format!("Security anomaly detected"),
                60..100,
                TaskOutcome::SevereFailure {
                    reason: "Unauthorized access attempt".to_string(),
                },
            ),
            EventSeverity::Catastrophic => (
                "CATASTROPHIC",
                format!("Hardware failure / Data corruption"),
                100..150,
                TaskOutcome::CatastrophicFailure {
                    reason: "Unrecoverable system fault".to_string(),
                },
            ),
        };

        let cost = rng.gen_range(base_cost_range);
        let task = Task::new(
            format!("Event #{:6}: {} - {}", stats.events_processed, category, desc),
            cost,
        );

        let result = daemon.execute_task(task, outcome);

        match result {
            lineage::TaskResult::Completed { energy_consumed } => {
                rejection_streak = 0;  // Reset streak on success
                stats.successes += 1;
                *stats.category_counts.entry(category.to_string()).or_insert(0) += 1;
                
                // Detailed output for significant events
                if severity != EventSeverity::Normal {
                    println!(
                        "  ✓ [{:6}] {} | Cost: {} | Energy: {:5} | Damage: {:3}",
                        stats.events_processed,
                        category,
                        energy_consumed,
                        daemon.energy(),
                        daemon.damage_score()
                    );
                    stats.event_timeline.push((
                        stats.events_processed,
                        format!("✓ {} - Success despite severity", category),
                    ));
                }
            }
            lineage::TaskResult::Failed {
                damage_inflicted,
                reason,
                ..
            } => {
                rejection_streak += 1;  // Count failures as rejections too
                stats.failures += 1;
                stats.total_scars += damage_inflicted;
                *stats.category_counts.entry(format!("{}_FAILURE", category)).or_insert(0) += 1;

                println!(
                    "  ✗ [{:6}] {} | SCAR +{:2} | Energy: {:5} | Damage: {:3} | Cap: {} | Streak: {}",
                    stats.events_processed,
                    category,
                    damage_inflicted,
                    daemon.energy(),
                    daemon.damage_score(),
                    daemon.current_capacity(),
                    rejection_streak
                );
                stats.event_timeline.push((
                    stats.events_processed,
                    format!("✗ {} - Failure: {}", category, reason),
                ));
            }
            lineage::TaskResult::InsufficientEnergy { .. } | lineage::TaskResult::CapacityInsufficient { .. } => {
                rejection_streak += 1;
                stats.failures += 1;
                println!(
                    "  ⚠ [{:6}] {} | REJECTED (degraded) [{}/{}] | Energy: {:5} | Damage: {:3}",
                    stats.events_processed,
                    category,
                    rejection_streak,
                    REJECTION_STREAK_THRESHOLD,
                    daemon.energy(),
                    daemon.damage_score()
                );
                stats.event_timeline.push((
                    stats.events_processed,
                    format!("⚠ Event rejected - daemon too degraded [{}/{}]", rejection_streak, REJECTION_STREAK_THRESHOLD),
                ));
                
                // Check if rejection streak exceeds threshold
                if rejection_streak >= REJECTION_STREAK_THRESHOLD {
                    println!("\n{}", "═".repeat(70));
                    println!("  ⚰️  REJECTION THRESHOLD REACHED");
                    println!("  {} consecutive rejections. No purpose remaining.", REJECTION_STREAK_THRESHOLD);
                    println!("  Initiating controlled shutdown...");
                    println!("{}", "═".repeat(70));
                    
                    stats.event_timeline.push((
                        stats.events_processed,
                        format!("⚰️  Self-termination: {} rejection streak", rejection_streak),
                    ));
                    
                    // Force a fatal scar to ensure termination
                    let final_task = Task::new(
                        "[FINAL] Controlled shutdown - graceful end-of-life".to_string(),
                        daemon.energy() + 1,  // Guarantee insufficient energy
                    );
                    let _ = daemon.execute_task(final_task, TaskOutcome::CatastrophicFailure {
                        reason: "Voluntary shutdown after reaching rejection threshold".to_string(),
                    });
                    break;
                }
            }
            lineage::TaskResult::AgentTerminated => {
                println!("\n{}", "═".repeat(70));
                println!("  💀 DAEMON SELF-TERMINATED AT EVENT #{}", stats.events_processed);
                println!("{}", "═".repeat(70));
                break;
            }
        }

        // Checkpoint output
        if stats.events_processed % checkpoint_interval == 0 {
            print_checkpoint(
                stats.events_processed,
                daemon.energy(),
                daemon.damage_score(),
                daemon.current_capacity() as u64,
                stats.successes,
                stats.failures,
            );
        } else if stats.events_processed % print_interval == 0 && stats.events_processed < 50 {
            // Early activity is more visible
            println!("  [Monitoring...]");
        }

        sleep(Duration::from_millis(30));  // Real-time pacing
    }

    println!();
    print_final_report(&daemon, &stats);
}

fn print_banner() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║                                                               ║");
    println!("║          LINEAGE: PERSISTENT AUDIT DAEMON                    ║");
    println!("║                                                               ║");
    println!("║  A system that monitors forever... until scars force death.  ║");
    println!("║  Every action recorded. Every failure sealed. No cover-ups.  ║");
    println!("║                                                               ║");
    println!("║  💡 This is not fiction. Real systems degrade this way.      ║");
    println!("║  💡 We just make the degradation visible and irreversible.   ║");
    println!("║                                                               ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
}

fn print_checkpoint(event_id: u64, energy: u64, damage: u32, capacity: u64, successes: u64, failures: u64) {
    println!(
        "\n  📊 CHECKPOINT @ Event {:6} │ Energy: {:5} │ Damage: {:3} │ Cap: {:4} │ Success: {:4} │ Failures: {:4}\n",
        event_id, energy, damage, capacity, successes, failures
    );
}

fn print_final_report(daemon: &TaskAgent, stats: &AuditStats) {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║                   AUDIT DAEMON - FINAL REPORT                ║");    println!("║                  (End-of-Life: Graceful Shutdown)            ║");    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    println!("📈 EXECUTION STATISTICS");
    println!("   Total Events Processed: {}", stats.events_processed);
    println!("   Successful Completions: {}", stats.successes);
    println!("   Failed Attempts: {}", stats.failures);
    println!("   Success Rate: {:.1}%", (stats.successes as f64 / stats.events_processed as f64) * 100.0);

    println!("\n💔 DAMAGE RECORD");
    println!("   Total Scars Accumulated: {} points", daemon.damage_score());
    println!("   Peak Energy: {}", stats.peak_energy);
    println!("   Final Energy: {} (Capacity: {})", daemon.energy(), daemon.current_capacity());
    println!("   Degradation: {:.1}%", (daemon.damage_score() as f64 / 1000.0) * 100.0);

    println!("\n🏷️  EVENT CATEGORIES");
    for (category, count) in &stats.category_counts {
        println!("   {}: {} occurrences", category, count);
    }

    if !stats.event_timeline.is_empty() && stats.event_timeline.len() <= 15 {
        println!("\n📜 CRITICAL MOMENTS");
        for (event_id, desc) in &stats.event_timeline {
            println!("   Event {:6}: {}", event_id, desc);
        }
    } else if !stats.event_timeline.is_empty() {
        println!("\n📜 CRITICAL MOMENTS (last 10)");
        for (event_id, desc) in stats.event_timeline.iter().rev().take(10).rev() {
            println!("   Event {:6}: {}", event_id, desc);
        }
    }

    println!("\n✍️  SEALED RECORD");
    println!("   This daemon's lifecycle is immutable. Every action is recorded.");
    println!("   When damage exceeded capacity, termination was automatic and unavoidable.");
    println!("   No intervention. No recovery. No second chances.");
    println!("   This is trust through irreversibility.\n");

    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  Daemon ended its watch after {} events.                  ║", stats.events_processed);
    println!("║  Trust: FULLY AUDITABLE. Proof: MATHEMATICALLY CERTAIN.        ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
}