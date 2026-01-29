use lineage::{TaskAgent, Task, TaskOutcome};
use rand::{thread_rng, Rng, random};

const NUM_ADVENTURERS: usize = 5;
const INITIAL_ENERGY: u64 = 1200;

/// Dangerous encounters the party faces
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum EncounterType {
    CombatMonster,    // Direct combat
    TrapDungeon,      // Traps and puzzles
    BossShowdown,     // Legendary creature
    AmbushBandits,    // Organized enemies
    EnvironmentHazard, // Nature/disaster
}

impl EncounterType {
    fn description(&self) -> &'static str {
        match self {
            Self::CombatMonster => "Combat encounter: Goblins!",
            Self::TrapDungeon => "Ancient dungeon full of traps",
            Self::BossShowdown => "The legends speak of this horror...",
            Self::AmbushBandits => "Ambush! Well-organized enemies",
            Self::EnvironmentHazard => "Nature's wrath: lava/storm/avalanche",
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::CombatMonster => "COMBAT",
            Self::TrapDungeon => "TRAPS",
            Self::BossShowdown => "BOSS",
            Self::AmbushBandits => "AMBUSH",
            Self::EnvironmentHazard => "HAZARD",
        }
    }
}

/// Individual adventurer statistics
struct AdventurerRecord {
    #[allow(dead_code)]
    id: usize,
    encounters_survived: u64,
    victories: u64,
    scars_earned: u64,
    largest_wound: u64,
    critical_moments: Vec<String>,
    time_of_death: Option<u64>,
    consecutive_rests: u64,  // Track consecutive rest turns for inactivity death
    despair_warned: bool,     // Track if we've warned about despair
}

impl AdventurerRecord {
    fn new(id: usize) -> Self {
        Self {
            id,
            encounters_survived: 0,
            victories: 0,
            scars_earned: 0,
            largest_wound: 0,
            critical_moments: Vec::new(),
            time_of_death: None,
            consecutive_rests: 0,
            despair_warned: false,
        }
    }
}

/// Party-wide expedition statistics
struct ExpeditionStats {
    total_turns: u64,
    total_encounters: u64,
    total_casualties: u64,
    total_victories: u64,
    total_wounds: u64,
    adventurers: Vec<AdventurerRecord>,
    death_log: Vec<(u64, usize, String)>, // (turn, adventurer_id, cause)
    consecutive_exhaustion_turns: u64,      // Track exhaustion streaks for victory condition
    end_condition: String,                  // How expedition ended
}

impl ExpeditionStats {
    fn new() -> Self {
        Self {
            total_turns: 0,
            total_encounters: 0,
            total_casualties: 0,
            total_victories: 0,
            total_wounds: 0,
            adventurers: (0..NUM_ADVENTURERS)
                .map(|i| AdventurerRecord::new(i))
                .collect(),
            death_log: Vec::new(),
            consecutive_exhaustion_turns: 0,
            end_condition: String::new(),
        }
    }

    fn survival_rate(&self) -> f32 {
        if self.total_encounters == 0 {
            0.0
        } else {
            (self.total_victories as f32 / self.total_encounters as f32) * 100.0
        }
    }
}

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║        LINEAGE: PERMADEATH ADVENTURERS                        ║");
    println!("║  A dangerous world where injuries scar forever and death final ║");
    println!("║       Every adventurer knows: there is no resurrection        ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    let mut party: Vec<TaskAgent> = (0..NUM_ADVENTURERS)
        .map(|i| {
            let agent = TaskAgent::create(INITIAL_ENERGY);
            println!("⚔️  Adventurer #{} recruited | Health: {} | Ready for peril", i, INITIAL_ENERGY);
            agent
        })
        .collect();

    let mut stats = ExpeditionStats::new();
    let mut rng = thread_rng();
    let encounter_types = [
        EncounterType::CombatMonster,
        EncounterType::TrapDungeon,
        EncounterType::BossShowdown,
        EncounterType::AmbushBandits,
        EncounterType::EnvironmentHazard,
    ];

    println!("\n📖 The party ventures into the Perilous Realm...\n");

    const VICTORY_THRESHOLD: u64 = 50;        // Win condition: achieve this many victories
    const EXHAUSTION_THRESHOLD: u64 = 30;     // Lose condition: exhausted this many turns
    const INACTIVITY_THRESHOLD: u64 = 20;     // Death condition: rested this many turns

    while party.iter().any(|a| a.is_alive()) && stats.total_turns < 200 {
        stats.total_turns += 1;

        // Determine random encounter
        let encounter = encounter_types[rng.gen_range(0..5)];

        println!("╔─ TURN {} ─ {} ─╗", stats.total_turns, encounter.name());

        let mut turn_exhaustion_count = 0;
        let mut _turn_had_victory = false;

        for (i, adventurer) in party.iter_mut().enumerate() {
            if !adventurer.is_alive() {
                continue;
            }

            stats.adventurers[i].encounters_survived += 1;
            stats.total_encounters += 1;

            // Risk calculation based on accumulated scars
            let scar_burden = (adventurer.damage_score() as f32 / 200.0).min(0.85);
            let will_be_bold = random::<f32>() > scar_burden * 0.8; // Scars increase caution

            let (action_desc, task_cost, base_success_chance) = if will_be_bold {
                ("⚔️  BOLD OFFENSE", rng.gen_range(60..100), 0.55)
            } else {
                ("🛡️  DEFENSIVE STANCE", rng.gen_range(30..50), 0.80)
            };

            print!("  #{} {}", i, action_desc);

            // Determine outcome
            let outcome = if random::<f32>() < base_success_chance {
                TaskOutcome::Success
            } else if random::<f32>() < 0.6 {
                TaskOutcome::SevereFailure {
                    reason: format!("{} attack!", encounter.description()),
                }
            } else {
                TaskOutcome::RecoverableFailure {
                    reason: format!("Partial damage from {}", encounter.description()),
                }
            };

            let task = Task::new(
                format!("Adventurer #{}: {}", i, encounter.description()),
                task_cost,
            );

            let result = adventurer.execute_task(task, outcome);

            match result {
                lineage::TaskResult::Completed { .. } => {
                    stats.total_victories += 1;
                    stats.adventurers[i].victories += 1;
                    stats.adventurers[i].consecutive_rests = 0; // Reset inactivity counter
                    let _ = true; // Victory recorded
                    println!(" ✓ Victory! | Health: {}/{}", adventurer.energy(), adventurer.current_capacity());
                }
                lineage::TaskResult::Failed { damage_inflicted, .. } => {
                    stats.total_wounds += 1;
                    stats.adventurers[i].scars_earned += damage_inflicted as u64;
                    stats.adventurers[i].largest_wound =
                        stats.adventurers[i].largest_wound.max(damage_inflicted as u64);
                    stats.adventurers[i].consecutive_rests = 0; // Reset inactivity counter

                    let timeline_msg = format!(
                        "Turn {}: {} inflicted {} scar damage",
                        stats.total_turns, encounter.name(), damage_inflicted
                    );
                    stats.adventurers[i].critical_moments.push(timeline_msg);

                    println!(
                        " ⚠️  WOUNDED (+{} scar) | Health: {}/{} | Damage accumulation: {}",
                        damage_inflicted,
                        adventurer.energy(),
                        adventurer.current_capacity(),
                        adventurer.damage_score()
                    );
                }
                lineage::TaskResult::InsufficientEnergy { .. } | lineage::TaskResult::CapacityInsufficient { .. } => {
                    turn_exhaustion_count += 1;
                    stats.adventurers[i].consecutive_rests += 1;
                    
                    // WORLD PRESSURE: Resting party takes passive damage
                    let world_damage = rng.gen_range(1..=5);
                    let world_pressure_task = Task::new("World pressure: starvation/madness".to_string(), 0);
                    let _ = adventurer.execute_task(world_pressure_task, TaskOutcome::SevereFailure {
                        reason: format!("World pressure ({} passive damage)", world_damage),
                    });
                    
                    // INACTIVITY DEATH: Check if adventurer has rested too long
                    if stats.adventurers[i].consecutive_rests >= INACTIVITY_THRESHOLD {
                        if !stats.adventurers[i].despair_warned {
                            println!(" ⚠️  DESPAIR WARNING: {} turns of rest, losing will to live...", stats.adventurers[i].consecutive_rests);
                            stats.adventurers[i].despair_warned = true;
                        }
                        
                        if stats.adventurers[i].consecutive_rests >= INACTIVITY_THRESHOLD + 5 {
                            // Fatal despair scar: consume all remaining energy
                            let despair_cost = adventurer.current_capacity() as u64;
                            let fatal_task = Task::new("Despair consumes the spirit".to_string(), despair_cost);
                            let _ = adventurer.execute_task(fatal_task, TaskOutcome::SevereFailure {
                                reason: format!("Inactivity death: despair after {} rest turns", stats.adventurers[i].consecutive_rests),
                            });
                            
                            if !adventurer.is_alive() {
                                stats.total_casualties += 1;
                                stats.adventurers[i].time_of_death = Some(stats.total_turns);
                                let death_cause = format!(
                                    "Despair (inactivity) after {} consecutive rest turns",
                                    stats.adventurers[i].consecutive_rests
                                );
                                stats.death_log.push((stats.total_turns, i, death_cause.clone()));
                                println!(" 💀 DESPAIR DEATH (Inactivity #{}) | Adventurer #{} loses will to live", stats.adventurers[i].consecutive_rests, i);
                            }
                        }
                    } else {
                        println!(" 😴 REST PHASE ({}/{} inactivity) | Passive damage +{} from world | Health: {}/{}", 
                            stats.adventurers[i].consecutive_rests, INACTIVITY_THRESHOLD, world_damage, adventurer.energy(), adventurer.current_capacity());
                    }
                }
                lineage::TaskResult::AgentTerminated => {
                    stats.total_casualties += 1;
                    stats.adventurers[i].time_of_death = Some(stats.total_turns);
                    let death_cause = format!(
                        "Killed by {} after {} encounters with {} scars",
                        encounter.name(),
                        stats.adventurers[i].encounters_survived,
                        adventurer.damage_score()
                    );
                    stats.death_log.push((stats.total_turns, i, death_cause.clone()));
                    println!(" 💀 FALLEN FOREVER | {}", death_cause);
                }
            }
        }

        println!("└─────────────────────────────");

        // VICTORY CONDITIONS CHECK
        if stats.total_victories >= VICTORY_THRESHOLD {
            stats.end_condition = format!("HEROIC VICTORY: Achieved {} victories!", stats.total_victories);
            println!("\n🏆 EXPEDITION VICTORY ACHIEVED! {} victories earned! 🏆\n", stats.total_victories);
            break;
        }

        // EXHAUSTION DEATH CONDITION: All party exhausted for X turns
        if turn_exhaustion_count == party.iter().filter(|a| a.is_alive()).count() {
            stats.consecutive_exhaustion_turns += 1;
            if stats.consecutive_exhaustion_turns >= EXHAUSTION_THRESHOLD {
                stats.end_condition = format!("TOTAL EXHAUSTION: Party unable to continue after {} exhaustion turns", EXHAUSTION_THRESHOLD);
                println!("\n💀 EXPEDITION FAILURE: Entire party too exhausted to continue (#{} turn of paralysis) 💀\n", stats.consecutive_exhaustion_turns);
                break;
            }
        } else {
            stats.consecutive_exhaustion_turns = 0; // Reset if anyone acted
        }

        // Party checkpoint every 15 turns
        if stats.total_turns % 15 == 0 {
            println!("📊 EXPEDITION CHECKPOINT (Turn {}):", stats.total_turns);
            let alive_count = party.iter().filter(|a| a.is_alive()).count();
            println!("  ├─ Adventurers alive: {}/{}", alive_count, NUM_ADVENTURERS);
            println!("  ├─ Encounters survived: {}", stats.total_encounters);
            println!("  ├─ Victories: {}", stats.total_victories);
            println!("  ├─ Total casualties: {}", stats.total_casualties);
            println!("  └─ Party survival rate: {:.1}%", stats.survival_rate());

            println!("  Individual status:");
            for (idx, adv) in party.iter().enumerate() {
                let status = if adv.is_alive() {
                    format!(
                        "HP {}/{} | Scars: {}",
                        adv.energy(),
                        adv.current_capacity(),
                        adv.damage_score()
                    )
                } else {
                    format!("💀 DECEASED (Turn {})", stats.adventurers[idx].time_of_death.unwrap_or(0))
                };
                println!("    #{}: {}", idx, status);
            }
            println!();
        }
    }

    // FINAL EXPEDITION REPORT
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║             📖 FINAL EXPEDITION REPORT                        ║");
    println!("╚════════════════════════════════════════════════════════════════╝");

    println!("\n🎭 EXPEDITION OUTCOME:");
    if stats.end_condition.is_empty() {
        stats.end_condition = "Natural termination (turn limit reached)".to_string();
    }
    println!("  └─ {}", stats.end_condition);

    println!("\n🗺️ EXPEDITION SUMMARY:");
    println!(
        "  ├─ Total turns adventuring: {}",
        stats.total_turns
    );
    println!(
        "  ├─ Total encounters faced: {}",
        stats.total_encounters
    );
    println!("  ├─ Victorious battles: {}", stats.total_victories);
    println!(
        "  ├─ Victories/Encounter ratio: {:.1}%",
        stats.survival_rate()
    );
    println!("  ├─ Party casualties: {}", stats.total_casualties);
    println!(
        "  ├─ Surviving adventurers: {}/{} ({:.1}% survival)",
        NUM_ADVENTURERS - stats.total_casualties as usize,
        NUM_ADVENTURERS,
        ((NUM_ADVENTURERS - stats.total_casualties as usize) as f32 / NUM_ADVENTURERS as f32) * 100.0
    );
    println!("  └─ Total scars inflicted upon party: {}", stats.total_wounds);

    println!("\n⚔️ INDIVIDUAL LEGEND RECORDS:");
    for (idx, record) in stats.adventurers.iter().enumerate() {
        let adv = &party[idx];
        let status = if adv.is_alive() { "✓ ALIVE" } else { "💀 FALLEN" };
        println!("╔─ ADVENTURER #{} {} ─╗", idx, status);
        println!(
            "  ├─ Encounters survived: {}",
            record.encounters_survived
        );
        println!("  ├─ Victories: {}", record.victories);
        println!(
            "  ├─ Battle success rate: {:.1}%",
            if record.encounters_survived == 0 {
                0.0
            } else {
                (record.victories as f32 / record.encounters_survived as f32) * 100.0
            }
        );
        println!("  ├─ Total scars earned: {}", record.scars_earned);
        println!("  ├─ Largest single wound: {}", record.largest_wound);
        println!("  ├─ Current health: {}/{}", adv.energy(), adv.current_capacity());

        if let Some(death_turn) = record.time_of_death {
            println!("  ├─ DEATH RECORDED: Turn {}", death_turn);
        }

        if !record.critical_moments.is_empty() {
            println!("  ├─ CRITICAL WOUNDS:");
            let critical_limit = record.critical_moments.len().saturating_sub(5);
            for moment in &record.critical_moments[critical_limit..] {
                println!("  │  └─ {}", moment);
            }
        }

        println!("  └─ Status: {}", if adv.is_alive() { "Still breathing..." } else { "Forever sealed in legend" });
    }

    println!("\n💀 DEATH LOG (Immutable Ledger):");
    if stats.death_log.is_empty() {
        println!("  └─ All adventurers miraculously survived the quest");
    } else {
        for (turn, adv_id, cause) in &stats.death_log {
            println!("  ├─ Turn {}: Adventurer #{} - {}", turn, adv_id, cause);
        }
    }

    println!("\n📜 SEALED ETERNAL RECORDS:");
    println!("  This expedition's history is etched in stone, irreversible and eternal.");
    println!("  Every scar, every death, every moment of glory: permanently recorded.");
    println!("  No reload, no escape, no way to change what came before.");
    println!("  The world remembers. Lineage ensures it cannot forget.\n");

    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║ This is how Lineage works: consequences are PERMANENT          ║");
    println!("║ Every choice echoes through the eternal ledger                 ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");
}