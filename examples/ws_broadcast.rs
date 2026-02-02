// WebSocket broadcast server for real-time market and agent data
// Run: cargo run --example ws_broadcast --release
// Connect: websocat ws://127.0.0.1:9001

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use tokio::net::{TcpListener, TcpStream};
use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::Utc;
use futures_util::{SinkExt, StreamExt};

/// Market price event sent over WebSocket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketEvent {
    pub symbol: String,
    pub price: f64,
    pub timestamp: String,
    pub change_percent: f64,
}

/// Agent state event sent over WebSocket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentEvent {
    pub agent_id: String,
    pub capital: f64,
    pub trades_executed: u32,
    pub win_rate: f64,
    pub status: String, // "active", "deceased", "resting"
    pub timestamp: String,
}

/// Broadcast payload with multiple event types
#[derive(Debug, Serialize, Deserialize)]
pub struct BroadcastPayload {
    pub event_type: String, // "market" or "agent"
    pub market: Option<MarketEvent>,
    pub agent: Option<AgentEvent>,
    pub sequence: u64,
}

/// Shared connection tracker for broadcast management
#[derive(Clone)]
struct ServerState {
    connections: Arc<RwLock<HashMap<u64, tokio::sync::mpsc::Sender<Message>>>>,
    sequence: Arc<RwLock<u64>>,
}

impl ServerState {
    fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            sequence: Arc::new(RwLock::new(0)),
        }
    }

    async fn add_connection(&self, id: u64, tx: tokio::sync::mpsc::Sender<Message>) {
        let mut conns = self.connections.write().await;
        conns.insert(id, tx);
        println!("[ws] Client {} connected. Total: {}", id, conns.len());
    }

    async fn remove_connection(&self, id: u64) {
        let mut conns = self.connections.write().await;
        conns.remove(&id);
        println!("[ws] Client {} disconnected. Total: {}", id, conns.len());
    }

    async fn broadcast(&self, payload: BroadcastPayload) {
        let conns = self.connections.read().await;
        let msg = Message::Text(serde_json::to_string(&payload).unwrap_or_default());

        let mut dead_connections = Vec::new();
        for (id, tx) in conns.iter() {
            if tx.send(msg.clone()).await.is_err() {
                dead_connections.push(*id);
            }
        }
        drop(conns);

        // Clean up dead connections
        for id in dead_connections {
            self.remove_connection(id).await;
        }
    }

    async fn next_sequence(&self) -> u64 {
        let mut seq = self.sequence.write().await;
        *seq += 1;
        *seq
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = ServerState::new();
    let listener = TcpListener::bind("127.0.0.1:9001").await?;
    println!("📡 WebSocket server listening on ws://127.0.0.1:9001");
    println!("   (broadcast market ticks and agent updates to connected clients)");
    println!();

    // Spawn market ticker task
    let state_ticker = state.clone();
    tokio::spawn(async move {
        market_ticker(&state_ticker).await;
    });

    // Spawn agent state updater task
    let state_agent = state.clone();
    tokio::spawn(async move {
        agent_updater(&state_agent).await;
    });

    // Accept incoming connections
    let mut next_client_id = 0u64;
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("[tcp] New connection from {}", addr);

        let state_clone = state.clone();
        let client_id = next_client_id;
        next_client_id += 1;

        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, client_id, state_clone).await {
                eprintln!("[ws] Error handling client {}: {}", client_id, e);
            }
        });
    }
}

/// Handle individual WebSocket client connection
async fn handle_client(
    stream: TcpStream,
    client_id: u64,
    state: ServerState,
) -> Result<(), Box<dyn std::error::Error>> {
    let ws_stream = accept_async(stream).await?;
    let (sender, receiver) = ws_stream.split();
    let sender = Arc::new(tokio::sync::Mutex::new(sender));

    let (tx, mut rx) = tokio::sync::mpsc::channel::<Message>(100);
    state.add_connection(client_id, tx).await;

    // Send welcome message
    let welcome = json!({
        "type": "welcome",
        "client_id": client_id,
        "timestamp": Utc::now().to_rfc3339(),
        "message": "Connected to Lineage WebSocket broadcast server"
    });
    {
        let mut s = sender.lock().await;
        s.send(Message::Text(welcome.to_string())).await?;
    }

    // Relay outgoing messages to client
    let sender_clone = Arc::clone(&sender);
    let mut relay_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let mut s = sender_clone.lock().await;
            if s.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Listen for incoming messages (heartbeat/ping handling)
    let mut receiver = receiver;
    loop {
        tokio::select! {
            msg = receiver.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        // Handle client messages (e.g., subscription filters)
                        if text.contains("ping") || text.contains("heartbeat") {
                            println!("[ws] Heartbeat from client {}", client_id);
                        }
                    }
                    Some(Ok(Message::Ping(data))) => {
                        // Respond to ping with pong
                        let mut s = sender.lock().await;
                        let _ = s.send(Message::Pong(data)).await;
                    }
                    Some(Ok(Message::Pong(_))) => {
                        // Ignore pong messages
                    }
                    Some(Ok(Message::Close(_))) => {
                        println!("[ws] Client {} sent close frame", client_id);
                        break;
                    }
                    Some(Ok(Message::Binary(_))) => {
                        // Ignore binary messages
                    }
                    Some(Ok(Message::Frame(_))) => {
                        // Ignore frame messages
                    }
                    Some(Err(e)) => {
                        eprintln!("[ws] Error from client {}: {}", client_id, e);
                        break;
                    }
                    None => break,
                }
            }
            _ = &mut relay_task => break,
        }
    }

    state.remove_connection(client_id).await;
    relay_task.abort();
    Ok(())
}

/// Simulated market ticker - broadcasts BTC/ETH prices every 5 seconds
async fn market_ticker(state: &ServerState) {
    let symbols = vec!["BTC-USD", "ETH-USD"];
    let mut tick = 0u32;

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        for symbol in &symbols {
            // Simulate price movement
            let base = if *symbol == "BTC-USD" { 45_000.0 } else { 2_500.0 };
            let noise = (tick as f64).sin() * 200.0;
            let price = base + noise + (rand::random::<f64>() * 100.0);
            let change_percent = (noise / base) * 100.0;

            let event = MarketEvent {
                symbol: symbol.to_string(),
                price,
                timestamp: Utc::now().to_rfc3339(),
                change_percent,
            };

            let payload = BroadcastPayload {
                event_type: "market".to_string(),
                market: Some(event),
                agent: None,
                sequence: state.next_sequence().await,
            };

            state.broadcast(payload).await;
        }

        tick = tick.wrapping_add(1);
    }
}

/// Simulated agent updater - broadcasts agent state every 10 seconds
async fn agent_updater(state: &ServerState) {
    let agent_ids = vec!["Momentum", "Conservative", "Balanced"];
    let mut round = 0u32;

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

        for agent_id in &agent_ids {
            // Simulate agent state changes
            let base_capital = 100_000.0;
            let variation = (round as f64).sin() * 5_000.0;
            let capital = base_capital + variation + (rand::random::<f64>() * 2_000.0);
            let trades = 10 + (round as u32 * 2) % 50;
            let win_rate = 0.45 + ((round as f64).cos() * 0.15).abs();

            let status = if capital < 50_000.0 {
                "resting"
            } else if capital > 120_000.0 {
                "active"
            } else {
                "active"
            };

            let event = AgentEvent {
                agent_id: agent_id.to_string(),
                capital,
                trades_executed: trades,
                win_rate,
                status: status.to_string(),
                timestamp: Utc::now().to_rfc3339(),
            };

            let payload = BroadcastPayload {
                event_type: "agent".to_string(),
                market: None,
                agent: Some(event),
                sequence: state.next_sequence().await,
            };

            state.broadcast(payload).await;
        }

        round = round.wrapping_add(1);
    }
}
