# Archaeologist Scenario: Run multi_agent_competition 5 times to build graveyard data

Write-Host "🏛️ LINEAGE GRAVEYARD: ARCHAEOLOGIST SCENARIO" -ForegroundColor Cyan
Write-Host "===========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "This script runs the multi-agent competition 5 times to populate"
Write-Host "the graveyard with ~50 dead agents, simulating evolutionary pressure"
Write-Host "over multiple generations."
Write-Host ""

# Create graveyard if it doesn't exist
if (-not (Test-Path ".\.lineage\graveyard")) {
    New-Item -ItemType Directory -Path ".\.lineage\graveyard" -Force | Out-Null
}

for ($i = 1; $i -le 5; $i++) {
    Write-Host "📍 Generation $i / 5" -ForegroundColor Yellow
    Write-Host "Running multi_agent_competition..."
    
    cargo run --release --example multi_agent_competition 2>&1 | Select-Object -Last 40
    
    Write-Host ""
    $tomb_count = @(Get-ChildItem ".\.lineage\graveyard\*.tomb" -ErrorAction SilentlyContinue).Count
    Write-Host "Agents buried in graveyard: $tomb_count" -ForegroundColor Green
    Write-Host ""
}

Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Magenta
Write-Host "✅ ARCHAEOLOGIST ANALYSIS" -ForegroundColor Magenta
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Magenta
Write-Host ""
Write-Host "Running graveyard inspector..." -ForegroundColor Yellow
Write-Host ""
cargo run --example graveyard_inspector -- --summarize
Write-Host ""
Write-Host "Finding the Darwinian Champion across all generations..." -ForegroundColor Yellow
Write-Host ""
cargo run --example graveyard_inspector -- --darwinian
Write-Host ""
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Magenta
Write-Host "Complete! The eternal archive now contains a historical record" -ForegroundColor Green
Write-Host "of evolutionary pressure and natural selection." -ForegroundColor Green
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Magenta
