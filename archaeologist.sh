#!/bin/bash
# Archaeologist Scenario: Run multi_agent_competition 5 times to build graveyard data

echo "🏛️ LINEAGE GRAVEYARD: ARCHAEOLOGIST SCENARIO"
echo "==========================================="
echo ""
echo "This script runs the multi-agent competition 5 times to populate"
echo "the graveyard with ~50 dead agents, simulating evolutionary pressure"
echo "over multiple generations."
echo ""

# Create graveyard if it doesn't exist
mkdir -p .lineage/graveyard

for i in {1..5}; do
    echo "📍 Generation $i / 5"
    echo "Running multi_agent_competition..."
    cargo run --release --example multi_agent_competition 2>&1 | tail -40
    echo ""
    echo "Agents buried in graveyard: $(ls -1 .lineage/graveyard/*.tomb 2>/dev/null | wc -l)"
    echo ""
done

echo "═══════════════════════════════════════════════════════════════"
echo "✅ ARCHAEOLOGIST ANALYSIS"
echo "═══════════════════════════════════════════════════════════════"
echo ""
echo "Running graveyard inspector..."
echo ""
cargo run --example graveyard_inspector -- --summarize
echo ""
echo "Finding the Darwinian Champion across all generations..."
echo ""
cargo run --example graveyard_inspector -- --darwinian
echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "Complete! The eternal archive now contains a historical record"
echo "of evolutionary pressure and natural selection."
echo "═══════════════════════════════════════════════════════════════"
