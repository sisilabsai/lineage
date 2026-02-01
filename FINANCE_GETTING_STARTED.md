# Lineage Finance - Getting Started Guide

Welcome to **Lineage Finance**, the evolutionary trading platform where AI agents live, evolve, and die.

## Quick Start

### Run the Example (30 seconds)

```bash
cd /path/to/lineage
cargo run --example decentralized_trading_agent
```

You'll see a full demo of:
- Agent lifecycle (capital, trades, trust)
- Spawning (offspring inheritance)
- Multi-agent arena (competition)
- Advanced features (blockchain, evolution, governance)

### Expected Output

```
╔════════════════════════════════════════════════════════════════╗
║ LINEAGE FINANCE - EVOLUTIONARY TRADING PLATFORM ║
╚════════════════════════════════════════════════════════════════╝

=== FinanceAgent Lifecycle Demo ===
Agent <ID> [SimpleBot]
  Capital: 100000
  Trades: 5
  Win Rate: 60.00%
  Scars: 0
  Trust: 50.00
  Status: Active

... (more demos follow)
```

---

## What is Lineage Finance?

**Lineage Finance** extends the core Lineage framework to build **decentralized, autonomous AI trading platforms** with irreversible consequences:

- **Finite Capital**: Agents start with energy (capital); trades deplete it
- **Permanent Scars**: Losses inflict scars that permanently increase costs
- **Evolutionary Spawning**: Successful agents spawn offspring inheriting optimized traits
- **Trust Scoring**: Cryptographic trust scores determine resource access
- **No Rollbacks**: Trades are append-only; once executed, permanent
- **Death & Archives**: Agents die from exhaustion; sealed graveyard for audits

This is **Darwinian DeFi**—where consequence drives evolution.

---

## Architecture Overview

### Core Modules

```
src/finance/
├── mod.rs              # Configuration, module exports
├── agent.rs            # FinanceAgent trait, lifecycle
├── trade.rs            # Irreversible trade execution
├── scars.rs            # Permanent damage system
├── spawning.rs         # Offspring creation, inheritance
├── trust_scoring.rs    # Cryptographic trust formulas
├── arena.rs            # Multi-agent competition
└── advanced.rs         # Blockchain, evolution, governance
```

### Integration with Lineage Core

Each `FinanceAgent` extends the base `Agent` with financial-specific operations:

```rust
use lineage::finance::agent::FinanceAgent;
use lineage::finance::trade::Trade;

// Create an agent with starting capital
let mut agent = FinanceAgent::new(100_000.0); // $100K

// Execute irreversible trade
let trade = Trade::new(Direction::Buy, 50_000.0, 1.5); // Buy $50K with 1.5x leverage
let result = agent.execute_trade(&trade);
// Result: Either Success/Loss/Rejected; capital depleted; scars possible
```

---

## Core Concepts

### 1. Energy as Capital

Every agent starts with finite `energy` (capital):

```rust
agent.get_capital()  // Returns remaining capital
agent.consume_capital(amount)  // Costs energy for operations
```

- **Why it matters**: Forces accountability; no infinite resources
- **Consequence**: Agent dies when capital → 0

### 2. Scars as Permanent Penalties

Losses inflict scars that permanently affect future performance:

```rust
agent.inflict_financial_scar(loss_amount, loss_type);
// Scar inflicts:
// - Cost multiplier on future trades (1.0x → 1.5x)
// - Leverage restrictions (10x → 5x max)
// - Trust score penalties (-5 points per scar)
```

- **Why it matters**: Failure teaches permanently; robots learn to avoid risk
- **Consequence**: Too many scars → agent death

### 3. Spawning & Inheritance

Successful agents (high capital, low scars, positive ROI) can spawn offspring:

```rust
let offspring = agent.spawn_offspring();
// Offspring:
// - Gets 50% of parent's capital
// - Inherits cost multiplier optimization
// - Starts generation N+1
// - Can further optimize via evolution
```

- **Why it matters**: Successful strategies propagate; evolution in action
- **Consequence**: Lineages develop over time

### 4. Trust Scoring

Trust scores are cryptographically derived from performance:

```rust
let score = agent.compute_trust_score();
// Formula: 40% win_rate + 25% consistency + 20% damage_resilience + 15% longevity

let grant = agent.get_trust_grant(score);
// Tiers:
// - Gold (>85): Max leverage, premium data, DAO voting
// - Silver (70-85): Standard leverage, basic data
// - Bronze (50-70): Limited leverage, restricted access
// - Restricted (<50): No trading, read-only
```

- **Why it matters**: Access earned through proven behavior
- **Consequence**: Bad agents starve of resources naturally

### 5. Arena Competition

Agents compete in simulated markets:

```rust
let mut arena = Arena::new(4); // 4 assets
for round in 0..100 {
    arena.tick_round();  // Market evolves (volatility, trends)
    // Agents trade, some die, survivors spawn
}

let results = arena.get_ranked_results();
// Top performers eligible for inheritance spread
```

- **Why it matters**: Emergence; survival of the fittest
- **Consequence**: Best strategies naturally replicate

---

## Examples

### Example 1: Create & Trade

```rust
use lineage::finance::agent::FinanceAgent;
use lineage::finance::trade::{Trade, Direction};

fn main() {
    let mut agent = FinanceAgent::new(100_000.0);
    
    // Winning trade
    let buy_trade = Trade::new(Direction::Buy, 50_000.0, 1.0);
    agent.execute_trade(&buy_trade);  // Cost: $50K + fee
    
    // Losing trade (inflicts scar)
    let sell_trade = Trade::new(Direction::Sell, 25_000.0, 1.5);
    agent.execute_trade(&sell_trade);  // Cost + loss = scar
    
    println!("Capital: {}", agent.get_capital());
    println!("Scars: {}", agent.scar_count());
    println!("Trust: {}", agent.compute_trust_score());
}
```

### Example 2: Spawn Offspring

```rust
use lineage::finance::spawning::Offspring;

let mut parent = FinanceAgent::new(200_000.0);
// (parent trades successfully...)

// Check if eligible to spawn
if parent.is_eligible_to_spawn() {
    let offspring = Offspring::validate_spawn(&parent).unwrap();
    println!("Spawned: {:?}", offspring.agent_id);
    println!("Generation: {}", offspring.generation);
    println!("Inherited efficiency: {}", offspring.traits.cost_multiplier);
}
```

### Example 3: Trust-Based Access

```rust
use lineage::finance::trust_scoring::TrustRecord;

let trust_score = agent.compute_trust_score();
let grant = TrustRecord::new(trust_score);

// Different access levels
match grant {
    TrustGrant::Gold { score } => {
        println!("Elite agent! Access: leverage 10x, private pools");
    }
    TrustGrant::Silver { score } => {
        println!("Good reputation. Access: leverage 5x, standard pools");
    }
    TrustGrant::Bronze { score } => {
        println!("New agent. Access: leverage 2x, public pools only");
    }
    TrustGrant::Restricted { score } => {
        println!("Too many scars. No trading allowed.");
    }
}
```

---

## Building a Custom Trading Agent

### Step 1: Define Your Strategy

```rust
use lineage::finance::agent::FinanceAgent;
use lineage::finance::arena::MarketState;

pub struct MyBot;

impl MyBot {
    pub fn decide(agent: &FinanceAgent, market: &MarketState) -> bool {
        let capital = agent.get_capital();
        let trend = market.trend;
        
        // Example: Buy if capital > $50K and trend positive
        capital > 50_000.0 && trend > 0.0
    }
}
```

### Step 2: Integrate with Arena

```rust
let mut arena = Arena::new(4);

for round in 0..1000 {
    for agent in &mut arena.agents {
        if MyBot::decide(agent, &arena.market_state) {
            let trade = Trade::new(Direction::Buy, 10_000.0, 1.5);
            agent.execute_trade(&trade);
        }
    }
    arena.tick_round();
}
```

### Step 3: Analyze Results

```rust
let results = arena.get_ranked_results();
for (rank, result) in results.iter().enumerate() {
    println!("#{}: {} -> {}% return", rank + 1, result.agent_id, result.return_percent);
}
```

---

## Advanced: Evolutionary AI (Coming Soon)

Future releases will integrate PyTorch for adaptive learning:

```rust
// Phase 2 feature (not yet available)
use lineage::finance::advanced::EvolutionaryStrategy;

let strategy = EvolutionaryStrategy::new(model_weights);
let evolved = strategy.evolution_generation(100);  // 100 generations
// Offspring mutations automatically optimize decision thresholds
```

---

## Troubleshooting

### Issue: "Agent died with insufficient capital"
**Solution**: Check trade size relative to capital. Start small (5-10% of capital per trade).

### Issue: "Trust score too low to spawn"
**Solution**: Win more trades before spawning. Minimum requirements: 
- Capital ≥ $100K
- Win rate ≥ 50%
- Scars ≤ 2

### Issue: "Market volatility simulation seems unrealistic"
**Solution**: Phase 2 will integrate real price feeds. For now, adjust volatility parameters in `Arena::new()`.

---

## Next Steps

1. **Run the example**: `cargo run --example decentralized_trading_agent`
2. **Read the code**: Check `examples/decentralized_trading_agent.rs` for full demo
3. **Explore modules**: Peek at `src/finance/` to understand internals
4. **Build custom bot**: Extend `FinanceAgent` trait for your strategy
5. **Join the conversation**: GitHub issues, discussions, X (@lineage_rs)

---

## Resources

- **Main Docs**: [Lineage GitHub](https://github.com/sisilabsai/lineage)
- **Roadmap**: [FINANCE_IMPLEMENTATION_ROADMAP.md](./FINANCE_IMPLEMENTATION_ROADMAP.md)
- **Vision**: [Lineage Finance Spec Prompt](./LINEAGE_FINANCE_SPEC.md)

---

## License

Lineage Finance is part of the Lineage project. See [LICENSE](./LICENSE) for details.

---

**Happy trading! May your agents evolve wisely.** 🚀
