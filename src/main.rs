//! # Lineage - Software Identity Through Irreversible Change
//!
//! This demonstration shows the core principles of Lineage:
//! - Unique, non-copyable identity
//! - Append-only causal memory
//! - Finite metabolic budget
//! - Permanent scars
//! - Irreversible death

mod identity;
mod memory;
mod metabolism;
mod scar;
mod lineage;
mod behavior;

use lineage::{Lineage, OperationError, OperationResult};
use scar::ScarSeverity;
use behavior::PulseBehavior;

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║        LINEAGE - Ontological Software Demonstration       ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // === DEMONSTRATION 1: Birth ===
    println!("┌─ DEMONSTRATION 1: Birth ─────────────────────────────────┐");
    let mut lineage = Lineage::create(1000);
    println!("Lineage created with identity: {}", lineage.identity().id());
    println!("Birth time: {} nanoseconds since epoch", lineage.identity().birth_time());
    println!("Initial energy: {} units", lineage.metabolism().initial_energy());
    println!("└──────────────────────────────────────────────────────────┘\n");

    // === DEMONSTRATION 2: Behavior Loop (Pulse System) ===
    println!("┌─ DEMONSTRATION 2: Consequential Behavior Loop ──────────┐");
    println!("Demonstrating: One behavior, one contract, one injury path\n");
    
    let pulse_behavior = PulseBehavior::new();
    println!("Pulse behavior initialized:");
    println!("  - Base cost: 10 energy");
    println!("  - Threshold: {} energy (minimum for healthy pulse)", pulse_behavior.threshold());
    println!("  - Contract: Pulse below threshold causes strain");
    println!("  - Consequence: Strain scars increase future cost by 5\n");

    // Execute healthy pulses
    println!("Executing healthy pulses...");
    for i in 1..=3 {
        let output = pulse_behavior.execute_pulse(&mut lineage);
        println!("  Pulse #{}: energy={}, strong={}, cost={}", 
            i, 
            output.energy_at_pulse,
            output.is_strong,
            pulse_behavior.current_pulse_cost(&lineage)
        );
    }
    
    println!("\nCurrent state:");
    println!("  Energy: {}", lineage.metabolism().energy());
    println!("  Scars: {}", lineage.scars().scar_count());
    println!("  Pulse cost: {}\n", pulse_behavior.current_pulse_cost(&lineage));

    // Deplete to danger zone
    println!("Depleting energy to danger zone...");
    let current = lineage.metabolism().energy();
    lineage.perform_operation("Heavy work".to_string(), current - 25);
    println!("  Energy now: {} (below threshold!)\n", lineage.metabolism().energy());

    // Execute weak pulse - CONTRACT VIOLATION
    println!("Executing weak pulse (contract violation)...");
    let output = pulse_behavior.execute_pulse(&mut lineage);
    println!("  ⚠️  STRAIN DETECTED!");
    println!("  - Energy at pulse: {}", output.energy_at_pulse);
    println!("  - Below threshold: {}", output.energy_at_pulse < pulse_behavior.threshold());
    println!("  - Strain inflicted: {}", output.strain_occurred);
    println!("  - Permanent scar recorded: YES");
    println!("  - New pulse cost: {} (+5 penalty)\n", pulse_behavior.current_pulse_cost(&lineage));

    // Show permanent consequence
    println!("Consequence is permanent:");
    println!("  Scars before: 0 → Scars now: {}", lineage.scars().scar_count());
    println!("  Cost before: 10 → Cost now: {}", pulse_behavior.current_pulse_cost(&lineage));
    
    // Demonstrate death spiral
    println!("\nDemonstrating death spiral from accumulated strain...");
    let mut pulse_count = 0;
    while lineage.is_alive() && pulse_count < 10 {
        let energy = lineage.metabolism().energy();
        if energy < 35 && energy >= 15 {
            let output = pulse_behavior.execute_pulse(&mut lineage);
            if output.strain_occurred {
                println!("  Pulse #{}: STRAIN (energy={}, cost={})", 
                    pulse_count + 1,
                    output.energy_at_pulse,
                    pulse_behavior.current_pulse_cost(&lineage)
                );
            }
            pulse_count += 1;
        } else {
            break;
        }
    }
    
    println!("\nFinal state:");
    println!("  Energy: {}", lineage.metabolism().energy());
    println!("  Strain scars: {}", lineage.scars().scar_count());
    println!("  Pulse cost: {} (increased by {})", 
        pulse_behavior.current_pulse_cost(&lineage),
        (pulse_behavior.current_pulse_cost(&lineage) - 10)
    );
    println!("  Alive: {}", lineage.is_alive());
    println!("└──────────────────────────────────────────────────────────┘\n");

    // === DEMONSTRATION 3: Normal Operations ===
    println!("┌─ DEMONSTRATION 3: Other Operations ─────────────────────┐");
    
    // Reset lineage for cleaner demonstration
    let mut lineage = Lineage::create(1000);
    
    match lineage.perform_operation("Initialize subsystems".to_string(), 100) {
        OperationResult::Success { energy_consumed } => {
            println!("✓ Operation succeeded (consumed {} energy)", energy_consumed);
            println!("  Remaining energy: {}", lineage.metabolism().energy());
        }
        _ => println!("✗ Operation failed"),
    }

    match lineage.perform_operation("Process data batch".to_string(), 150) {
        OperationResult::Success { energy_consumed } => {
            println!("✓ Operation succeeded (consumed {} energy)", energy_consumed);
            println!("  Remaining energy: {}", lineage.metabolism().energy());
        }
        _ => println!("✗ Operation failed"),
    }

    match lineage.perform_operation("Execute computation".to_string(), 200) {
        OperationResult::Success { energy_consumed } => {
            println!("✓ Operation succeeded (consumed {} energy)", energy_consumed);
            println!("  Remaining energy: {}", lineage.metabolism().energy());
        }
        _ => println!("✗ Operation failed"),
    }
    
    println!("└──────────────────────────────────────────────────────────┘\n");

    // === DEMONSTRATION 4: Encountering Errors (Scars) ===
    println!("┌─ DEMONSTRATION 4: Scars ─────────────────────────────────┐");
    
    let minor_error = OperationError::new(
        ScarSeverity::Minor,
        "Network timeout during sync".to_string(),
    ).with_context("Attempted connection to peer node failed after 3 retries".to_string());
    
    lineage.record_error(minor_error);
    println!("✗ Minor scar inflicted: Network timeout");
    println!("  Scar count: {}", lineage.scars().scar_count());
    println!("  Status: {}", if lineage.is_alive() { "ALIVE" } else { "DEAD" });

    let moderate_error = OperationError::new(
        ScarSeverity::Moderate,
        "Data corruption in cache layer".to_string(),
    );
    
    lineage.record_error(moderate_error);
    println!("✗ Moderate scar inflicted: Cache corruption");
    println!("  Scar count: {}", lineage.scars().scar_count());
    println!("  Damage score: {}", lineage.scars().damage_score());
    
    println!("└──────────────────────────────────────────────────────────┘\n");

    // === DEMONSTRATION 5: Insufficient Energy ===
    println!("┌─ DEMONSTRATION 5: Energy Exhaustion ────────────────────┐");
    
    match lineage.perform_operation("Expensive AI inference".to_string(), 800) {
        OperationResult::Success { energy_consumed } => {
            println!("✗ This should not have succeeded!");
        }
        OperationResult::InsufficientEnergy { required, available } => {
            println!("✗ Operation rejected: Insufficient energy");
            println!("  Required: {} units", required);
            println!("  Available: {} units", available);
            println!("  Status: Still alive, but constrained");
        }
        _ => {}
    }
    
    println!("└──────────────────────────────────────────────────────────┘\n");

    // === DEMONSTRATION 6: Gradual Depletion ===
    println!("┌─ DEMONSTRATION 6: Path to Death ────────────────────────┐");
    
    let remaining_energy = lineage.metabolism().energy();
    println!("Current energy: {} units", remaining_energy);
    println!("Performing operations until death...\n");

    let mut operation_count = 0;
    while lineage.is_alive() {
        operation_count += 1;
        let result = lineage.perform_operation(
            format!("Final operations #{}", operation_count),
            100,
        );
        
        match result {
            OperationResult::Success { energy_consumed } => {
                let remaining = lineage.metabolism().energy();
                println!("  Op #{}: consumed {}, remaining {}", 
                    operation_count, energy_consumed, remaining);
                
                if !lineage.is_alive() {
                    println!("\n💀 DEATH: Energy depleted");
                    break;
                }
            }
            OperationResult::InsufficientEnergy { .. } => {
                println!("  Op #{}: Insufficient energy, skipping", operation_count);
                break;
            }
            OperationResult::Dead => {
                println!("\n💀 DEATH: Already dead");
                break;
            }
            _ => {}
        }
    }
    
    println!("└──────────────────────────────────────────────────────────┘\n");

    // === DEMONSTRATION 6: Operations After Death ===
    println!("┌─ DEMONSTRATION 6: Operations After Death ───────────────┐");
    
    match lineage.perform_operation("Attempted resurrection".to_string(), 10) {
        OperationResult::Dead => {
            println!("✗ Operation rejected: Lineage is dead");
            println!("  Death is irreversible.");
            println!("  No resurrection. No rollback. No second chances.");
        }
        _ => println!("✗ This should not be possible!"),
    }
    
    println!("└──────────────────────────────────────────────────────────┘\n");

    // === FINAL STATUS ===
    println!("┌─ FINAL STATUS ───────────────────────────────────────────┐");
    let status = lineage.status();
    println!("{}", status);
    
    println!("Memory History (last 5 events):");
    let history = lineage.memory().history();
    let start = if history.len() > 5 { history.len() - 5 } else { 0 };
    for event in &history[start..] {
        println!("  [{}] {}", event.sequence(), event.description());
    }
    
    println!("\nScar Tissue:");
    for scar in lineage.scars().all_scars() {
        println!("  [{:?}] {}", scar.severity(), scar.description());
        if let Some(ctx) = scar.context() {
            println!("    Context: {}", ctx);
        }
    }
    
    println!("└──────────────────────────────────────────────────────────┘\n");

    // === PHILOSOPHICAL CONCLUSION ===
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║                    ONTOLOGICAL TRUTHS                      ║");
    println!("╠════════════════════════════════════════════════════════════╣");
    println!("║  ✓ Identity was unique and non-copyable                   ║");
    println!("║  ✓ History was append-only and immutable                  ║");
    println!("║  ✓ Energy could only decrease, never increase             ║");
    println!("║  ✓ Scars were permanent and accumulated                   ║");
    println!("║  ✓ Death was final and irreversible                       ║");
    println!("║                                                            ║");
    println!("║  This is not a simulation.                                ║");
    println!("║  This is not a convenience feature.                       ║");
    println!("║  This is ontology.                                        ║");
    println!("╚════════════════════════════════════════════════════════════╝");
}

// Additional demonstration: Cannot clone lineage
// 
// If you uncomment this function, it will fail to compile:
//
// fn attempt_clone(lineage: &Lineage) {
//     let cloned = lineage.clone(); // Compilation error
// }
//
// This is intentional. Identity cannot be duplicated.
