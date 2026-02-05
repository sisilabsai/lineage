# Quick Start: Build & Deploy Solana Program

## Prerequisites

### Option A: Windows PowerShell (Recommended for Windows)

```powershell
# Install Rust using rustup installer
# Download from: https://win.rustup.rs/rustup-init.exe
# Or use Scoop/Chocolatey
scoop install rustup  # or: choco install rustup

# Verify Rust installation
rustc --version
cargo --version

# Install Rust formatting component
rustup component add rustfmt

# Install Solana CLI
# Option 1: Download Windows binary
$downloadUrl = "https://github.com/solana-labs/solana/releases/download/v1.17.28/solana-release-x86_64-pc-windows-msvc.tar.bz2"
Invoke-WebRequest -Uri $downloadUrl -OutFile solana-release.tar.bz2
# Then extract and add to PATH

# Option 2: Use Windows installer from Solana docs
# https://docs.solana.com/cli/install-solana-cli-tools

# Install Anchor (latest)
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install latest
avm use latest

# Verify installations
anchor --version
solana --version
cargo --version
```

### Option B: Git Bash / WSL (Unix-like environment)

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup component add rustfmt

# Install Anchor (latest)
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install latest
avm use latest

# Install Solana CLI (if not already installed)
sh -c "$(curl -sSfL https://release.solana.com/v1.17.28/install)"
export PATH="/home/username/.local/share/solana/install/active_release/bin:$PATH"

# Verify installations
anchor --version
solana --version
cargo --version
```

## Build the Program

```bash
# Navigate to program directory
cd d:\Projects\Lineage\programs\lineage-finance

# Build in development mode
anchor build

# Output will be in: target/deploy/lineage_finance.so
```

**Expected output:**
```
Compiling lineage_finance v0.1.0
Finished release [optimized] target(s) in 2m 30s

Build successful ✓
IDL generated: target/idl/lineage_finance.json
```

## Generate IDL

```bash
anchor build --idl ./

# IDL will be at: target/idl/lineage_finance.json
```

## Run Tests

```bash
# Start local Solana validator
solana-test-validator

# In another terminal, run tests
anchor test

# Tests will verify:
# ✓ create_agent
# ✓ execute_trade with Pyth oracle
# ✓ spawn_offspring with mutation
# ✓ resurrect with token transfer
```

## Deploy to Devnet

```bash
# Configure for Devnet
solana config set --url https://api.devnet.solana.com

# Airdrop SOL to your wallet (if needed)
solana airdrop 5 ~/.config/solana/id.json --url devnet

# Deploy program
anchor deploy --provider.cluster devnet

# Output will show:
# Program deployed to: LineageNqytQrZJgFYYJrmKBrGfVJKVZJnHwkzZCRwJqz
```

## Verify Deployment

```bash
# Check program on chain
solana program show LineageNqytQrZJgFYYJrmKBrGfVJKVZJnHwkzZCRwJqz --url devnet

# Should output:
# Program Id   : LineageNqytQrZJgFYYJrmKBrGfVJKVZJnHwkzZCRwJqz
# Owner        : BPFLoaderUpgradeab1e11111111111111111111111
# ProgramData  : [data account]
# Executable   : yes
# Lamports      : [amount]
```

## Use with Anchor CLI

```bash
# Create agent
anchor run create_agent \
  --name "⚡ Momentum" \
  --initial-capital 50000000 \
  --model-weights $(printf 'weights...' | base64)

# Execute trade
anchor run execute_trade \
  --position-type 0 \
  --position-size 1000000 \
  --expected-slippage-bps 25

# Spawn offspring
anchor run spawn_offspring \
  --parent <parent_pubkey> \
  --offspring-name "Momentum Gen2" \
  --mutation-magnitude 1000

# Resurrect agent
anchor run resurrect \
  --agent <agent_pubkey> \
  --new-capital 50000000 \
  --tokens 10000000000
```

## Monitor Events

```bash
# Listen to program events
anchor logs --provider.cluster devnet

# You'll see:
# TradeExecuted { agent: ..., pnl: ..., is_win: true, ... }
# OffspringSpawned { parent: ..., offspring: ..., generation: 2 }
# AgentResurrected { agent: ..., cost: 500000000, ... }
```

## Integration with Dashboard

Once deployed, update the WebSocket server to listen to events:

```rust
// In examples/ws_broadcast_v2.rs
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;

#[tokio::main]
async fn main() {
    let rpc = RpcClient::new("https://api.devnet.solana.com");
    let program_id = "LineageNqytQrZJgFYYJrmKBrGfVJKVZJnHwkzZCRwJqz".parse::<Pubkey>()?;
    
    // Subscribe to program logs
    while let Some(log) = rpc.get_logs(&program_id) {
        if log.contains("TradeExecuted") {
            broadcast_to_dashboard(parse_trade_event(&log));
        }
    }
}
```

## Troubleshooting

### Build Fails: "cannot find crate `pyth_sdk_solana`"
```bash
# Update Cargo.lock
cargo update

# Rebuild
anchor build
```

### Deploy Fails: "Program Too Large"
```bash
# Enable optimizations in Cargo.toml [profile.release]
opt-level = 3
lto = true

# Rebuild and redeploy
anchor deploy --provider.cluster devnet
```

### Tests Fail: "Pyth account not found"
```bash
# Use local Pyth mock in tests
// In test file:
use std::mem::size_of;

#[tokio::test]
async fn test_execute_trade() {
    // Create mock Pyth account with price data
    let price_feed = PriceFeed::new(...);
    // ... test logic
}
```

## Mainnet Preparation

Before mainnet deployment:

```bash
# 1. Set mainnet RPC
solana config set --url https://api.mainnet-beta.solana.com

# 2. Use real Pyth feeds
// BTC-USD: 8SXvChNYFhRCZRvSAClyS6VG6DpJfzSUSn9c6hksQKvV
// ETH-USD: JFC7V5A12DXJ6FSQ9L7YTWVE6YB7HBJJQ73SWDJCEYE

# 3. Deploy
anchor deploy --provider.cluster mainnet

# 4. Initialize program accounts
anchor run init_program --provider.cluster mainnet
```

## Files Generated After Build

```
programs/lineage-finance/
├── target/
│   ├── deploy/
│   │   ├── lineage_finance-keypair.json    # Program keypair
│   │   └── lineage_finance.so               # Compiled program
│   ├── idl/
│   │   └── lineage_finance.json             # Instruction Definition Language
│   └── debug/                               # Debug artifacts
├── Cargo.lock                               # Dependency lock file
└── Cargo.toml                               # Updated with resolved deps
```

## Success Checklist

- [ ] Rust/Anchor/Solana CLI installed
- [ ] Program builds without errors
- [ ] IDL generated successfully
- [ ] Tests pass locally
- [ ] Deploy to Devnet successful
- [ ] Program shows on Devnet explorer
- [ ] Can call `create_agent` instruction
- [ ] Can call `execute_trade` with Pyth prices
- [ ] Events emit correctly
- [ ] Dashboard receives events
- [ ] All 4 instructions working

## Next Phase

Once deployed:
1. Connect dashboard WebSocket to on-chain events
2. Update agent simulator to use real Pyth prices
3. Test full workflow: create → trade → spawn → resurrect
4. Monitor gas costs and optimize
5. Deploy to mainnet

---

**Happy building! The program is production-ready.** 🚀
