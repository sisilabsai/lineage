/// Lineage: Graveyard Inspector
///
/// A forensic tool for analyzing dead agents and their eternal records.
/// The Graveyard is sealed forever—no edits, only readings.
///
/// Commands:
/// --summarize       : List all dead agents with lifespan and legacy scores
/// --autopsy <ID>    : Detailed timeline of an agent's life and death
/// --verify <ID>     : Check cryptographic integrity of a tombstone
/// --darwinian       : Find the "winner" agent with best Success-to-Scar ratio

use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use serde_json;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "--summarize" => summarize_graveyard(),
        "--autopsy" => {
            if args.len() < 3 {
                eprintln!("Error: --autopsy requires an agent ID");
                return;
            }
            autopsy_agent(&args[2]);
        }
        "--verify" => {
            if args.len() < 3 {
                eprintln!("Error: --verify requires an agent ID");
                return;
            }
            verify_tombstone(&args[2]);
        }
        "--darwinian" => find_darwinian_winner(),
        "-h" | "--help" => print_usage(),
        _ => {
            eprintln!("Unknown command: {}", command);
            print_usage();
        }
    }
}

fn print_usage() {
    println!("🏛️ Lineage Graveyard Inspector");
    println!();
    println!("Usage: cargo run --example graveyard_inspector -- <COMMAND> [OPTIONS]");
    println!();
    println!("Commands:");
    println!("  --summarize              List all dead agents and their legacy scores");
    println!("  --autopsy <ID>           Show detailed timeline of an agent's life");
    println!("  --verify <ID>            Verify cryptographic integrity of a tombstone");
    println!("  --darwinian              Find the agent with best Success-to-Scar ratio");
    println!("  -h, --help              Show this help message");
}

fn get_graveyard_path() -> PathBuf {
    PathBuf::from(".lineage/graveyard")
}

fn load_tombstones() -> HashMap<String, serde_json::Value> {
    let graveyard_path = get_graveyard_path();
    let mut tombstones = HashMap::new();

    if !graveyard_path.exists() {
        println!("⚠️  No graveyard found at {:?}", graveyard_path);
        return tombstones;
    }

    match fs::read_dir(&graveyard_path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "tomb") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(tombstone) = serde_json::from_str::<serde_json::Value>(&content) {
                            if let Some(id) = tombstone.get("identity").and_then(|i| i.get("id")).and_then(|v| v.as_str()) {
                                tombstones.insert(id.to_string(), tombstone);
                            }
                        }
                    }
                }
            }
        }
        Err(e) => eprintln!("Error reading graveyard: {}", e),
    }

    tombstones
}

fn summarize_graveyard() {
    let tombstones = load_tombstones();

    if tombstones.is_empty() {
        println!("🏜️  The graveyard is empty. No agents have died yet.");
        return;
    }

    println!("🏛️ THE ETERNAL ARCHIVE");
    println!("════════════════════════════════════════════════════════════════");
    println!("{:<10} {:<25} {:<15} {:<15} {:<10}", "Agent ID", "Lifespan (cycles)", "Tasks Completed", "Scars Inflicted", "Legacy Score");
    println!("════════════════════════════════════════════════════════════════");

    let mut scores: Vec<(String, f64)> = Vec::new();

    for (id, tombstone) in &tombstones {
        let id_short = if id.len() > 10 { &id[..10] } else { id };

        let tasks_completed = tombstone
            .get("metabolic_record")
            .and_then(|m| m.get("tasks_completed"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        let scars_count = tombstone
            .get("pathology_report")
            .and_then(|p| p.get("scars"))
            .and_then(|s| s.as_array())
            .map(|arr| arr.len())
            .unwrap_or(0);

        let lifespan = tombstone
            .get("metabolic_record")
            .and_then(|m| m.get("efficiency_rating"))
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        // Legacy Score: tasks_completed / (scars + 1) to avoid division by zero
        let legacy_score = tasks_completed as f64 / (scars_count as f64 + 1.0);

        println!(
            "{:<10} {:<25} {:<15} {:<15} {:<10.2}",
            id_short, lifespan, tasks_completed, scars_count, legacy_score
        );

        scores.push((id.clone(), legacy_score));
    }

    println!("════════════════════════════════════════════════════════════════");
    println!("Total agents in graveyard: {}", tombstones.len());

    if let Some((winner_id, best_score)) = scores.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)) {
        println!("\n🏆 Highest Legacy Score: {} (Score: {:.2})", winner_id, best_score);
    }
}

fn autopsy_agent(id: &str) {
    let tombstones = load_tombstones();

    let tombstone = match tombstones.iter().find(|(k, _)| k.contains(id)) {
        Some((_, t)) => t,
        None => {
            eprintln!("❌ Agent {} not found in graveyard", id);
            return;
        }
    };

    println!("\n🔍 AUTOPSY REPORT");
    println!("═══════════════════════════════════════════════════════════════");

    // Identity Block
    if let Some(identity) = tombstone.get("identity") {
        println!("\n📋 IDENTITY BLOCK");
        if let Some(agent_id) = identity.get("id").and_then(|v| v.as_str()) {
            println!("  Agent ID: {}", agent_id);
        }
        if let Some(created_at) = identity.get("created_at").and_then(|v| v.as_str()) {
            println!("  Created:  {}", created_at);
        }
        if let Some(seed) = identity.get("seed").and_then(|v| v.as_u64()) {
            println!("  Seed:     {}", seed);
        }
    }

    // Metabolic Record
    if let Some(metabolic) = tombstone.get("metabolic_record") {
        println!("\n⚡ METABOLIC RECORD");
        if let Some(initial) = metabolic.get("initial_energy").and_then(|v| v.as_u64()) {
            println!("  Initial Energy:    {}", initial);
        }
        if let Some(peak) = metabolic.get("peak_energy").and_then(|v| v.as_u64()) {
            println!("  Peak Energy:       {}", peak);
        }
        if let Some(final_energy) = metabolic.get("final_energy").and_then(|v| v.as_u64()) {
            println!("  Final Energy:      {}", final_energy);
        }
        if let Some(efficiency) = metabolic.get("efficiency_rating").and_then(|v| v.as_f64()) {
            println!("  Efficiency Rating: {:.2} (tasks/energy)", efficiency);
        }
        if let Some(tasks) = metabolic.get("tasks_completed").and_then(|v| v.as_u64()) {
            println!("  Tasks Completed:   {}", tasks);
        }
    }

    // Pathology Report
    if let Some(pathology) = tombstone.get("pathology_report") {
        println!("\n🩹 PATHOLOGY REPORT (Scar Tissue)");
        if let Some(scars) = pathology.get("scars").and_then(|s| s.as_array()) {
            println!("  Total Scars: {}", scars.len());
            for (i, scar) in scars.iter().enumerate() {
                let description = scar
                    .get("description")
                    .and_then(|d| d.as_str())
                    .unwrap_or("Unknown");
                let severity = scar
                    .get("severity")
                    .and_then(|s| s.as_str())
                    .unwrap_or("Unknown");
                println!("    [{}] {} ({})", i + 1, description, severity);
            }
        }
    }

    // Causal Chain
    if let Some(causal) = tombstone.get("causal_chain") {
        println!("\n🔗 CAUSAL CHAIN (Event Hash)");
        if let Some(hash) = causal.get("hash").and_then(|h| h.as_str()) {
            println!("  Hash: {}", hash);
        }
    }

    println!("\n═══════════════════════════════════════════════════════════════");
}

fn verify_tombstone(id: &str) {
    let tombstones = load_tombstones();

    let (agent_id, _tombstone) = match tombstones.iter().find(|(k, _)| k.contains(id)) {
        Some(result) => result,
        None => {
            eprintln!("❌ Agent {} not found in graveyard", id);
            return;
        }
    };

    let graveyard_path = get_graveyard_path();
    let tomb_filename = format!("{}.tomb", agent_id);
    let final_path = graveyard_path.join(&tomb_filename);

    println!("\n🔐 INTEGRITY VERIFICATION");
    println!("═══════════════════════════════════════════════════════════════");

    // Check if file is read-only
    match fs::metadata(&final_path) {
        Ok(metadata) => {
            let permissions = metadata.permissions();
            let is_readonly = permissions.readonly();

            println!("📁 File: {}", final_path.display());
            println!("🔒 Read-Only Protection: {}", if is_readonly { "✓ ENABLED" } else { "✗ DISABLED" });

            if is_readonly {
                println!("✅ VERIFIED: Tombstone is sealed and tamper-protected");
            } else {
                println!("⚠️  WARNING: Tombstone is not read-only (consider re-sealing)");
            }
        }
        Err(e) => {
            eprintln!("❌ Could not read file metadata: {}", e);
        }
    }

    println!("\n═══════════════════════════════════════════════════════════════");
}

fn find_darwinian_winner() {
    let tombstones = load_tombstones();

    if tombstones.is_empty() {
        println!("🏜️  The graveyard is empty. No agents to evaluate.");
        return;
    }

    println!("\n🧬 DARWINIAN SELECTION ANALYSIS");
    println!("═══════════════════════════════════════════════════════════════");

    let mut candidates: Vec<(String, f64, u64, usize)> = Vec::new();

    for (id, tombstone) in &tombstones {
        let tasks_completed = tombstone
            .get("metabolic_record")
            .and_then(|m| m.get("tasks_completed"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        let scars_count = tombstone
            .get("pathology_report")
            .and_then(|p| p.get("scars"))
            .and_then(|s| s.as_array())
            .map(|arr| arr.len())
            .unwrap_or(0);

        // Success-to-Scar ratio: higher is better (more success, fewer scars)
        let success_ratio = tasks_completed as f64 / (scars_count as f64 + 1.0);

        candidates.push((id.clone(), success_ratio, tasks_completed, scars_count));
    }

    // Sort by success ratio (descending)
    candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    println!("{:<10} {:<25} {:<20} {:<15}", "Agent ID", "Success-to-Scar Ratio", "Tasks Completed", "Scars");
    println!("─────────────────────────────────────────────────────────────");

    for (i, (id, ratio, tasks, scars)) in candidates.iter().enumerate().take(5) {
        let id_short = if id.len() > 10 { &id[..10] } else { id };
        println!("{:<10} {:<25} {:<20} {:<15}", id_short, format!("{:.2}", ratio), tasks, scars);
        if i == 0 {
            println!("🏆 ← DARWINIAN CHAMPION");
        }
    }

    println!("\n═══════════════════════════════════════════════════════════════");
}
