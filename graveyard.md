🏛️ Project: The Eternal Archive (Graveyard)
1. The Functional Requirements
To "wow" everyone, the Graveyard must achieve three things:

Cryptographic Sealing: Every tombstone must contain a merkle_root or a causal_hash that links every event in the agent's life. If one byte of the history is changed, the "Grave" becomes invalid.

Post-Mortem Queries: We should be able to ask the Graveyard: "Who was the longest-lived agent?" or "What was the most common cause of death in 2026?"

The "Lazarus" Prevention: The system must check the Graveyard upon every Agent::new(). If an ID is found in the Graveyard, the program must panic—because a dead identity cannot be reborn.

2. The Implementation Prompt
"Implement a persistent Graveyard module for the Lineage system with the following specifications:"

A. The Tombstone Schema
Don't just save the totals; save the Final State Snapshot.

Identity Block: ID, Hash, and "Creation Seed".

Metabolic Record: Final Energy, Peak Energy, and "Efficiency Rating" (Tasks/Energy).

Pathology Report: A full list of Scars with timestamps and "Incurred From" metadata.

Causal Chain: A cryptographic hash representing the entire sequence of events.

B. The Persistence Engine
Use a sub-directory .lineage/graveyard/.

Files must be saved as ReadOnly at the OS level after writing to prevent accidental deletion or editing.

Implement a Registry that maintains an in-memory index of the dead for lightning-fast "Lazarus" checks.

C. The Forensic CLI (The "Wow" Factor)
Create an example examples/graveyard_inspector.rs that provides:

--summarize: A table of all dead agents, their lifespan, and their "Legacy Score."

--autopsy <ID>: A detailed, color-coded timeline of an agent’s life, showing the exact moment "The Golden Years" ended and "The Decay" began.

--verify <ID>: Re-calculates all hashes in the causal chain to prove the record hasn't been tampered with since death.

3. The "Wow" Examples to Build
To demonstrate the power of the Graveyard, you should code these two specific scenarios:

Scenario A: The "Ghost in the Machine"
Run an agent until it dies. Close the program. Try to run a new agent with the same ID.

Result: The system detects the tombstone in the .lineage/graveyard/ and refuses to initialize, printing: ONTOLOGICAL ERROR: Identity [ID] is already sealed in the Eternal Archive.

Scenario B: The "Archaeologist"
Run the multi_agent_competition example 5 times. This will populate the graveyard with ~50 dead agents.

Result: Use the graveyard_inspector to find the "Darwinian Winner"—the agent across all 5 historical runs that had the best Success-to-Scar ratio.

4. Technical Constraints to Achieve
No Overwrites: The bury() function must return an error if a file with that name exists.

Atomic Writes: Use a "temporary file + rename" pattern so that if the power cuts out during burial, the grave isn't corrupted.

Zero-Cost Inspection: The Graveyard should be able to list thousands of dead agents without loading their full event histories into RAM.