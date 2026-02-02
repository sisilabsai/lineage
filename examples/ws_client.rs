// WebSocket client for consuming real-time market and agent updates
// Run: cargo run --example ws_client --release
// Connect to: ws://127.0.0.1:9001 (ensure ws_broadcast.rs is running)

use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use serde_json::{json, Value};
use std::time::Duration;
use futures_util::{StreamExt, SinkExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔌 WebSocket Client - Connecting to ws://127.0.0.1:9001");
    println!();

    // Connect to WebSocket server
    let url = "ws://127.0.0.1:9001";
    let (ws_stream, _response) = connect_async(url).await?;
    println!("✅ Connected to {}", url);
    println!();

    let (mut sender, mut receiver) = ws_stream.split();

    // Send heartbeat periodically
    let heartbeat_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(30));
        loop {
            interval.tick().await;
            let ping = json!({ "type": "heartbeat" });
            let _ = sender
                .send(Message::Text(ping.to_string()))
                .await;
        }
    });

    // Print header
    println!(
        "{:<25} {:<12} {:<15} {:<20}",
        "EVENT", "SYMBOL/AGENT", "VALUE", "TIMESTAMP"
    );
    println!("{}", "─".repeat(75));

    // Receive and process messages
    let mut message_count = 0u32;
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                message_count += 1;

                // Try to parse as JSON
                if let Ok(json) = serde_json::from_str::<Value>(&text) {
                    if let Some(event_type) = json.get("type").and_then(|v| v.as_str()) {
                        // Welcome message
                        if event_type == "welcome" {
                            println!();
                            println!("📨 Welcome: {}", json.get("message").unwrap_or(&Value::Null));
                            println!("   Client ID: {}", json.get("client_id").unwrap_or(&Value::Null));
                            println!();
                            continue;
                        }
                    }

                    // Market event
                    if let Some(market) = json.get("market").and_then(|v| v.as_object()) {
                        let symbol = market
                            .get("symbol")
                            .and_then(|v| v.as_str())
                            .unwrap_or("?");
                        let price = market
                            .get("price")
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0);
                        let change = market
                            .get("change_percent")
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0);
                        let ts = market
                            .get("timestamp")
                            .and_then(|v| v.as_str())
                            .unwrap_or("?");

                        let sign = if change >= 0.0 { "📈" } else { "📉" };
                        println!(
                            "{:<25} {:<12} ${:<14.2} {}",
                            format!("{} MARKET", sign),
                            symbol,
                            price,
                            format_timestamp(ts)
                        );
                    }

                    // Agent event
                    if let Some(agent) = json.get("agent").and_then(|v| v.as_object()) {
                        let agent_id = agent
                            .get("agent_id")
                            .and_then(|v| v.as_str())
                            .unwrap_or("?");
                        let capital = agent
                            .get("capital")
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0);
                        let win_rate = agent
                            .get("win_rate")
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0);
                        let status = agent
                            .get("status")
                            .and_then(|v| v.as_str())
                            .unwrap_or("?");
                        let ts = agent
                            .get("timestamp")
                            .and_then(|v| v.as_str())
                            .unwrap_or("?");

                        let status_icon = match status {
                            "active" => "🟢",
                            "deceased" => "💀",
                            _ => "⏸️",
                        };

                        println!(
                            "{:<25} {:<12} ${:<14.0} {} ({:.1}%)",
                            format!("{} AGENT", status_icon),
                            agent_id,
                            capital,
                            format_timestamp(ts),
                            win_rate * 100.0
                        );
                    }
                }
            }
            Ok(Message::Close(_)) => {
                println!("\n❌ Server closed connection");
                break;
            }
            Err(e) => {
                eprintln!("\n❌ WebSocket error: {}", e);
                break;
            }
            _ => {}
        }

        // Exit after 100 messages for demo
        if message_count >= 100 {
            println!("\n✅ Demo complete (100 messages received)");
            break;
        }
    }

    heartbeat_task.abort();
    Ok(())
}

fn format_timestamp(ts: &str) -> String {
    // Extract time portion from RFC3339 timestamp
    ts.split('T')
        .nth(1)
        .unwrap_or(ts)
        .split('+')
        .next()
        .unwrap_or(ts)
        .to_string()
}
