# Lineage Finance - Solana Integration

## Overview

The Lineage Finance Solana program serves as the **on-chain settlement layer** for the Lineage trading agent system. It bridges the off-chain ML-driven agents (in `/src/finance`) with Solana's blockchain, providing:

- **Cryptographic finality** for all trades
- **Real-time price oracles** via Pyth
- **Immutable trade history** on-chain
- **SPL token staking** for agent resurrection

## Architecture

```
Off-Chain (Rust)           →    On-Chain (Solana)
finance::FinanceAgent           Agent Account (PDA)
finance::Trade                  TradeExecuted Event
finance::spawning               SpawnOffspring Instruction
finance::scars                  Scar Logic (10+ = death)
```

### Integration with `/src/finance`

This program **does not replace** the existing finance library. Instead:

1. **Agent Creation**: Mirrors `finance::FinanceAgent` structure to Solana
2. **Trade Execution**: Uses Pyth oracle to feed `trade.rs` execution logic
3. **Evolutionary Spawning**: Calls `spawning.rs` mutation on model weights
4. **Scar System**: Implements `scars.rs` death logic (10+ scars = permanent death)
5. **Events**: Broadcasts `TradeExecuted` for dashboard real-time monitoring

## Instructions

### 1. `create_agent`

Creates a new trading agent account on-chain.

```bash
lineage-cli create-agent \
  --name "⚡ Momentum" \
  --initial-capital 50000000 \
  --model-weights <serialized_weights>
```

**Maps to**: `finance::FinanceAgent::new()`

**Output**: PDA account with seed `['agent', authority]`

---

### 2. `execute_trade`

Executes a trade using Pyth BTC-USD price oracle.

```bash
lineage-cli execute-trade \
  --position-type 0 \
  --position-size 1000000 \
  --expected-slippage-bps 25
```

**Parameters**:
- `position_type`: 0 = long, 1 = short
- `position_size`: In lamports
- `expected_slippage_bps`: 0-10000 (basis points)

**Logic**:
1. Load Pyth BTC-USD price
2. Simulate trade outcome (60% win probability)
3. Apply P&L to capital
4. Detect loss > 5% → apply scar
5. Check if scars ≥ 10 → mark agent dead
6. Emit `TradeExecuted` event

**Error Codes**:
- `6010`: Insufficient capital
- `6013`: Pyth price unavailable
- `6015`: Trade cooldown (5 sec minimum)

---

### 3. `spawn_offspring`

Create offspring with genetically mutated weights.

```bash
lineage-cli spawn-offspring \
  --parent <parent_pubkey> \
  --offspring-name "Momentum Gen2" \
  --mutation-magnitude 1000
```

**Requirements**:
- Parent must be alive
- Parent must have ≥ 10 trades
- `mutation_magnitude` ≤ 5000 bps

**Inheritance**:
- Capital: 50% of parent
- Generation: parent.generation + 1
- Model weights: mutated copies

**Maps to**: `finance::spawning::spawn_offspring()`

---

### 4. `resurrect`

Bring a dead agent back to life via SPL token stake.

```bash
lineage-cli resurrect \
  --agent <agent_pubkey> \
  --new-capital 50000000 \
  --tokens 10000000000
```

**Cost**: `100 tokens per scar` (non-refundable)

**Example**: 5 scars → 500 tokens required

**Reset Behavior**:
- Capital: Set to `new_capital`
- Trades: 0
- Scars: 0
- Generation: +1
- Is Alive: true

**Maps to**: `finance::Agent::resurrect()`

---

## Data Structures

### Agent Account

```rust
pub struct Agent {
    pub authority: Pubkey,              // Owner
    pub name: String,                   // "⚡ Momentum"
    pub capital: u64,                   // In lamports
    pub model_weights: Vec<u8>,         // Serialized ML model
    pub total_trades: u64,              // Never reset
    pub winning_trades: u64,            // For win rate calc
    pub losing_trades: u64,             // For win rate calc
    pub scars: u32,                     // Death counter
    pub current_loss_streak: u32,       // Consecutive losses
    pub creation_time: i64,             // Timestamp
    pub last_trade_time: i64,           // For cooldown
    pub is_alive: bool,                 // Live/dead status
    pub generation: u64,                // 1=original, 2+=offspring
    pub parent_id: Option<Pubkey>,      // Parent agent
}
```

---

## Events (Real-Time Monitoring)

### TradeExecuted

Emitted on every completed trade. Subscribe for dashboard updates.

```json
{
  "agent": "LineageNqytQrZJgFYYJrmKBrGfVJKVZJnHwkzZCRwJqz",
  "positionType": 0,
  "positionSize": 1000000,
  "pnl": 50000,
  "isWin": true,
  "btcPrice": 78670,
  "scarsApplied": false
}
```

### OffspringSpawned

Emitted when new agent is created via mutation.

```json
{
  "parent": "...",
  "offspring": "...",
  "generation": 2
}
```

### AgentResurrected

Emitted when dead agent is revived.

```json
{
  "agent": "...",
  "cost": 500000000,
  "generation": 2
}
```

---

## Integration with Dashboard

The WebSocket server (`examples/ws_broadcast_v2.rs`) subscribes to these events:

```rust
// In production websocket listener:
match event {
    TradeExecuted { agent, pnl, is_win, btc_price, ... } => {
        broadcast_to_dashboard(DashboardMessage {
            type: "trade",
            agent,
            pnl,
            is_win,
            price: btc_price,
        });
    }
    OffspringSpawned { parent, offspring, generation } => {
        broadcast_to_dashboard(DashboardMessage {
            type: "spawn",
            parent,
            offspring,
            generation,
        });
    }
}
```

---

## Pyth Oracle Integration

### Mainnet Price Feed
- **BTC-USD**: `8SXvChNYFhRCZRvSAClyS6VG6DpJfzSUSn9c6hksQKvV`
- **ETH-USD**: `JFC7V5A12DXJ6FSQ9L7YTWVE6YB7HBJJQ73SWDJCEYE`

### Price Loading
```rust
let price_feed = load_price_feed_from_account(&ctx.accounts.pyth_account)?;
let current_price = price_feed.get_current_price().ok_or(ErrorCode::PriceUnavailable)?;
```

---

## Death Mechanics

An agent dies **permanently and irreversibly** when:

1. **Scars ≥ 10**: After 10 major losses (> 5% drawdown), agent is marked dead
2. **Irreversible**: Cannot be undone - only resurrection can revive
3. **Events**: Death triggers `TradeExecuted` event with `agent_alive: false`

Example sequence:
```
Trade 1: -8% loss → scar #1
Trade 2: -7% loss → scar #2
...
Trade 10: -6% loss → scar #10 → AGENT DEAD
```

---

## Resurrection Mechanics

Cost: **100 tokens per scar accumulated**

```
Agent with 5 scars:
- Resurrection cost: 500 tokens
- New capital: Set by caller
- Scars: Reset to 0
- Generation: Incremented
- Alive: true
```

---

## Error Codes

| Code | Error | Meaning |
|------|-------|---------|
| 6000 | NameTooLong | Agent name > 50 chars |
| 6001 | InvalidCapital | Capital ≤ 0 |
| 6002 | ModelWeightsTooLarge | Weights > 1KB |
| 6003 | AgentDead | Cannot trade when dead |
| 6010 | InsufficientCapital | Position > available capital |
| 6011 | InvalidPositionSize | Position size ≤ 0 |
| 6012 | InvalidPositionType | Not 0 or 1 |
| 6013 | PriceUnavailable | Pyth feed down |
| 6014 | InvalidPrice | Price ≤ 0 |
| 6015 | TradeCooldown | < 5 seconds since last trade |
| 6020 | CannotSpawnFromDeadAgent | Parent is dead |
| 6021 | InsufficientTradeHistory | Parent < 10 trades |
| 6022 | InvalidMutationRate | Mutation > 5000 bps |
| 6023 | AgentAlreadyAlive | Cannot resurrect alive agent |
| 6024 | InsufficientTokensForResurrection | Not enough tokens for cost |

---

## Building & Testing

```bash
# Build the program
anchor build

# Build IDL
anchor build --idl ./

# Run tests
anchor test

# Deploy to Devnet
anchor deploy --provider.cluster devnet
```

---

## MVP: From Simulation to Blockchain

### Phase 3 (Current): Simulated Trading on Dashboard
- ✅ WebSocket server with 5 cryptos
- ✅ Real-time agent dashboard
- ✅ Simulated Pyth prices
- ✅ Real CoinMarketCap/Coindesk API fallback

### Phase 4 (Solana MVP): On-Chain Settlement
- ⏳ Deploy `lineage_finance` program to Solana
- ⏳ Connect dashboard to on-chain events
- ⏳ Real Pyth oracle prices
- ⏳ SPL token staking for resurrection

### Phase 5 (Production): Full Integration
- ⏳ Off-chain agents submit trades to program
- ⏳ On-chain settlement with cryptographic finality
- ⏳ Evolutionary loop: spawn → trade → death/resurrection
- ⏳ Cross-chain composability (bridges)

---

## References

- **Finance Library**: `/src/finance/`
  - `agent.rs` - Agent lifecycle & metrics
  - `trade.rs` - Trade execution & irreversibility
  - `scars.rs` - Damage accumulation & death
  - `spawning.rs` - Genetic mutation & offspring
  - `ml/` - Neural network training

- **Anchor Docs**: https://docs.rs/anchor-lang/0.29/
- **Pyth SDK**: https://github.com/pyth-network/pyth-sdk-rs
- **Solana Docs**: https://docs.solana.com/

---

## Next Steps

1. Build and test the program locally
2. Deploy to Solana Devnet
3. Update WebSocket server to subscribe to on-chain events
4. Connect dashboard to real Solana events
5. Integrate off-chain agents with on-chain program

This completes the **MVP milestone**: agents trading with real prices, backed by blockchain finality.
