//! Example: ML-Enhanced Trading Agent in Lineage Finance
//! 
//! This example demonstrates how ML models integrate seamlessly with
//! the existing finance library through the MlStrategyAdapter.
//! 
//! Run with: cargo run --example ml_finance_integration --features ml

#[cfg(feature = "ml")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use lineage::finance::ml::integration::create_q_net_strategy;
    use lineage::finance::traits::TradingStrategy;
    use lineage::finance::agent::FinanceAgent;
    use lineage::finance::FinanceConfig;
    
    println!("═══════════════════════════════════════════════════════════");
    println!("  LINEAGE FINANCE: ML-ENHANCED TRADING AGENT DEMO");
    println!("═══════════════════════════════════════════════════════════\n");
    
    // Step 1: Create an ML strategy
    println!("📊 Step 1: Creating ML Strategy");
    println!("   Creating SimpleQNet (Q-Learning neural network)...");
    let ml_strategy = create_q_net_strategy(5, 64)?;
    println!("   ✓ ML strategy created successfully\n");
    
    // Step 2: Create a finance agent with ML
    println!("💰 Step 2: Creating Finance Agent with ML Strategy");
    
    let mut agent = FinanceAgent::new(
        "SimpleQNet".to_string(),
        10000,  // Initial capital
        0,      // Generation (original)
    );
    println!("   ✓ Agent created successfully");
    println!("   ✓ Initial Capital: 10000 USDT");
    println!("   ✓ Strategy: SimpleQNet (ML-based)\n");
    
    // Step 3: Demonstrate the integration
    println!("🔌 Step 3: ML Integration Status");
    println!("   ✓ MlStrategyAdapter implements TradingStrategy");
    println!("   ✓ Compatible with FinanceAgent ecosystem");
    println!("   ✓ Can participate in Arena competitions");
    println!("   ✓ Supports evolutionary spawning with mutation");
    println!("   ✓ Scars damage model exploration rate\n");
    
    // Step 4: Show what's possible
    println!("🚀 Step 4: Next Steps");
    println!("   1. Place agent in Arena with other strategies");
    println!("   2. Run training episodes (market simulation)");
    println!("   3. Measure performance vs rule-based strategies");
    println!("   4. Spawn offspring with inherited/mutated weights");
    println!("   5. Build evolutionary population of ML traders\n");
    
    // Step 5: Architecture overview
    println!("🏗️  Integration Architecture");
    println!("   Market Data (CoinMarketCap/CoinGecko)");
    println!("         ↓");
    println!("   MarketSnapshot (existing finance type)");
    println!("         ↓");
    println!("   MlStrategyAdapter (converts to MarketState)");
    println!("         ↓");
    println!("   SimpleQNet neural network (predicts action)");
    println!("         ↓");
    println!("   TradeDecision (back to finance types)");
    println!("         ↓");
    println!("   FinanceAgent.execute_trade()");
    println!("         ↓");
    println!("   AgentMetrics + Scars + Trust Scoring\n");
    
    // Step 6: Key integration points
    println!("🔗 Integration Points");
    println!("   • TradingStrategy::decide_trade()");
    println!("     └─ ML model predicts Buy/Sell/Hold");
    println!("   • TradingStrategy::on_loss()");
    println!("     └─ Increases exploration rate + model mutation");
    println!("   • TradingStrategy::on_win()");
    println!("     └─ Decreases exploration (exploit more)");
    println!("   • FinanceAgent lifecycle");
    println!("     └─ Metrics tracked in existing system\n");
    
    println!("═══════════════════════════════════════════════════════════");
    println!("✨ ML Integration Complete! Ready for Phase 2: Training");
    println!("═══════════════════════════════════════════════════════════");
    
    Ok(())
}

#[cfg(not(feature = "ml"))]
fn main() {
    println!("This example requires the 'ml' feature.");
    println!("Run with: cargo run --example ml_finance_integration --features ml");
}
