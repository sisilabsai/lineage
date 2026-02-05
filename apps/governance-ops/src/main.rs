use std::collections::VecDeque;
use std::env;
use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use axum::{
    extract::{Path, State, WebSocketUpgrade},
    http::{header::AUTHORIZATION, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum::extract::ws::{Message, WebSocket};
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::task;
use tokio::sync::{broadcast, RwLock};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use lineage::{
    GovernanceConfig, GovernanceCouncil, GovernanceEvent, Graveyard, ProposalOutcome, ProposalRisk,
    Tombstone, VoteChoice,
};

const LEDGER_BUFFER_LIMIT: usize = 250;
const METRICS_SNAPSHOT_LIMIT: usize = 180;
const DEFAULT_INTERVAL_MS: u64 = 1500;

#[derive(Clone)]
struct AppContext {
    tx: broadcast::Sender<String>,
    state: Arc<RwLock<AppState>>,
    history_path: PathBuf,
    admin_key: Option<String>,
}

struct AppState {
    council: GovernanceCouncil,
    ledger_history: Vec<LedgerEntry>,
    ledger_buffer: VecDeque<LedgerEntry>,
    latest_metrics: Option<RoundMetrics>,
    latest_members: Vec<MemberSnapshot>,
    metrics_history: Vec<RoundMetrics>,
    graveyard_ids: Vec<String>,
    ledger_cursor: usize,
}

impl AppState {
    fn new(council: GovernanceCouncil, history: PersistedHistory) -> Self {
        let PersistedHistory {
            ledger_history,
            metrics_history,
            graveyard_ids,
        } = history;
        let latest_metrics = metrics_history.last().cloned();
        let latest_members = member_snapshots(&council);
        let mut state = Self {
            council,
            ledger_history,
            ledger_buffer: VecDeque::with_capacity(LEDGER_BUFFER_LIMIT),
            latest_metrics,
            latest_members,
            metrics_history,
            graveyard_ids,
            ledger_cursor: 0,
        };

        let start = state.ledger_history.len().saturating_sub(LEDGER_BUFFER_LIMIT);
        for entry in state.ledger_history.iter().skip(start) {
            state.ledger_buffer.push_back(entry.clone());
        }

        state
    }

    fn record_ledger(&mut self, entry: LedgerEntry) {
        self.ledger_history.push(entry.clone());
        self.ledger_buffer.push_back(entry);
        while self.ledger_buffer.len() > LEDGER_BUFFER_LIMIT {
            self.ledger_buffer.pop_front();
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RoundMetrics {
    round: u32,
    proposal_id: String,
    title: String,
    risk: String,
    outcome: String,
    for_votes: u32,
    against_votes: u32,
    abstain_votes: u32,
    members: u32,
    turnout_pct: f64,
    dissent_rate_pct: f64,
    scars_round: u32,
    total_damage: u32,
    ledger_total: u32,
    timestamp_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LedgerEntry {
    message: String,
    severity: String,
    timestamp_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MemberSnapshot {
    member_id: String,
    name: String,
    energy: u64,
    damage: u32,
    alive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct PersistedHistory {
    #[serde(default)]
    ledger_history: Vec<LedgerEntry>,
    #[serde(default)]
    metrics_history: Vec<RoundMetrics>,
    #[serde(default)]
    graveyard_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct AdminProposalRequest {
    title: String,
    risk: Option<String>,
    voting_window_secs: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct AdminVoteRequest {
    proposal_id: String,
    member_id: Option<String>,
    member_name: Option<String>,
    choice: String,
}

#[tokio::main]
async fn main() {
    if let Err(err) = Graveyard::initialize() {
        eprintln!("Graveyard init failed: {}", err);
    }

    let history_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("governance_history.json");
    let history = load_history(&history_path);
    let admin_key = env::var("GOVERNANCE_OPS_ADMIN_KEY").ok();
    if admin_key.is_none() {
        eprintln!("Admin endpoints disabled: GOVERNANCE_OPS_ADMIN_KEY not set.");
    }

    let council = seed_council();
    let state = Arc::new(RwLock::new(AppState::new(council, history)));
    {
        let mut guard = state.write().await;
        for id in Graveyard::list_all() {
            if !guard.graveyard_ids.contains(&id) {
                guard.graveyard_ids.push(id);
            }
        }
    }
    let (tx, _) = broadcast::channel::<String>(512);

    let context = AppContext {
        tx: tx.clone(),
        state: Arc::clone(&state),
        history_path: history_path.clone(),
        admin_key,
    };

    tokio::spawn(governance_loop(
        tx.clone(),
        Arc::clone(&state),
        history_path.clone(),
    ));

    let web_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("web");
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/api/state", get(api_state))
        .route("/api/graveyard", get(api_graveyard))
        .route("/api/graveyard/:id", get(api_tombstone))
        .route("/api/admin/proposal", post(admin_proposal))
        .route("/api/admin/vote", post(admin_vote))
        .fallback_service(ServeDir::new(web_root).append_index_html_on_directories(true))
        .with_state(context);

    let addr = SocketAddr::from(([127, 0, 0, 1], 9108));
    println!("Governance Ops Console running at http://{}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(
    State(context): State<AppContext>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, context))
}

async fn handle_socket(mut socket: WebSocket, context: AppContext) {
    let mut rx = context.tx.subscribe();

    let status_payload = json!({
        "type": "status",
        "mode": "live",
        "timestamp": chrono::Utc::now().timestamp_millis(),
    })
    .to_string();
    let _ = socket.send(Message::Text(status_payload)).await;

    if let Some(snapshot) = build_snapshot(&context.state).await {
        let _ = socket.send(Message::Text(snapshot)).await;
    }

    loop {
        match rx.recv().await {
            Ok(message) => {
                if socket.send(Message::Text(message)).await.is_err() {
                    break;
                }
            }
            Err(broadcast::error::RecvError::Lagged(skipped)) => {
                if let Some(payload) = build_resync(&context.state, skipped).await {
                    if socket.send(Message::Text(payload)).await.is_err() {
                        break;
                    }
                }
            }
            Err(broadcast::error::RecvError::Closed) => break,
        }
    }
}

async fn build_snapshot(state: &Arc<RwLock<AppState>>) -> Option<String> {
    let state = state.read().await;
    let metrics_history = metrics_history_slice(&state);
    let metrics = state.latest_metrics.clone();
    let members = state.latest_members.clone();
    let ledger_events: Vec<LedgerEntry> = state.ledger_buffer.iter().cloned().collect();
    let graveyard_ids = state.graveyard_ids.clone();
    if state.latest_metrics.is_none()
        && state.ledger_buffer.is_empty()
        && metrics_history.is_empty()
    {
        return None;
    }

    Some(
        json!({
            "type": "snapshot",
            "metrics": metrics,
            "metrics_history": metrics_history,
            "members": members,
            "ledger_events": ledger_events,
            "graveyard_ids": graveyard_ids,
        })
        .to_string(),
    )
}

async fn build_resync(state: &Arc<RwLock<AppState>>, skipped: u64) -> Option<String> {
    let state = state.read().await;
    let metrics = state.latest_metrics.clone();
    let members = state.latest_members.clone();
    let ledger_events: Vec<LedgerEntry> = state.ledger_buffer.iter().cloned().collect();
    let graveyard_ids = state.graveyard_ids.clone();
    Some(
        json!({
            "type": "resync",
            "skipped": skipped,
            "metrics": metrics,
            "metrics_history": metrics_history_slice(&state),
            "members": members,
            "ledger_events": ledger_events,
            "graveyard_ids": graveyard_ids,
        })
        .to_string(),
    )
}

async fn api_state(State(context): State<AppContext>) -> impl IntoResponse {
    let state = context.state.read().await;
    let graveyard_stats = graveyard_stats_value();
    let metrics = state.latest_metrics.clone();
    let members = state.latest_members.clone();
    let ledger_history = state.ledger_history.clone();
    let metrics_history = state.metrics_history.clone();
    let graveyard_ids = state.graveyard_ids.clone();

    Json(json!({
        "metrics": metrics,
        "metrics_history": metrics_history,
        "members": members,
        "ledger_history": ledger_history,
        "graveyard_ids": graveyard_ids,
        "graveyard_stats": graveyard_stats,
    }))
}

async fn api_graveyard() -> impl IntoResponse {
    let ids = Graveyard::list_all();
    let stats = graveyard_stats_value();
    Json(json!({ "ids": ids, "stats": stats }))
}

async fn api_tombstone(Path(id): Path<String>) -> Result<Json<Tombstone>, axum::http::StatusCode> {
    match Graveyard::load(&id) {
        Ok(tombstone) => Ok(Json(tombstone)),
        Err(_) => Err(axum::http::StatusCode::NOT_FOUND),
    }
}

async fn admin_proposal(
    State(context): State<AppContext>,
    headers: HeaderMap,
    Json(payload): Json<AdminProposalRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    authorize_admin(&headers, &context.admin_key)?;
    let title = payload.title.trim();
    if title.is_empty() {
        return Err(bad_request("title is required"));
    }

    let risk = match payload.risk.as_deref() {
        None => ProposalRisk::Medium,
        Some(value) => parse_risk(value)
            .ok_or_else(|| bad_request("risk must be low, medium, or high"))?,
    };
    let voting_window_secs = payload.voting_window_secs.unwrap_or(45);
    if voting_window_secs <= 0 {
        return Err(bad_request("voting_window_secs must be greater than 0"));
    }

    let (proposal_id, events, history_snapshot) = {
        let mut guard = context.state.write().await;
        let proposal_id = guard
            .council
            .propose(title.to_string(), risk, voting_window_secs);
        let mut ledger_cursor = guard.ledger_cursor;
        let events = ledger_entries(&guard.council, &mut ledger_cursor);
        guard.ledger_cursor = ledger_cursor;
        for entry in &events {
            guard.record_ledger(entry.clone());
        }
        let ledger_total = guard.ledger_history.len() as u32;
        if let Some(metrics) = guard.latest_metrics.as_mut() {
            metrics.ledger_total = ledger_total;
        }
        if let Some(metrics) = guard.metrics_history.last_mut() {
            metrics.ledger_total = ledger_total;
        }
        let history_snapshot = build_history_snapshot(&guard);
        (proposal_id.as_str().to_string(), events, history_snapshot)
    };

    broadcast_events(&context.tx, &events);
    persist_history_async(context.history_path.clone(), history_snapshot);

    Ok(Json(json!({
        "ok": true,
        "proposal_id": proposal_id,
        "events": events,
    })))
}

async fn admin_vote(
    State(context): State<AppContext>,
    headers: HeaderMap,
    Json(payload): Json<AdminVoteRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    authorize_admin(&headers, &context.admin_key)?;
    let choice = parse_vote_choice(&payload.choice)
        .ok_or_else(|| bad_request("choice must be for, against, or abstain"))?;

    let (receipt, events, members, buried_ids, burial_entries, history_snapshot) = {
        let mut guard = context.state.write().await;
        let member_id = match resolve_member_id(&guard.council, &payload) {
            Ok(id) => id,
            Err(err) => return Err(bad_request(&err)),
        };

        let receipt = match guard
            .council
            .vote_by_id(&payload.proposal_id, &member_id, choice)
        {
            Ok(receipt) => receipt,
            Err(err) => return Err(bad_request(&err.to_string())),
        };

        let mut ledger_cursor = guard.ledger_cursor;
        let events = ledger_entries(&guard.council, &mut ledger_cursor);
        guard.ledger_cursor = ledger_cursor;
        for entry in &events {
            guard.record_ledger(entry.clone());
        }

        let buried_ids = guard.council.bury_dead_members();
        let mut burial_entries = Vec::new();
        for id in &buried_ids {
            let entry = LedgerEntry {
                message: format!("Tombstone sealed: {}", id),
                severity: "critical".to_string(),
                timestamp_ms: chrono::Utc::now().timestamp_millis(),
            };
            guard.record_ledger(entry.clone());
            burial_entries.push(entry);
            if !guard.graveyard_ids.contains(id) {
                guard.graveyard_ids.push(id.clone());
            }
        }

        let members = member_snapshots(&guard.council);
        guard.latest_members = members.clone();
        let ledger_total = guard.ledger_history.len() as u32;
        if let Some(metrics) = guard.latest_metrics.as_mut() {
            metrics.ledger_total = ledger_total;
        }
        if let Some(metrics) = guard.metrics_history.last_mut() {
            metrics.ledger_total = ledger_total;
        }

        let history_snapshot = build_history_snapshot(&guard);

        (
            receipt,
            events,
            members,
            buried_ids,
            burial_entries,
            history_snapshot,
        )
    };

    let receipt_value = json!({
        "proposal_id": receipt.proposal_id,
        "member_id": receipt.member_id,
        "choice": vote_label(receipt.choice),
        "energy_cost": receipt.energy_cost,
        "timestamp": receipt.timestamp.timestamp_millis(),
    });

    broadcast_events(&context.tx, &events);
    broadcast_events(&context.tx, &burial_entries);
    broadcast_members(&context.tx, &members);

    for id in &buried_ids {
        let payload = json!({
            "type": "graveyard_event",
            "id": id,
            "timestamp": chrono::Utc::now().timestamp_millis(),
        });
        let _ = context.tx.send(payload.to_string());
    }

    persist_history_async(context.history_path.clone(), history_snapshot);

    Ok(Json(json!({
        "ok": true,
        "receipt": receipt_value,
        "events": events,
        "buried_ids": buried_ids,
    })))
}

async fn governance_loop(
    tx: broadcast::Sender<String>,
    state: Arc<RwLock<AppState>>,
    history_path: PathBuf,
) {
    let mut rng = StdRng::from_entropy();
    let mut round = {
        let guard = state.read().await;
        guard
            .metrics_history
            .last()
            .map(|metrics| metrics.round + 1)
            .unwrap_or(1)
    };

    loop {
        let (metrics, events, members, buried_ids, burial_entries, history_snapshot) = {
            let mut guard = state.write().await;
            let mut ledger_cursor = guard.ledger_cursor;
            let (mut metrics, events, members) = run_round(
                &mut guard.council,
                &mut rng,
                round,
                &mut ledger_cursor,
            );
            guard.ledger_cursor = ledger_cursor;

            for entry in &events {
                guard.record_ledger(entry.clone());
            }

            let buried_ids = guard.council.bury_dead_members();
            let mut burial_entries = Vec::new();
            for id in &buried_ids {
                let entry = LedgerEntry {
                    message: format!("Tombstone sealed: {}", id),
                    severity: "critical".to_string(),
                    timestamp_ms: chrono::Utc::now().timestamp_millis(),
                };
                guard.record_ledger(entry.clone());
                burial_entries.push(entry);
                if !guard.graveyard_ids.contains(id) {
                    guard.graveyard_ids.push(id.clone());
                }
            }

            metrics.ledger_total = guard.ledger_history.len() as u32;
            guard.latest_metrics = Some(metrics.clone());
            guard.latest_members = members.clone();
            guard.metrics_history.push(metrics.clone());

            let history_snapshot = build_history_snapshot(&guard);

            (
                metrics,
                events,
                members,
                buried_ids,
                burial_entries,
                history_snapshot,
            )
        };

        broadcast_metrics(&tx, &metrics);
        broadcast_members(&tx, &members);
        broadcast_events(&tx, &events);
        broadcast_events(&tx, &burial_entries);

        for id in buried_ids {
            let payload = json!({
                "type": "graveyard_event",
                "id": id,
                "timestamp": chrono::Utc::now().timestamp_millis(),
            });
            let _ = tx.send(payload.to_string());
        }

        persist_history_async(history_path.clone(), history_snapshot);

        round += 1;
        tokio::time::sleep(Duration::from_millis(DEFAULT_INTERVAL_MS)).await;
    }
}

fn run_round(
    council: &mut GovernanceCouncil,
    rng: &mut StdRng,
    round: u32,
    ledger_cursor: &mut usize,
) -> (RoundMetrics, Vec<LedgerEntry>, Vec<MemberSnapshot>) {
    let titles = [
        "Increase validator penalties",
        "Reduce block rewards",
        "Adopt new treasury policy",
        "Emergency governance patch",
        "Freeze unstable markets",
        "Raise quorum threshold",
    ];

    let title = titles[rng.gen_range(0..titles.len())].to_string();
    let risk = match rng.gen_range(0..3) {
        0 => ProposalRisk::Low,
        1 => ProposalRisk::Medium,
        _ => ProposalRisk::High,
    };

    let total_damage_before = total_damage(council);
    let proposal_id = council.propose(title.clone(), risk, 45);

    let member_ids = council.member_ids();
    let mut for_votes = 0u32;
    let mut against_votes = 0u32;
    let mut abstain_votes = 0u32;

    for member_id in &member_ids {
        let choice = vote_choice_for_risk(rng, risk);
        if council
            .vote(proposal_id.clone(), member_id, choice)
            .is_ok()
        {
            match choice {
                VoteChoice::For => for_votes += 1,
                VoteChoice::Against => against_votes += 1,
                VoteChoice::Abstain => abstain_votes += 1,
            }
        }
    }

    let outcome = council
        .close(proposal_id.clone())
        .unwrap_or(ProposalOutcome::Failed);

    let total_damage_after = total_damage(council);
    let scars_round = total_damage_after.saturating_sub(total_damage_before);

    let total_votes = for_votes + against_votes + abstain_votes;
    let members = member_ids.len() as u32;
    let turnout_pct = if members == 0 {
        0.0
    } else {
        (total_votes as f64 / members as f64) * 100.0
    };

    let dissenters = match outcome {
        ProposalOutcome::Passed => against_votes,
        ProposalOutcome::Failed => for_votes,
        ProposalOutcome::NoQuorum => 0,
    };
    let dissent_rate_pct = if total_votes == 0 {
        0.0
    } else {
        (dissenters as f64 / total_votes as f64) * 100.0
    };

    let ledger_total = council.ledger().events().len() as u32;

    let metrics = RoundMetrics {
        round,
        proposal_id: proposal_id.as_str().to_string(),
        title,
        risk: risk_label(risk).to_string(),
        outcome: outcome_label(outcome).to_string(),
        for_votes,
        against_votes,
        abstain_votes,
        members,
        turnout_pct,
        dissent_rate_pct,
        scars_round,
        total_damage: total_damage_after,
        ledger_total,
        timestamp_ms: chrono::Utc::now().timestamp_millis(),
    };

    let events = ledger_entries(council, ledger_cursor);
    let members_snapshot = member_snapshots(council);
    (metrics, events, members_snapshot)
}

fn seed_council() -> GovernanceCouncil {
    let config = GovernanceConfig {
        vote_cost: 60,
        abstain_cost: 30,
        ..GovernanceConfig::default()
    };
    let mut council = GovernanceCouncil::new(config);
    let _ = council.add_member("Chair".to_string(), 600);
    let _ = council.add_member("Guardians".to_string(), 600);
    let _ = council.add_member("Treasury".to_string(), 600);
    let _ = council.add_member("Protocol".to_string(), 600);
    let _ = council.add_member("Risk".to_string(), 600);
    council
}

fn total_damage(council: &GovernanceCouncil) -> u32 {
    council
        .member_ids()
        .iter()
        .filter_map(|id| council.member_damage(id))
        .sum()
}

fn member_snapshots(council: &GovernanceCouncil) -> Vec<MemberSnapshot> {
    let mut snapshots = Vec::new();

    for member_id in council.member_ids() {
        let name = council.member_name(&member_id).unwrap_or("Unknown");
        let energy = council.member_energy(&member_id).unwrap_or(0);
        let damage = council.member_damage(&member_id).unwrap_or(0);
        let alive = council.member_is_alive(&member_id).unwrap_or(false);
        snapshots.push(MemberSnapshot {
            member_id,
            name: name.to_string(),
            energy,
            damage,
            alive,
        });
    }

    snapshots.sort_by(|a, b| a.name.cmp(&b.name));
    snapshots
}

fn vote_choice_for_risk(rng: &mut StdRng, risk: ProposalRisk) -> VoteChoice {
    let roll: f64 = rng.r#gen();
    match risk {
        ProposalRisk::Low => {
            if roll < 0.7 {
                VoteChoice::For
            } else if roll < 0.9 {
                VoteChoice::Abstain
            } else {
                VoteChoice::Against
            }
        }
        ProposalRisk::Medium => {
            if roll < 0.5 {
                VoteChoice::For
            } else if roll < 0.8 {
                VoteChoice::Against
            } else {
                VoteChoice::Abstain
            }
        }
        ProposalRisk::High => {
            if roll < 0.3 {
                VoteChoice::For
            } else if roll < 0.85 {
                VoteChoice::Against
            } else {
                VoteChoice::Abstain
            }
        }
    }
}

fn ledger_entries(council: &GovernanceCouncil, ledger_index: &mut usize) -> Vec<LedgerEntry> {
    let events = council.ledger().events();
    if *ledger_index >= events.len() {
        return Vec::new();
    }

    let slice = &events[*ledger_index..];
    *ledger_index = events.len();

    slice.iter().map(event_to_entry).collect()
}

fn event_to_entry(event: &GovernanceEvent) -> LedgerEntry {
    let (message, severity, timestamp) = match event {
        GovernanceEvent::MemberAdded { name, timestamp, .. } => (
            format!("Member admitted: {}", name),
            "info",
            timestamp.timestamp_millis(),
        ),
        GovernanceEvent::ProposalCreated {
            title, risk, timestamp, ..
        } => (
            format!("Proposal opened: {} ({})", title, risk_label(*risk)),
            "info",
            timestamp.timestamp_millis(),
        ),
        GovernanceEvent::VoteCast {
            member_id,
            choice,
            timestamp,
            ..
        } => (
            format!("Vote cast: {} -> {}", short_id(member_id), vote_label(*choice)),
            "info",
            timestamp.timestamp_millis(),
        ),
        GovernanceEvent::ProposalClosed {
            outcome, turnout_pct, timestamp, ..
        } => (
            format!(
                "Proposal closed: {} (turnout {:.1}%)",
                outcome_label(*outcome),
                turnout_pct
            ),
            "info",
            timestamp.timestamp_millis(),
        ),
        GovernanceEvent::DissentScarred {
            member_id,
            reason,
            timestamp,
            ..
        } => (
            format!("Dissent scarred: {} ({})", short_id(member_id), reason),
            "warning",
            timestamp.timestamp_millis(),
        ),
        GovernanceEvent::ProposalExecuted {
            success, timestamp, ..
        } => (
            format!(
                "Execution {}",
                if *success { "succeeded" } else { "failed" }
            ),
            if *success { "info" } else { "critical" },
            timestamp.timestamp_millis(),
        ),
    };

    LedgerEntry {
        message,
        severity: severity.to_string(),
        timestamp_ms: timestamp,
    }
}

fn broadcast_metrics(tx: &broadcast::Sender<String>, metrics: &RoundMetrics) {
    let payload = json!({
        "type": "metrics",
        "round": metrics.round,
        "proposal_id": metrics.proposal_id,
        "title": metrics.title,
        "risk": metrics.risk,
        "outcome": metrics.outcome,
        "for_votes": metrics.for_votes,
        "against_votes": metrics.against_votes,
        "abstain_votes": metrics.abstain_votes,
        "members": metrics.members,
        "turnout_pct": metrics.turnout_pct,
        "dissent_rate_pct": metrics.dissent_rate_pct,
        "scars_round": metrics.scars_round,
        "total_damage": metrics.total_damage,
        "ledger_total": metrics.ledger_total,
        "timestamp": metrics.timestamp_ms,
    });
    let _ = tx.send(payload.to_string());
}

fn broadcast_events(tx: &broadcast::Sender<String>, events: &[LedgerEntry]) {
    for entry in events {
        let payload = json!({
            "type": "ledger_event",
            "message": entry.message,
            "severity": entry.severity,
            "timestamp": entry.timestamp_ms,
        });
        let _ = tx.send(payload.to_string());
    }
}

fn broadcast_members(tx: &broadcast::Sender<String>, members: &[MemberSnapshot]) {
    let payload = json!({
        "type": "members",
        "members": members,
    });
    let _ = tx.send(payload.to_string());
}

fn risk_label(risk: ProposalRisk) -> &'static str {
    match risk {
        ProposalRisk::Low => "Low",
        ProposalRisk::Medium => "Medium",
        ProposalRisk::High => "High",
    }
}

fn outcome_label(outcome: ProposalOutcome) -> &'static str {
    match outcome {
        ProposalOutcome::Passed => "Passed",
        ProposalOutcome::Failed => "Failed",
        ProposalOutcome::NoQuorum => "NoQuorum",
    }
}

fn vote_label(choice: VoteChoice) -> &'static str {
    match choice {
        VoteChoice::For => "For",
        VoteChoice::Against => "Against",
        VoteChoice::Abstain => "Abstain",
    }
}

fn short_id(id: &str) -> String {
    if id.len() <= 6 {
        id.to_string()
    } else {
        format!("{}...", &id[..6])
    }
}

fn authorize_admin(
    headers: &HeaderMap,
    admin_key: &Option<String>,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let expected = match admin_key {
        Some(key) => key.as_str(),
        None => {
            return Err((
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({
                    "ok": false,
                    "error": "admin key not configured"
                })),
            ))
        }
    };

    let mut provided = headers
        .get("x-admin-key")
        .and_then(|value| value.to_str().ok())
        .map(|value| value.trim().to_string());

    if provided.is_none() {
        provided = headers.get(AUTHORIZATION).and_then(|value| value.to_str().ok()).and_then(
            |value| {
                let value = value.trim();
                if let Some(stripped) = value.strip_prefix("Bearer ") {
                    Some(stripped.trim().to_string())
                } else {
                    None
                }
            },
        );
    }

    match provided.as_deref() {
        Some(key) if key == expected => Ok(()),
        _ => Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "ok": false,
                "error": "invalid admin key"
            })),
        )),
    }
}

fn bad_request(message: &str) -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::BAD_REQUEST, Json(json!({ "ok": false, "error": message })))
}

fn parse_risk(value: &str) -> Option<ProposalRisk> {
    match value.trim().to_lowercase().as_str() {
        "low" => Some(ProposalRisk::Low),
        "medium" => Some(ProposalRisk::Medium),
        "high" => Some(ProposalRisk::High),
        _ => None,
    }
}

fn parse_vote_choice(value: &str) -> Option<VoteChoice> {
    match value.trim().to_lowercase().as_str() {
        "for" => Some(VoteChoice::For),
        "against" => Some(VoteChoice::Against),
        "abstain" => Some(VoteChoice::Abstain),
        _ => None,
    }
}

fn resolve_member_id(
    council: &GovernanceCouncil,
    request: &AdminVoteRequest,
) -> Result<String, String> {
    if let Some(member_id) = request.member_id.as_ref() {
        if council.member_name(member_id).is_some() {
            return Ok(member_id.clone());
        }
        return Err(format!("member_id {} not found", member_id));
    }

    if let Some(member_name) = request.member_name.as_ref() {
        let matches: Vec<String> = council
            .member_ids()
            .into_iter()
            .filter(|id| {
                council
                    .member_name(id)
                    .map(|name| name.eq_ignore_ascii_case(member_name))
                    .unwrap_or(false)
            })
            .collect();

        return match matches.len() {
            0 => Err(format!("member_name {} not found", member_name)),
            1 => Ok(matches[0].clone()),
            _ => Err("member_name is ambiguous, provide member_id".to_string()),
        };
    }

    Err("member_id or member_name is required".to_string())
}

fn metrics_history_slice(state: &AppState) -> Vec<RoundMetrics> {
    let start = state
        .metrics_history
        .len()
        .saturating_sub(METRICS_SNAPSHOT_LIMIT);
    state.metrics_history[start..].to_vec()
}

fn load_history(path: &PathBuf) -> PersistedHistory {
    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(err) => {
            if err.kind() != std::io::ErrorKind::NotFound {
                eprintln!("History load failed: {}", err);
            }
            return PersistedHistory::default();
        }
    };

    match serde_json::from_reader(file) {
        Ok(history) => history,
        Err(err) => {
            eprintln!("History parse failed: {}", err);
            PersistedHistory::default()
        }
    }
}

fn build_history_snapshot(state: &AppState) -> PersistedHistory {
    PersistedHistory {
        ledger_history: state.ledger_history.clone(),
        metrics_history: state.metrics_history.clone(),
        graveyard_ids: state.graveyard_ids.clone(),
    }
}

fn persist_history_async(path: PathBuf, history: PersistedHistory) {
    task::spawn_blocking(move || {
        if let Some(parent) = path.parent() {
            if let Err(err) = fs::create_dir_all(parent) {
                eprintln!("History directory create failed: {}", err);
                return;
            }
        }

        let temp_path = path.with_extension("tmp");
        let file = match fs::File::create(&temp_path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("History write failed: {}", err);
                return;
            }
        };

        if let Err(err) = serde_json::to_writer_pretty(&file, &history) {
            eprintln!("History serialize failed: {}", err);
            return;
        }

        if let Err(_err) = fs::rename(&temp_path, &path) {
            if path.exists() {
                let _ = fs::remove_file(&path);
            }
            if let Err(err) = fs::rename(&temp_path, &path) {
                eprintln!("History replace failed: {}", err);
            }
        }
    });
}

fn graveyard_stats_value() -> Option<serde_json::Value> {
    Graveyard::statistics().ok().map(|stats| {
        json!({
            "total_agents": stats.total_agents,
            "average_lifespan_seconds": stats.average_lifespan_seconds,
            "average_efficiency": stats.average_efficiency,
            "total_scars": stats.total_scars,
            "most_common_scar_count": stats.most_common_scar_count,
            "highest_legacy_score": stats.highest_legacy_score,
        })
    })
}
