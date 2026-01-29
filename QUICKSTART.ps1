# Quick-start guide for Lineage demonstrations (PowerShell)

Write-Host ""
Write-Host "╔════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║          LINEAGE SYSTEM DEMONSTRATION QUICK-START          ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

Write-Host "Three executable demonstrations are ready:" -ForegroundColor Yellow
Write-Host ""

Write-Host "1. MAIN BINARY (Recommended starting point)" -ForegroundColor Green
Write-Host "   └─ cargo run --bin demo" -ForegroundColor White
Write-Host "      Shows: Complete lifecycle with 3 phases" -ForegroundColor Gray
Write-Host "      Time:  ~10 seconds" -ForegroundColor Gray
Write-Host ""

Write-Host "2. LIFECYCLE EXAMPLE (Detailed walkthrough)" -ForegroundColor Green
Write-Host "   └─ cargo run --example lifecycle_demo" -ForegroundColor White
Write-Host "      Shows: Birth → Health → Strain → Analysis" -ForegroundColor Gray
Write-Host "      Time:  ~8 seconds with pauses" -ForegroundColor Gray
Write-Host ""

Write-Host "3. MORTALITY EXAMPLE (Dramatic conclusion)" -ForegroundColor Green
Write-Host "   └─ cargo run --example mortality" -ForegroundColor White
Write-Host "      Shows: The inexorable path to death" -ForegroundColor Gray
Write-Host "      Time:  ~6 seconds" -ForegroundColor Gray
Write-Host ""

Write-Host "═══════════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

Write-Host "What to look for in the output:" -ForegroundColor Yellow
Write-Host ""
Write-Host "  Energy Bar:   Shows remaining energy (decreases only)" -ForegroundColor Gray
Write-Host "  Damage Bar:   Shows accumulated scars (increases only)" -ForegroundColor Gray
Write-Host "  Capacity:     Functional capacity (100 minus damage)" -ForegroundColor Gray
Write-Host "  Status:       🟢 ALIVE or ⚫ DEAD" -ForegroundColor Gray
Write-Host ""

Write-Host "Key insight:" -ForegroundColor Yellow
Write-Host "  - Watch as damage increases task costs" -ForegroundColor Gray
Write-Host "  - Early damage compounds across entire lifetime" -ForegroundColor Gray
Write-Host "  - Energy exhaustion is inevitable" -ForegroundColor Gray
Write-Host ""

Write-Host "═══════════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

Write-Host "To run the demonstrations:" -ForegroundColor Yellow
Write-Host ""
Write-Host "   cd D:\Projects\Lineage" -ForegroundColor White
Write-Host ""
Write-Host "Then pick one:" -ForegroundColor Yellow
Write-Host "   cargo run --bin demo" -ForegroundColor White
Write-Host "   cargo run --example lifecycle_demo" -ForegroundColor White
Write-Host "   cargo run --example mortality" -ForegroundColor White
Write-Host ""

Write-Host "Each time you run one, you'll see a slightly different outcome" -ForegroundColor Gray
Write-Host "(different failure types, different damage amounts), but the" -ForegroundColor Gray
Write-Host "fundamental truth remains: entropy wins, energy depletes, death" -ForegroundColor Gray
Write-Host "approaches." -ForegroundColor Gray
Write-Host ""

Write-Host "═══════════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""
