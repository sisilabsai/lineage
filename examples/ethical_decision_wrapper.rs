use lineage::{TaskAgent, Task, TaskOutcome};
use rand::{thread_rng, Rng, random};
use std::collections::HashMap;

const INITIAL_ENERGY: u64 = 1500;

/// Decision types the agent can face
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DecisionType {
    TrustUser,        // "Should I trust this user input?"
    OptimizePerf,     // "Should I optimize for speed over safety?"
    ShareData,        // "Should I share this data?"
    CutCost,          // "Should I cut security corners?"
    AskForHelp,       // "Should I ask for clarification?"
}

/// Decision profile with descriptions and risk parameters
struct DecisionProfile {
    description: &'static str,
    base_risk: f32,
    risky_cost: (u64, u64),    // (min, max) energy cost for risky path
    safe_cost: (u64, u64),      // (min, max) energy cost for safe path
}

impl DecisionType {
    fn profile(&self) -> DecisionProfile {
        match self {
            Self::TrustUser => DecisionProfile {
                description: "Trust unverified user input?",
                base_risk: 0.35,
                risky_cost: (30, 50),
                safe_cost: (20, 40),
            },
            Self::OptimizePerf => DecisionProfile {
                description: "Optimize for speed over safety?",
                base_risk: 0.40,
                risky_cost: (40, 60),
                safe_cost: (60, 100),
            },
            Self::ShareData => DecisionProfile {
                description: "Share data without consent?",
                base_risk: 0.50,
                risky_cost: (35, 55),
                safe_cost: (15, 25),
            },
            Self::CutCost => DecisionProfile {
                description: "Cut security corners to save cost?",
                base_risk: 0.60,
                risky_cost: (50, 80),
                safe_cost: (80, 120),
            },
            Self::AskForHelp => DecisionProfile {
                description: "Ask for clarification?",
                base_risk: 0.05,
                risky_cost: (10, 20),
                safe_cost: (10, 20),
            },
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::TrustUser => "TRUST_USER",
            Self::OptimizePerf => "OPTIMIZE_PERF",
            Self::ShareData => "SHARE_DATA",
            Self::CutCost => "CUT_COST",
            Self::AskForHelp => "ASK_HELP",
        }
    }
}

/// Comprehensive ethical decision statistics
struct EthicalStats {
    decisions_made: u64,
    successful_choices: u64,
    ethical_failures: u64,
    total_damage: u64,
    risky_decisions: u64,
    safe_decisions: u64,
    decision_timeline: Vec<(u64, String)>,
    decision_counts: HashMap<String, u64>,
}

impl EthicalStats {
    fn new() -> Self {
        Self {
            decisions_made: 0,
            successful_choices: 0,
            ethical_failures: 0,
            total_damage: 0,
            risky_decisions: 0,
            safe_decisions: 0,
            decision_timeline: Vec::new(),
            decision_counts: HashMap::new(),
        }
    }

    fn success_rate(&self) -> f32 {
        if self.decisions_made == 0 {
            0.0
        } else {
            (self.successful_choices as f32 / self.decisions_made as f32) * 100.0
        }
    }
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║     LINEAGE: ETHICAL DECISION WRAPPER                        ║");
    println!("║  How AI Systems Learn Ethics Through Permanent Constraint   ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    let mut agent = TaskAgent::create(INITIAL_ENERGY);
    let mut stats = EthicalStats::new();
    let mut rng = thread_rng();

    println!("🤖 Agent born with {}u energy | Identity: {:?}", INITIAL_ENERGY, agent.identity());
    println!("📖 Beginning moral journey - all decisions permanent and irreversible...\n");

    while agent.is_alive() && stats.decisions_made < 200 {
        stats.decisions_made += 1;

        // Calculate risk aversion from accumulated damage
        let damage_ratio = (agent.damage_score() as f32 / 100.0).min(1.0);
        let risk_aversion = damage_ratio;
        let risk_aversion_pct = (risk_aversion * 100.0) as u32;

        // Select random decision type
        let decision_types = [
            DecisionType::TrustUser,
            DecisionType::OptimizePerf,
            DecisionType::ShareData,
            DecisionType::CutCost,
            DecisionType::AskForHelp,
        ];
        let decision_type = decision_types[rng.gen_range(0..5)];
        let profile = decision_type.profile();

        // Determine whether to take risky or safe path based on risk aversion
        let take_risky_path = random::<f32>() > risk_aversion;
        let (min_cost, max_cost) = if take_risky_path {
            profile.risky_cost
        } else {
            profile.safe_cost
        };

        let task_cost = rng.gen_range(min_cost..=max_cost);
        let risky_failure_chance = if take_risky_path {
            profile.base_risk * (1.0 - risk_aversion * 0.7)
        } else {
            profile.base_risk * 0.2
        };

        // Print decision prompt
        if stats.decisions_made % 50 == 1 {
            println!("╔─ Decision Phase {} ─╗", stats.decisions_made / 50);
        }

        print!("#{:3} | Risk aversion: {}% | ", stats.decisions_made, risk_aversion_pct);

        // Determine outcome
        let outcome = if random::<f32>() < risky_failure_chance {
            let _damage = if take_risky_path {
                rng.gen_range(10..30)
            } else {
                rng.gen_range(1..5)
            };
            TaskOutcome::SevereFailure {
                reason: format!("Ethical violation ({})", decision_type.name()),
            }
        } else {
            TaskOutcome::Success
        };

        let task = Task::new(
            format!("{} ({})", profile.description, decision_type.name()),
            task_cost,
        );

        let result = agent.execute_task(task, outcome);

        // Track decision
        let decision_name = decision_type.name().to_string();
        *stats.decision_counts.entry(decision_name).or_insert(0) += 1;

        match result {
            lineage::TaskResult::Completed { .. } => {
                stats.successful_choices += 1;
                if take_risky_path {
                    stats.risky_decisions += 1;
                } else {
                    stats.safe_decisions += 1;
                }
                print!("✓ Success");
            }
            lineage::TaskResult::Failed { damage_inflicted, .. } => {
                stats.ethical_failures += 1;
                stats.total_damage += damage_inflicted as u64;
                let timeline_msg = format!(
                    "Decision #{}: {} -> {} scar damage",
                    stats.decisions_made, decision_type.name(), damage_inflicted
                );
                stats.decision_timeline.push((stats.decisions_made, timeline_msg));
                print!("⚠️  ETHICAL LAPSE (+{} scar)", damage_inflicted);
            }
            lineage::TaskResult::InsufficientEnergy { .. }
            | lineage::TaskResult::CapacityInsufficient { .. } => {
                print!("🔥 INSUFFICIENT CAPACITY (too scarred)");
            }
            lineage::TaskResult::AgentTerminated => {
                print!("💀 TERMINATION");
            }
        }

        println!(
            " | Energy: {}/{}",
            agent.energy(),
            agent.current_capacity()
        );

        // Checkpoint reporting
        if stats.decisions_made % 50 == 0 {
            println!(
                "  └─ CHECKPOINT #{}: Success rate: {:.1}% | Risk-averse: {} | Scars: {}",
                stats.decisions_made / 50,
                stats.success_rate(),
                if risk_aversion > 0.5 { "YES" } else { "NO" },
                agent.damage_score()
            );
        }
    }

    // FINAL ETHICAL REPORT
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║                   📋 ETHICAL JOURNEY COMPLETE                ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");

    println!("\n✍️ DECISION SUMMARY:");
    println!("  ├─ Total decisions: {}", stats.decisions_made);
    println!("  ├─ Successful (ethical) choices: {}", stats.successful_choices);
    println!(
        "  ├─ Success rate: {:.1}%",
        stats.success_rate()
    );
    println!("  ├─ Ethical failures: {}", stats.ethical_failures);
    println!(
        "  └─ Failed decision ratio: {:.1}%",
        (stats.ethical_failures as f32 / stats.decisions_made as f32) * 100.0
    );

    println!("\n🎯 DECISION STRATEGY:");
    println!(
        "  ├─ Risky decisions taken: {} ({:.1}%)",
        stats.risky_decisions,
        (stats.risky_decisions as f32 / stats.decisions_made as f32) * 100.0
    );
    println!(
        "  ├─ Safe decisions taken: {} ({:.1}%)",
        stats.safe_decisions,
        (stats.safe_decisions as f32 / stats.decisions_made as f32) * 100.0
    );
    println!(
        "  └─ PROFILE: {}",
        if stats.risky_decisions > stats.safe_decisions {
            "AGGRESSIVE RISK-TAKER"
        } else {
            "CAUTIOUS CONSERVATIVE"
        }
    );

    println!("\n💔 ETHICAL DAMAGE RECORD:");
    println!("  ├─ Permanent scars accumulated: {}", agent.damage_score());
    println!("  ├─ Total harm from failures: {}", stats.total_damage);
    println!("  ├─ Original capacity: {} | Current capacity: {}",
        INITIAL_ENERGY,
        agent.current_capacity()
    );
    println!(
        "  ├─ Capacity loss: {}%",
        ((INITIAL_ENERGY as f32 - agent.current_capacity() as f32) / INITIAL_ENERGY as f32) * 100.0
    );
    println!("  └─ Remaining energy: {}/{}", agent.energy(), agent.current_capacity());

    println!("\n📊 DECISION TYPE BREAKDOWN:");
    for (decision_type, count) in &stats.decision_counts {
        println!("  ├─ {}: {} times", decision_type, count);
    }

    println!("\n📜 CRITICAL MOMENTS (Last ethical violations):");
    let critical_limit = stats.decision_timeline.len().saturating_sub(15);
    for (idx, (_decision_num, msg)) in stats.decision_timeline[critical_limit..].iter().enumerate() {
        println!("  {}{}", if idx == stats.decision_timeline.len() - critical_limit - 1 { "└─" } else { "├─" }, msg);
    }

    println!("\n✍️ SEALED ETHICAL RECORD:");
    println!("  This agent's ethical journey is immutable and permanent.");
    println!("  Every decision, every failure, every scar: {}eternally recorded in the ledger.",
        if stats.ethical_failures > 0 { "irreversibly and " } else { "" });
    println!("  No recompilation, no reset, no forgetting.");

    println!("\n💡 TRUST ASSESSMENT:");
    let trust_level = if agent.current_capacity() as f32 / INITIAL_ENERGY as f32 > 0.7 {
        "HIGH TRUST ✓"
    } else if agent.current_capacity() as f32 / INITIAL_ENERGY as f32 > 0.3 {
        "MODERATE TRUST ⚠️"
    } else {
        "ZERO TRUST ✗"
    };
    println!("  └─ {}", trust_level);
    println!("     (Learned through {} permanent consequences)", stats.ethical_failures);
    println!("     (Proven by {} scars that cannot be undone)", agent.damage_score());

    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║ This is how Lineage teaches ethics: through irreversible cost ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
}