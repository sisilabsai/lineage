use lineage::{TaskAgent, Task, TaskOutcome};
use rand::{thread_rng, Rng, random};
use std::collections::HashMap;

const NUM_AGENTS: usize = 8;
const INITIAL_POWER: u64 = 1500;

/// Types of proposals the network must vote on
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ProposalType {
    ChangeProtocol,    // Modify core rules
    AcceptTransaction,  // Process large transfer
    AllocateFunds,      // Distribute treasury
    SecurityUpgrade,    // Critical patch
    PolicyChange,       // Governance modification
}

impl ProposalType {
    fn description(&self) -> &'static str {
        match self {
            Self::ChangeProtocol => "Protocol Change: Increase block size to 8MB",
            Self::AcceptTransaction => "Accept $10M transaction from exchange",
            Self::AllocateFunds => "Allocate 100K tokens to development fund",
            Self::SecurityUpgrade => "Emergency patch: Fix critical vulnerability",
            Self::PolicyChange => "Voting threshold change: 60% → 50% consensus",
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::ChangeProtocol => "PROTOCOL",
            Self::AcceptTransaction => "TRANSACTION",
            Self::AllocateFunds => "FUNDS",
            Self::SecurityUpgrade => "SECURITY",
            Self::PolicyChange => "POLICY",
        }
    }

    fn default_risk(&self) -> f32 {
        match self {
            Self::ChangeProtocol => 0.70,      // Very controversial
            Self::AcceptTransaction => 0.50,   // Moderate controversy
            Self::AllocateFunds => 0.45,       // Moderate
            Self::SecurityUpgrade => 0.15,     // Low controversy (safety)
            Self::PolicyChange => 0.65,        // Controversial
        }
    }
}

/// Voting agent with decision history
struct VotingAgent {
    agent: TaskAgent,
    votes_cast: u64,
    votes_for: u64,
    votes_against: u64,
    scars_from_controversy: u64,
    voting_history: Vec<String>,
    #[allow(dead_code)]
    power_loss_timeline: Vec<(u64, f32)>, // (turn, voting_power_pct)
}

impl VotingAgent {
    fn new(agent: TaskAgent) -> Self {
        Self {
            agent,
            votes_cast: 0,
            votes_for: 0,
            votes_against: 0,
            scars_from_controversy: 0,
            voting_history: Vec::new(),
            power_loss_timeline: Vec::new(),
        }
    }

    fn voting_power_percent(&self) -> f32 {
        let capacity_loss = (self.agent.damage_score() as f32 / INITIAL_POWER as f32).min(1.0);
        ((1.0 - capacity_loss) * 100.0).max(0.0)
    }

    fn is_silent(&self) -> bool {
        self.agent.current_capacity() < 50
    }
}

/// Proposal voting round
struct VotingRound {
    #[allow(dead_code)]
    turn: u64,
    proposal: ProposalType,
    votes_for: u64,
    votes_against: u64,
    #[allow(dead_code)]
    votes_abstain: u64,
    consensus_percent: f32,
    is_controversial: bool,
    #[allow(dead_code)]
    dissent_scarring: HashMap<usize, u64>,  // agent_id -> scar damage from controversy
    #[allow(dead_code)]
    outcome: String,
}

/// System-wide voting statistics
struct ConsensusStats {
    total_rounds: u64,
    consensus_rounds: u64,
    controversial_rounds: u64,
    failed_votes: u64,
    total_scarring: u64,
    voting_rounds: Vec<VotingRound>,
    power_concentration: Vec<f32>,  // voting power inequality over time
}

impl ConsensusStats {
    fn new() -> Self {
        Self {
            total_rounds: 0,
            consensus_rounds: 0,
            controversial_rounds: 0,
            failed_votes: 0,
            total_scarring: 0,
            voting_rounds: Vec::new(),
            power_concentration: Vec::new(),
        }
    }

    fn consensus_quality(&self) -> f32 {
        if self.total_rounds == 0 {
            0.0
        } else {
            (self.consensus_rounds as f32 / self.total_rounds as f32) * 100.0
        }
    }
}

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║     LINEAGE: DISTRIBUTED CONSENSUS VOTING SYSTEM              ║");
    println!("║  History weighs forever: controversial votes scar dissenters   ║");
    println!("║      Scarred agents lose power → system learns caution        ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    let mut agents: Vec<VotingAgent> = (0..NUM_AGENTS)
        .map(|i| {
            let agent = TaskAgent::create(INITIAL_POWER);
            println!("🔗 Agent #{} joins network | Voting power: 100% | Ready to govern", i);
            VotingAgent::new(agent)
        })
        .collect();

    let mut stats = ConsensusStats::new();
    let mut rng = thread_rng();
    let proposal_types = [
        ProposalType::ChangeProtocol,
        ProposalType::AcceptTransaction,
        ProposalType::AllocateFunds,
        ProposalType::SecurityUpgrade,
        ProposalType::PolicyChange,
    ];

    println!("\n📜 Network begins voting on proposals...\n");

    const CONSENSUS_THRESHOLD: f32 = 66.67;  // 2/3 consensus = low cost
    const MAJORITY_THRESHOLD: f32 = 50.0;    // Simple majority = controversy costs
    #[allow(dead_code)]
    const CONTROVERSY_THRESHOLD: f32 = 30.0; // < 30% agreement = major scarring

    for round in 1..=25 {
        stats.total_rounds = round;
        let proposal = proposal_types[rng.gen_range(0..5)];
        let proposal_risk = proposal.default_risk();

        println!("╔─ ROUND {} ─ {} ─╗", round, proposal.name());
        println!("  Proposal: {}", proposal.description());
        println!("  Risk baseline: {:.0}%\n", proposal_risk * 100.0);

        let mut votes_for = 0u64;
        let mut votes_against = 0u64;
        let mut votes_abstain = 0u64;
        let mut dissent_scarring: HashMap<usize, u64> = HashMap::new();
        let mut _active_voters = 0u64;

        // VOTING PHASE
        for (i, agent) in agents.iter_mut().enumerate() {
            if !agent.agent.is_alive() {
                continue;
            }

            // Check if agent is too scarred to vote (silent)
            if agent.is_silent() {
                println!("  #{} 🤐 TOO SCARRED TO VOTE (silent)", i);
                votes_abstain += 1;
                continue;
            }

            _active_voters += 1;

            // Scarred agents become more conservative
            let scarring_influence = (agent.agent.damage_score() as f32 / INITIAL_POWER as f32).min(1.0);
            let conservative_bias = scarring_influence * 0.7;  // Scars → oppose risky proposals
            let effective_risk = proposal_risk * (1.0 - conservative_bias * 0.5);

            // Agent votes
            let vote_for = random::<f32>() > effective_risk;

            if vote_for {
                votes_for += 1;
                agent.votes_for += 1;
                println!("  #{} ✓ VOTES FOR | Power: {:.0}%", i, agent.voting_power_percent());
            } else {
                votes_against += 1;
                agent.votes_against += 1;
                println!("  #{} ✗ VOTES AGAINST | Power: {:.0}%", i, agent.voting_power_percent());
            }

            agent.votes_cast += 1;
        }

        // CALCULATE CONSENSUS LEVEL
        let total_votes = votes_for + votes_against;
        let consensus_pct = if total_votes > 0 {
            (votes_for as f32 / total_votes as f32) * 100.0
        } else {
            50.0
        };

        println!("\n  Vote tallies:");
        println!("    For: {} | Against: {} | Abstain: {}", votes_for, votes_against, votes_abstain);
        println!("    Consensus: {:.1}%\n", consensus_pct);

        let (is_controversial, consensus_cost) = if consensus_pct >= CONSENSUS_THRESHOLD {
            // CONSENSUS: Smooth passing, minimal cost
            stats.consensus_rounds += 1;
            (false, 10u64)
        } else if consensus_pct >= MAJORITY_THRESHOLD {
            // MAJORITY: Passed but controversial
            stats.controversial_rounds += 1;
            (true, 50u64)
        } else {
            // FAILED: Major controversy
            stats.failed_votes += 1;
            (true, 100u64)
        };

        // SCARRING PHASE: Controversial proposals scar dissenters
        let mut _scarring_applied = false;
        if is_controversial {
            for (i, agent) in agents.iter_mut().enumerate() {
                if !agent.agent.is_alive() || agent.is_silent() {
                    continue;
                }

                if agent.votes_against > 0 && random::<f32>() < (1.0 - consensus_pct / 100.0) {
                    // Dissenters take scar damage based on controversy
                    let scar_damage = consensus_cost / 3;
                    
                    // Apply scarring via failed task
                    let scar_task = Task::new(
                        format!("Controversy scar from {} vote", proposal.name()),
                        50,
                    );
                    let _ = agent.agent.execute_task(scar_task, TaskOutcome::SevereFailure {
                        reason: format!("Lost minority vote: {:.1}% consensus", consensus_pct),
                    });

                    dissent_scarring.insert(i, scar_damage);
                    agent.scars_from_controversy += scar_damage;
                    stats.total_scarring += scar_damage;
                    let _ = _scarring_applied; // Marked as applied

                    println!("  ⚠️  Agent #{} SCARRED by controversy (+{} scar) | New power: {:.0}%",
                        i, scar_damage, agent.voting_power_percent());
                }
            }
        }

        // CONSENSUS TASK: Agents pay cost for participating
        for (_i, agent) in agents.iter_mut().enumerate() {
            if !agent.agent.is_alive() || agent.votes_cast == 0 {
                continue;
            }

            let cost = if is_controversial { 80 } else { 30 };
            let task = Task::new(format!("Consensus vote: {}", proposal.name()), cost);

            let outcome = if is_controversial && random::<f32>() < 0.3 {
                TaskOutcome::RecoverableFailure {
                    reason: format!("Controversial proposal stressed network"),
                }
            } else {
                TaskOutcome::Success
            };

            let _ = agent.agent.execute_task(task, outcome);
        }

        let outcome_msg = if consensus_pct >= CONSENSUS_THRESHOLD {
            format!("✓ PASSED WITH CONSENSUS ({:.1}%)", consensus_pct)
        } else if consensus_pct >= MAJORITY_THRESHOLD {
            format!("✓ PASSED WITH MAJORITY ({:.1}%) - CONTROVERSIAL", consensus_pct)
        } else {
            format!("✗ FAILED ({:.1}% support) - MAJOR DIVISION", consensus_pct)
        };

        println!("  └─ OUTCOME: {}\n", outcome_msg);

        // Record voting round
        stats.voting_rounds.push(VotingRound {
            turn: round,
            proposal,
            votes_for,
            votes_against,
            votes_abstain,
            consensus_percent: consensus_pct,
            is_controversial,
            dissent_scarring: dissent_scarring.clone(),
            outcome: outcome_msg,
        });

        // Track power concentration (Gini coefficient approximation)
        let mut powers: Vec<f32> = agents.iter()
            .filter(|a| a.agent.is_alive())
            .map(|a| a.voting_power_percent())
            .collect();
        powers.sort_by(|a, b| b.partial_cmp(a).unwrap());
        
        let top_3_power = powers.iter().take(3).sum::<f32>();
        let total_power = powers.iter().sum::<f32>();
        let concentration = if total_power > 0.0 { top_3_power / total_power * 100.0 } else { 0.0 };
        stats.power_concentration.push(concentration);

        // CHECKPOINT every 5 rounds
        if round % 5 == 0 {
            println!("📊 GOVERNANCE CHECKPOINT (Round {}):", round);
            println!("  ├─ Consensus rounds: {} / {}", stats.consensus_rounds, stats.total_rounds);
            println!("  ├─ Controversial rounds: {}", stats.controversial_rounds);
            println!("  ├─ Failed votes: {}", stats.failed_votes);
            println!("  ├─ Total scarring applied: {}", stats.total_scarring);
            println!("  ├─ Power concentration (top 3): {:.1}%", concentration);

            println!("  └─ Agent Status:");
            for (idx, agent) in agents.iter().enumerate() {
                if !agent.agent.is_alive() {
                    println!("    #{}: 💀 TERMINATED", idx);
                } else if agent.is_silent() {
                    println!("    #{}: 🤐 SILENT (power {:.0}%)", idx, agent.voting_power_percent());
                } else {
                    println!("    #{}: Votes: {} | Scars: {} | Power: {:.0}%",
                        idx, agent.votes_cast, agent.agent.damage_score(), agent.voting_power_percent());
                }
            }
            println!();
        }
    }

    // FINAL CONSENSUS REPORT
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║           📋 DISTRIBUTED CONSENSUS FINAL REPORT               ║");
    println!("╚════════════════════════════════════════════════════════════════╝");

    println!("\n🗳️ VOTING SUMMARY:");
    println!("  ├─ Total rounds: {}", stats.total_rounds);
    println!("  ├─ Consensus rounds (67%+): {} ({:.1}%)", 
        stats.consensus_rounds, 
        (stats.consensus_rounds as f32 / stats.total_rounds as f32) * 100.0);
    println!("  ├─ Controversial rounds (50-67%): {}", stats.controversial_rounds);
    println!("  ├─ Failed votes (<50%): {}", stats.failed_votes);
    println!("  ├─ Consensus quality: {:.1}%", stats.consensus_quality());
    println!("  └─ Total scarring from disagreements: {}", stats.total_scarring);

    println!("\n💡 VOTING BEHAVIOR EVOLUTION:");
    println!("  ├─ Power concentration (round 1): {:.1}%", stats.power_concentration.first().unwrap_or(&0.0));
    println!("  ├─ Power concentration (round 5): {:.1}%", 
        stats.power_concentration.get(4).unwrap_or(&0.0));
    println!("  ├─ Power concentration (final): {:.1}%", 
        stats.power_concentration.last().unwrap_or(&0.0));
    
    if stats.power_concentration.last().unwrap_or(&0.0) > stats.power_concentration.first().unwrap_or(&0.0) {
        println!("  └─ TREND: Power centralizing (dissenters scarred into silence)");
    } else {
        println!("  └─ TREND: Power distributed fairly");
    }

    println!("\n🤖 INDIVIDUAL AGENT RECORDS:");
    let mut surviving = 0;
    let mut silent = 0;
    let mut terminated = 0;

    for (idx, agent) in agents.iter().enumerate() {
        println!("╔─ AGENT #{} ", idx);
        
        if !agent.agent.is_alive() {
            println!("💀 TERMINATED");
            terminated += 1;
        } else if agent.is_silent() {
            println!("🤐 SILENT (too scarred to vote)");
            silent += 1;
        } else {
            println!("✓ ACTIVE");
            surviving += 1;
        }
        
        println!("  ├─ Votes cast: {}", agent.votes_cast);
        println!("  ├─ Votes for: {} | Votes against: {}", agent.votes_for, agent.votes_against);
        if agent.votes_cast > 0 {
            println!("  ├─ Dissent rate: {:.1}%", 
                (agent.votes_against as f32 / agent.votes_cast as f32) * 100.0);
        }
        println!("  ├─ Scars from controversy: {}", agent.scars_from_controversy);
        println!("  ├─ Current damage: {}", agent.agent.damage_score());
        println!("  ├─ Voting power remaining: {:.1}%", agent.voting_power_percent());
        println!("  └─ Status: {}", 
            if !agent.agent.is_alive() { "Terminated" } 
            else if agent.is_silent() { "Silent witness" }
            else { "Active participant" });
    }

    println!("\n🏛️ NETWORK RESILIENCE:");
    println!("  ├─ Agents still participating: {} / {}", surviving, NUM_AGENTS);
    println!("  ├─ Agents too scarred to vote: {} / {}", silent, NUM_AGENTS);
    println!("  ├─ Agents terminated: {} / {}", terminated, NUM_AGENTS);
    println!("  └─ Effective governance: {:.1}%", 
        (surviving as f32 / NUM_AGENTS as f32) * 100.0);

    println!("\n📜 PROPOSAL OUTCOME HISTORY:");
    let mut consensus_count = 0;
    let mut controversial_count = 0;
    let mut failed_count = 0;
    
    for (idx, round) in stats.voting_rounds.iter().enumerate() {
        if round.is_controversial {
            if round.votes_for < round.votes_against {
                failed_count += 1;
                println!("  ├─ Round {}: {} - {} (FAILED)", idx + 1, round.proposal.name(), 
                    format!("{:.1}%", round.consensus_percent));
            } else {
                controversial_count += 1;
                println!("  ├─ Round {}: {} - {} (CONTROVERSIAL PASS)", idx + 1, round.proposal.name(),
                    format!("{:.1}%", round.consensus_percent));
            }
        } else {
            consensus_count += 1;
            println!("  ├─ Round {}: {} - {} (CONSENSUS)", idx + 1, round.proposal.name(),
                format!("{:.1}%", round.consensus_percent));
        }
    }

    println!("\n⛓️ DISTRIBUTED LEDGER (IMMUTABLE):");
    println!("  This voting history is permanently recorded in the blockchain.");
    println!("  ├─ {} consensus decisions (cost: cheap, stable)", consensus_count);
    println!("  ├─ {} controversial passes (cost: scarring to dissenters)", controversial_count);
    println!("  └─ {} failed votes (cost: major scarring & power loss)", failed_count);
    println!("\n  Every scar is visible to all. Every dissent is recorded.");
    println!("  Scarred agents lose influence. System learns caution.");
    println!("  This is how governance evolves under Lineage.");

    println!("\n💭 NETWORK LEARNING:");
    if stats.consensus_quality() > 70.0 {
        println!("  The network achieved HIGH CONSENSUS QUALITY.");
        println!("  Proposals tended to be non-controversial.");
        println!("  Agents trusted each other's judgment.");
    } else if stats.consensus_quality() > 50.0 {
        println!("  The network achieved MODERATE consensus.");
        println!("  Some controversial decisions scarred dissenters.");
        println!("  Power began concentrating among survivors.");
    } else {
        println!("  The network experienced DEEP DIVISION.");
        println!("  Repeated controversial votes scarred many agents.");
        println!("  Most agents are now silent or terminated.");
        println!("  Future proposals will pass more easily (fewer voters).");
    }

    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║ This is how Lineage enables trustless governance:              ║");
    println!("║ History is permanent. Consequences are irreversible.           ║");
    println!("║ Scarring teaches caution. Power flows to consistent winners.   ║");
    println!("║ The system organically learns what it values.                  ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");
}
