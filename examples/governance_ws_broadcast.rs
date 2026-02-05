use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use futures_util::SinkExt;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, RwLock};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

use lineage::{
    GovernanceConfig, GovernanceCouncil, GovernanceEvent, ProposalOutcome, ProposalRisk, VoteChoice,
};

#[derive(Clone)]
struct Settings {
    replay: bool,
    rounds: u32,
    interval_ms: u64,
    seed: Option<u64>,
    replay_file: Option<PathBuf>,
    record_file: Option<PathBuf>,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReplayFrame {
    metrics: RoundMetrics,
    events: Vec<LedgerEntry>,
    members: Vec<MemberSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReplaySession {
    frames: Vec<ReplayFrame>,
    ledger_history: Vec<LedgerEntry>,
}

struct ServerState {
    latest_metrics: Option<RoundMetrics>,
    latest_members: Vec<MemberSnapshot>,
    ledger_buffer: VecDeque<LedgerEntry>,
    buffer_limit: usize,
}

impl ServerState {
    fn new(buffer_limit: usize) -> Self {
        Self {
            latest_metrics: None,
            latest_members: Vec::new(),
            ledger_buffer: VecDeque::with_capacity(buffer_limit),
            buffer_limit,
        }
    }

    fn record_ledger(&mut self, entry: LedgerEntry) {
        self.ledger_buffer.push_back(entry);
        while self.ledger_buffer.len() > self.buffer_limit {
            self.ledger_buffer.pop_front();
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = parse_args();

    let listener = TcpListener::bind("127.0.0.1:9001").await?;
    println!("Lineage Governance WS server listening on ws://127.0.0.1:9001");
    println!("Dashboard: open examples/governance_dashboard.html");
    println!("Mode: {}", if settings.replay { "REPLAY" } else { "LIVE" });

    let status_payload = build_status_payload(&settings);
    let (tx, _) = broadcast::channel::<String>(512);
    let state = Arc::new(RwLock::new(ServerState::new(200)));

    let generator_settings = settings.clone();
    let generator_tx = tx.clone();
    let generator_state = Arc::clone(&state);
    tokio::spawn(async move {
        if let Err(err) = run_generator(generator_tx, generator_settings, generator_state).await {
            eprintln!("Generator error: {}", err);
        }
    });

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Client connected from {}", addr);

        let rx = tx.subscribe();
        let status = status_payload.clone();
        let client_state = Arc::clone(&state);
        tokio::spawn(async move {
            if let Err(err) = handle_client(stream, rx, status, client_state).await {
                eprintln!("Client error: {}", err);
            }
        });
    }
}

async fn handle_client(
    stream: tokio::net::TcpStream,
    mut rx: broadcast::Receiver<String>,
    status_payload: String,
    state: Arc<RwLock<ServerState>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut ws_stream = accept_async(stream).await?;
    let _ = ws_stream.send(Message::Text(status_payload)).await;

    loop {
        match rx.recv().await {
            Ok(message) => {
                if ws_stream.send(Message::Text(message)).await.is_err() {
                    break;
                }
            }
            Err(broadcast::error::RecvError::Lagged(skipped)) => {
                let resync = build_resync_payload(&state, skipped).await;
                if let Some(payload) = resync {
                    if ws_stream.send(Message::Text(payload)).await.is_err() {
                        break;
                    }
                }
            }
            Err(broadcast::error::RecvError::Closed) => break,
        }
    }

    Ok(())
}

async fn build_resync_payload(
    state: &Arc<RwLock<ServerState>>,
    skipped: u64,
) -> Option<String> {
    let state = state.read().await;
    let ledger_events: Vec<LedgerEntry> = state.ledger_buffer.iter().cloned().collect();

    if state.latest_metrics.is_none() && ledger_events.is_empty() {
        return None;
    }

    let payload = json!({
        "type": "resync",
        "skipped": skipped,
        "metrics": state.latest_metrics,
        "members": state.latest_members,
        "ledger_events": ledger_events,
    });

    Some(payload.to_string())
}

async fn run_generator(
    tx: broadcast::Sender<String>,
    settings: Settings,
    state: Arc<RwLock<ServerState>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = match settings.seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_entropy(),
    };

    if settings.replay {
        let session = if let Some(path) = &settings.replay_file {
            match load_replay_file(path) {
                Ok(session) => session,
                Err(err) => {
                    eprintln!("Replay file load failed: {}. Generating fresh frames.", err);
                    build_replay_session(settings.rounds, &mut rng)
                }
            }
        } else {
            build_replay_session(settings.rounds, &mut rng)
        };

        if let Some(path) = &settings.record_file {
            save_replay_file(path, &session)?;
        }

        replay_loop(&tx, &state, &session.frames, settings.interval_ms).await?;
    } else {
        live_loop(
            &tx,
            &state,
            &mut rng,
            settings.interval_ms,
            settings.record_file,
            settings.rounds,
        )
        .await?;
    }

    Ok(())
}

async fn live_loop(
    tx: &broadcast::Sender<String>,
    state: &Arc<RwLock<ServerState>>,
    rng: &mut StdRng,
    interval_ms: u64,
    record_file: Option<PathBuf>,
    record_rounds: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut council = seed_council();
    let mut ledger_index = 0usize;
    let mut round = 1u32;
    let mut record_frames: Vec<ReplayFrame> = Vec::new();
    let mut record_ledger: Vec<LedgerEntry> = Vec::new();
    let record_limit = record_file.as_ref().map(|_| record_rounds as usize);
    let mut record_written = record_limit == Some(0);

    loop {
        let (metrics, events, members) = run_round(&mut council, rng, round, &mut ledger_index);
        broadcast_events(tx, state, &events);
        broadcast_metrics(tx, state, &metrics);
        broadcast_members(tx, state, &members);

        if let Some(limit) = record_limit {
            if !record_written {
                record_frames.push(ReplayFrame {
                    metrics: metrics.clone(),
                    events: events.clone(),
                    members: members.clone(),
                });
                record_ledger.extend(events.clone());
                if record_frames.len() >= limit {
                    if let Some(path) = &record_file {
                        let session = ReplaySession {
                            frames: record_frames.clone(),
                            ledger_history: record_ledger.clone(),
                        };
                        save_replay_file(path, &session)?;
                        record_written = true;
                    }
                }
            }
        }

        round += 1;
        tokio::time::sleep(Duration::from_millis(interval_ms)).await;
    }
}

async fn replay_loop(
    tx: &broadcast::Sender<String>,
    state: &Arc<RwLock<ServerState>>,
    frames: &[ReplayFrame],
    interval_ms: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let total = frames.len();

    loop {
        for (index, frame) in frames.iter().enumerate() {
            let payload = json!({
                "type": "replay",
                "frame": (index + 1) as u32,
                "total_frames": total as u32,
                "timestamp": chrono::Utc::now().timestamp_millis(),
            });
            let _ = tx.send(payload.to_string());

            broadcast_events(tx, state, &frame.events);
            broadcast_metrics(tx, state, &frame.metrics);
            broadcast_members(tx, state, &frame.members);

            tokio::time::sleep(Duration::from_millis(interval_ms)).await;
        }
    }
}

fn build_replay_session(rounds: u32, rng: &mut StdRng) -> ReplaySession {
    let mut council = seed_council();
    let mut ledger_index = 0usize;
    let mut frames = Vec::new();
    let mut ledger_history = Vec::new();

    for round in 1..=rounds {
        let (metrics, events, members) = run_round(&mut council, rng, round, &mut ledger_index);
        ledger_history.extend(events.clone());
        frames.push(ReplayFrame {
            metrics,
            events,
            members,
        });
    }

    ReplaySession {
        frames,
        ledger_history,
    }
}

fn run_round(
    council: &mut GovernanceCouncil,
    rng: &mut StdRng,
    round: u32,
    ledger_index: &mut usize,
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

    let events = ledger_entries(council, ledger_index);
    let members_snapshot = member_snapshots(council);
    (metrics, events, members_snapshot)
}

fn seed_council() -> GovernanceCouncil {
    let mut council = GovernanceCouncil::new(GovernanceConfig::default());
    let _ = council.add_member("Chair".to_string(), 1200);
    let _ = council.add_member("Guardians".to_string(), 1150);
    let _ = council.add_member("Treasury".to_string(), 1100);
    let _ = council.add_member("Protocol".to_string(), 1050);
    let _ = council.add_member("Risk".to_string(), 1000);
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
        snapshots.push(MemberSnapshot {
            member_id,
            name: name.to_string(),
            energy,
            damage,
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

fn broadcast_metrics(
    tx: &broadcast::Sender<String>,
    state: &Arc<RwLock<ServerState>>,
    metrics: &RoundMetrics,
) {
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

    let payload_string = payload.to_string();
    let _ = tx.send(payload_string);

    let metrics_clone = metrics.clone();
    let state = Arc::clone(state);
    tokio::spawn(async move {
        let mut state = state.write().await;
        state.latest_metrics = Some(metrics_clone);
    });
}

fn broadcast_events(
    tx: &broadcast::Sender<String>,
    state: &Arc<RwLock<ServerState>>,
    events: &[LedgerEntry],
) {
    for entry in events {
        let payload = json!({
            "type": "ledger_event",
            "message": entry.message,
            "severity": entry.severity,
            "timestamp": entry.timestamp_ms,
        });
        let payload_string = payload.to_string();
        let _ = tx.send(payload_string);

        let entry_clone = entry.clone();
        let state = Arc::clone(state);
        tokio::spawn(async move {
            let mut state = state.write().await;
            state.record_ledger(entry_clone);
        });
    }
}

fn broadcast_members(
    tx: &broadcast::Sender<String>,
    state: &Arc<RwLock<ServerState>>,
    members: &[MemberSnapshot],
) {
    let payload = json!({
        "type": "members",
        "members": members,
    });

    let payload_string = payload.to_string();
    let _ = tx.send(payload_string);

    let members_clone = members.to_vec();
    let state = Arc::clone(state);
    tokio::spawn(async move {
        let mut state = state.write().await;
        state.latest_members = members_clone;
    });
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

fn build_status_payload(settings: &Settings) -> String {
    json!({
        "type": "status",
        "mode": if settings.replay { "replay" } else { "live" },
        "interval_ms": settings.interval_ms,
        "timestamp": chrono::Utc::now().timestamp_millis(),
    })
    .to_string()
}

fn load_replay_file(path: &PathBuf) -> Result<ReplaySession, Box<dyn std::error::Error>> {
    let file = File::open(path)?;

    if let Ok(session) = serde_json::from_reader::<_, ReplaySession>(file) {
        return Ok(session);
    }

    let file = File::open(path)?;
    let frames: Vec<ReplayFrame> = serde_json::from_reader(file)?;
    Ok(ReplaySession {
        frames,
        ledger_history: Vec::new(),
    })
}

fn save_replay_file(
    path: &PathBuf,
    session: &ReplaySession,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, session)?;
    Ok(())
}

fn parse_args() -> Settings {
    let mut settings = Settings {
        replay: false,
        rounds: 20,
        interval_ms: 1200,
        seed: None,
        replay_file: None,
        record_file: None,
    };

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--replay" => settings.replay = true,
            "--rounds" => {
                if let Some(value) = args.next() {
                    settings.rounds = value.parse().unwrap_or(settings.rounds);
                }
            }
            "--interval-ms" => {
                if let Some(value) = args.next() {
                    settings.interval_ms = value.parse().unwrap_or(settings.interval_ms);
                }
            }
            "--seed" => {
                if let Some(value) = args.next() {
                    settings.seed = value.parse().ok();
                }
            }
            "--replay-file" => {
                if let Some(value) = args.next() {
                    settings.replay_file = Some(PathBuf::from(value));
                }
            }
            "--record-file" => {
                if let Some(value) = args.next() {
                    settings.record_file = Some(PathBuf::from(value));
                }
            }
            _ => {}
        }
    }

    settings
}
