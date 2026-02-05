# Lineage (lineage-rs) v0.2.2

Lineage is a Rust framework for building autonomous systems where identity is unique, history is permanent, and consequences are irreversible.

Core guarantees:
- Unique identity. No duplication or cloning.
- Append-only history. Every action is recorded.
- Finite energy and capital. Resources only decrease.
- Permanent scars. Damage never disappears.
- Irreversible death. No resurrection or rollback.

## Table of Contents

- Overview
- Philosophy
- Quick Start
- Core Concepts
- Libraries and Modules
- Dashboards and UIs
- Apps
- Programs
- Examples
- Repository Layout
- Documentation
- Testing
- Contributing
- License

## Overview

Lineage provides a strict, consequence-first model for agents and systems. It is built for simulations, governance, finance, provenance tracking, and any domain where immutable history and irreversible outcomes matter.

## Philosophy

Lineage enforces constraints at the API and type level:
- No rollback of history.
- No cloning of identities.
- No energy or capital reset.
- No scar removal.
- No resurrection without explicit mechanics.

Use Lineage when you need verifiable accountability and long-lived consequences. Do not use it if your system depends on undo, mutable history, or state reset.

## Quick Start

1. Add the dependency

```toml
[dependencies]
lineage = { path = "../lineage" }
```

2. Create your first agent

```rust
use lineage::TaskAgent;

let agent = TaskAgent::create(1000);
println!("Agent id: {}", agent.identity().id());
```

3. Run a core example

```bash
cargo run --example arena_with_live_market --release
```

4. Run the governance dashboard

```bash
cargo run --example governance_ws_broadcast --release
```

Open `examples/governance_dashboard.html`. See `examples/DASHBOARD_SETUP.md` for setup.

## Core Concepts

- Identity: cryptographically unique and non-clonable identifiers.
- Memory: append-only event history.
- Metabolism: finite energy model.
- Scars: permanent damage records.
- Graveyard: sealed tombstones for dead agents.
- Trust: capability scoring based on behavior.

## Libraries and Modules

### Core System

Modules in `src/` provide identity, memory, metabolism, scars, and lifecycle orchestration.

### Agent

Task-driven agents with irreversible energy spend:

```rust
use lineage::{TaskAgent, Task, TaskOutcome};

let mut agent = TaskAgent::create(500);
let task = Task::new("Execute governance vote".to_string(), 20);
let result = agent.execute_task(task, TaskOutcome::Success);
```

### Finance

Trading agents, arenas, and market data clients live in `src/finance/`.

```rust
use lineage::finance::{FinanceAgent, MarketDataClient};

let mut agent = FinanceAgent::new("Momentum".to_string(), 100_000, 0);
let client = MarketDataClient::new(5);
```

Market data can use real providers when API keys are available. See `COINMARKETCAP_INTEGRATION.md` and `FINANCE_GETTING_STARTED.md`.

### Governance

Proposal, vote, and ledger workflows in `src/governance/`:

```rust
use lineage::{GovernanceCouncil, GovernanceConfig, ProposalRisk, VoteChoice};

let mut council = GovernanceCouncil::new(GovernanceConfig::default());
let member_id = council.add_member("Treasury".to_string(), 600);
let proposal_id = council.propose("Increase quorum".to_string(), ProposalRisk::Medium, 60);
let _ = council.vote(proposal_id.clone(), &member_id, VoteChoice::For);
let _ = council.close(proposal_id);
```

### Provenance

Immutable chain-of-custody in `src/provenance/`:

```rust
use lineage::provenance::{ProvenanceVault, ProvenanceConfig, Asset};

let mut vault = ProvenanceVault::new(ProvenanceConfig::default());
let asset = Asset::new("Vaulted Artifact".to_string());
vault.register_asset(asset).ok();
```

### Graveyard

Sealed tombstones and postmortem data:

```rust
use lineage::Graveyard;

let _ = Graveyard::initialize();
let all = Graveyard::list_all();
```

### ML Feature

Machine learning examples are gated behind the `ml` feature.

```bash
cargo run --example ml_learning_advanced --features ml
```

### Libtorch (Required for ML Examples)

The ML examples use PyTorch's C++ distribution (`libtorch`). These binaries are large and are intentionally excluded from git.

1. Download the libtorch archive for your OS.
2. Extract it to `libs/libtorch/`.
3. Re-run the ML examples.

Download links:

```text
https://pytorch.org/get-started/locally/
https://download.pytorch.org/libtorch/
```

## Dashboards and UIs

- Governance Web Dashboard: `examples/governance_dashboard.html` with `governance_ws_broadcast`.
- Governance TUI Arena: `cargo run --example consensus_arena_tui`.
- Finance Web Dashboard: `examples/dashboard.html` with `examples/serve_dashboard.py`.
- Trust Score Dashboard: `cargo run --example trust_score_dashboard` and `examples/TRUST_SCORE_DASHBOARD_README.md`.
- Metrics Server: `cargo run --example metrics_server --release` for Prometheus-style metrics.

## Apps

### Governance Ops Console

A full app in `apps/governance-ops/` uses governance + graveyard + websockets.

Run it:

```bash
set GOVERNANCE_OPS_ADMIN_KEY=supersecret
cargo run --manifest-path apps/governance-ops/Cargo.toml
```

Open `http://127.0.0.1:9108`.

Admin endpoints require `GOVERNANCE_OPS_ADMIN_KEY` and accept `X-Admin-Key` or `Authorization: Bearer`.

## Programs

### Lineage Finance (Solana)

The `programs/lineage-finance/` package is a Solana program that mirrors the off-chain finance library. See `programs/lineage-finance/README.md` for build and deployment instructions.

## Examples

Finance and Market:
- `arena_with_live_market.rs`
- `decentralized_trading_agent.rs`
- `market_data_integration.rs`

Governance:
- `governance_ws_broadcast.rs`
- `consensus_arena_tui.rs`
- `distributed_consensus_voting.rs`
- `interactive_consensus_arena.rs`

Core System:
- `lifecycle_demo.rs`
- `persistent_audit_daemon.rs`
- `ghost_in_the_machine.rs`
- `mortality.rs`

Trust:
- `trust_score_dashboard.rs`

Graveyard:
- `graveyard_inspector.rs`

Provenance:
- `provenance_chain_demo.rs`
- `provenance_recall_sim.rs`

ML:
- `ml_learning_advanced.rs`
- `ml_finance_integration.rs`
- `validate_ml_learning.rs`
- `phase3_training_with_evolution.rs`
- `training_loop_example.rs`

Networking:
- `ws_broadcast.rs`
- `ws_broadcast_v2.rs`
- `ws_client.rs`
- `metrics_server.rs`
- `metrics_server_v2.rs`

## Repository Layout

- `src/` core library modules and public API.
- `examples/` runnable demos and dashboards.
- `apps/` real applications built on Lineage.
- `programs/` on-chain and external integrations.
- `docs/` and `*.md` documents and delivery notes.

## Documentation

Start here:
- `START_HERE.md`
- `QUICK_REFERENCE.md`
- `CODE_ARCHITECTURE.md`
- `DOCTRINE.md`

Governance:
- `PHASE_3_WEB_DASHBOARD.md`
- `examples/DASHBOARD_SETUP.md`
- `PHASE_3_COMPLETION_WEB.md`

Finance:
- `FINANCE_GETTING_STARTED.md`
- `MARKET_DATA_INTEGRATION.md`
- `COINMARKETCAP_INTEGRATION.md`

Graveyard and Provenance:
- `GRAVEYARD_GUIDE.md`
- `GRAVEYARD_INDEX.md`
- `GRAVEYARD_TESTING.md`

## Testing

```bash
cargo test --release
```

ML examples:

```bash
cargo test --features ml
```

## Contributing

See `CONTRIBUTING.md` and `CODE_OF_CONDUCT.md`.

## License

MIT. See `LICENSE`.
