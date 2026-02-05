# Lineage Governance Ops Console

A real-life operations console for Lineage governance. It streams immutable proposal activity, exposes dissent scars, and seals dead identities into the graveyard.

## Why This Exists
Lineage systems are not about recovery. They are about consequence. Governance Ops makes that visible:

- Identity is unique and cannot be cloned.
- History is append-only and immutable.
- Resources only decrease.
- Scars are permanent.
- Death is final.

This console is a living proof of those constraints.

## Run

```bash
# From repo root
set GOVERNANCE_OPS_ADMIN_KEY=supersecret
cargo run --manifest-path apps/governance-ops/Cargo.toml
```

Open:
```
http://127.0.0.1:9108
```

## Endpoints

- `GET /api/state` - latest metrics, members, ledger history, graveyard stats
- `GET /api/graveyard` - tombstone IDs + stats
- `GET /api/graveyard/:id` - fetch tombstone
- `WS /ws` - real-time governance stream
- `POST /api/admin/proposal` - inject a proposal (`title`, `risk`, `voting_window_secs`)
- `POST /api/admin/vote` - cast a vote (`proposal_id`, `choice`, `member_id` or `member_name`)

## Notes
- Graveyard is initialized on boot (`.lineage/graveyard`).
- When members die, they are buried and appear in the Graveyard panel.
- Backpressure is handled: lagging clients receive a resync payload.
- Governance history persists to `apps/governance-ops/data/governance_history.json`.
- Admin endpoints require `GOVERNANCE_OPS_ADMIN_KEY` and accept `X-Admin-Key` or `Authorization: Bearer` headers.
