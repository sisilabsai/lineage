// Prometheus metrics HTTP server for monitoring Lineage system
// Run: cargo run --example metrics_server_v2 --release
// Scrape: curl http://127.0.0.1:9184/metrics

use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use std::sync::Arc;
use std::net::SocketAddr;
use std::time::SystemTime;
use std::sync::atomic::{AtomicU64, AtomicU32, Ordering};
use std::sync::Mutex;
use std::collections::HashMap;

/// Simple metrics collector for Lineage system
pub struct LineageMetrics {
    // Market data metrics
    pub cache_hit_rate: Arc<Mutex<f64>>,
    pub request_success_rate: Arc<Mutex<f64>>,
    pub api_errors_total: Arc<AtomicU64>,

    // Agent metrics (stored as label -> value map)
    pub agent_capital: Arc<Mutex<HashMap<String, f64>>>,
    pub agent_trades_total: Arc<Mutex<HashMap<String, u64>>>,
    pub agent_win_rate: Arc<Mutex<HashMap<String, f64>>>,
    pub agent_scars_total: Arc<Mutex<HashMap<String, u64>>>,

    // Arena metrics
    pub arena_rounds_total: Arc<AtomicU64>,
    pub arena_agent_wins: Arc<Mutex<HashMap<String, u64>>>,
    pub arena_agent_capital: Arc<Mutex<HashMap<String, f64>>>,

    // System metrics
    pub active_connections: Arc<AtomicU32>,
    pub messages_broadcast_total: Arc<AtomicU64>,
    pub uptime_start: SystemTime,
}

impl LineageMetrics {
    pub fn new() -> Self {
        LineageMetrics {
            cache_hit_rate: Arc::new(Mutex::new(96.2)),
            request_success_rate: Arc::new(Mutex::new(99.8)),
            api_errors_total: Arc::new(AtomicU64::new(0)),
            agent_capital: Arc::new(Mutex::new(HashMap::new())),
            agent_trades_total: Arc::new(Mutex::new(HashMap::new())),
            agent_win_rate: Arc::new(Mutex::new(HashMap::new())),
            agent_scars_total: Arc::new(Mutex::new(HashMap::new())),
            arena_rounds_total: Arc::new(AtomicU64::new(0)),
            arena_agent_wins: Arc::new(Mutex::new(HashMap::new())),
            arena_agent_capital: Arc::new(Mutex::new(HashMap::new())),
            active_connections: Arc::new(AtomicU32::new(0)),
            messages_broadcast_total: Arc::new(AtomicU64::new(0)),
            uptime_start: SystemTime::now(),
        }
    }

    pub fn gather(&self) -> String {
        let mut output = String::new();

        // Gauge: Cache hit rate
        let cache_rate = self.cache_hit_rate.lock().unwrap();
        output.push_str(&format!(
            "# HELP lineage_cache_hit_rate Cache hit rate percentage\n"
        ));
        output.push_str(&format!("# TYPE lineage_cache_hit_rate gauge\n"));
        output.push_str(&format!("lineage_cache_hit_rate {:.2}\n\n", cache_rate));
        drop(cache_rate);

        // Gauge: Request success rate
        let success_rate = self.request_success_rate.lock().unwrap();
        output.push_str(&format!(
            "# HELP lineage_request_success_rate Request success rate percentage\n"
        ));
        output.push_str(&format!("# TYPE lineage_request_success_rate gauge\n"));
        output.push_str(&format!("lineage_request_success_rate {:.2}\n\n", success_rate));
        drop(success_rate);

        // Counter: API errors
        let errors = self.api_errors_total.load(Ordering::Relaxed);
        output.push_str(&format!(
            "# HELP lineage_api_errors_total Total API errors\n"
        ));
        output.push_str(&format!("# TYPE lineage_api_errors_total counter\n"));
        output.push_str(&format!("lineage_api_errors_total {}\n\n", errors));

        // Gauge: Agent capital (per agent)
        let agent_capital = self.agent_capital.lock().unwrap();
        if !agent_capital.is_empty() {
            output.push_str(
                "# HELP lineage_agent_capital Agent current capital in USD\n",
            );
            output.push_str("# TYPE lineage_agent_capital gauge\n");
            for (agent, capital) in agent_capital.iter() {
                output.push_str(&format!(
                    "lineage_agent_capital{{agent=\"{}\"}} {:.2}\n",
                    agent, capital
                ));
            }
            output.push('\n');
        }
        drop(agent_capital);

        // Counter: Agent trades (per agent)
        let agent_trades = self.agent_trades_total.lock().unwrap();
        if !agent_trades.is_empty() {
            output.push_str(
                "# HELP lineage_agent_trades_total Total agent trades executed\n",
            );
            output.push_str("# TYPE lineage_agent_trades_total counter\n");
            for (agent, trades) in agent_trades.iter() {
                output.push_str(&format!(
                    "lineage_agent_trades_total{{agent=\"{}\"}} {}\n",
                    agent, trades
                ));
            }
            output.push('\n');
        }
        drop(agent_trades);

        // Gauge: Agent win rate (per agent)
        let agent_wins = self.agent_win_rate.lock().unwrap();
        if !agent_wins.is_empty() {
            output.push_str(
                "# HELP lineage_agent_win_rate Agent win rate as fraction [0-1]\n",
            );
            output.push_str("# TYPE lineage_agent_win_rate gauge\n");
            for (agent, win_rate) in agent_wins.iter() {
                output.push_str(&format!(
                    "lineage_agent_win_rate{{agent=\"{}\"}} {:.4}\n",
                    agent, win_rate
                ));
            }
            output.push('\n');
        }
        drop(agent_wins);

        // Counter: Agent scars (per agent)
        let agent_scars = self.agent_scars_total.lock().unwrap();
        if !agent_scars.is_empty() {
            output.push_str(
                "# HELP lineage_agent_scars_total Total agent losses/scars\n",
            );
            output.push_str("# TYPE lineage_agent_scars_total counter\n");
            for (agent, scars) in agent_scars.iter() {
                output.push_str(&format!(
                    "lineage_agent_scars_total{{agent=\"{}\"}} {}\n",
                    agent, scars
                ));
            }
            output.push('\n');
        }
        drop(agent_scars);

        // Counter: Arena rounds
        let rounds = self.arena_rounds_total.load(Ordering::Relaxed);
        output.push_str(
            "# HELP lineage_arena_rounds_total Total number of arena competition rounds\n",
        );
        output.push_str("# TYPE lineage_arena_rounds_total counter\n");
        output.push_str(&format!("lineage_arena_rounds_total {}\n\n", rounds));

        // Counter: Arena wins (per agent)
        let arena_wins = self.arena_agent_wins.lock().unwrap();
        if !arena_wins.is_empty() {
            output.push_str(
                "# HELP lineage_arena_agent_wins_total Total agent arena round wins\n",
            );
            output.push_str("# TYPE lineage_arena_agent_wins_total counter\n");
            for (agent, wins) in arena_wins.iter() {
                output.push_str(&format!(
                    "lineage_arena_agent_wins_total{{agent=\"{}\"}} {}\n",
                    agent, wins
                ));
            }
            output.push('\n');
        }
        drop(arena_wins);

        // Gauge: Arena final capital (per agent)
        let final_capital = self.arena_agent_capital.lock().unwrap();
        if !final_capital.is_empty() {
            output.push_str(
                "# HELP lineage_arena_agent_final_capital Agent capital after arena round\n",
            );
            output.push_str("# TYPE lineage_arena_agent_final_capital gauge\n");
            for (agent, capital) in final_capital.iter() {
                output.push_str(&format!(
                    "lineage_arena_agent_final_capital{{agent=\"{}\"}} {:.2}\n",
                    agent, capital
                ));
            }
            output.push('\n');
        }
        drop(final_capital);

        // Gauge: Active connections
        let conns = self.active_connections.load(Ordering::Relaxed);
        output.push_str(
            "# HELP lineage_active_connections Active WebSocket connections\n",
        );
        output.push_str("# TYPE lineage_active_connections gauge\n");
        output.push_str(&format!("lineage_active_connections {}\n\n", conns));

        // Counter: Messages broadcast
        let messages = self.messages_broadcast_total.load(Ordering::Relaxed);
        output.push_str(
            "# HELP lineage_messages_broadcast_total Total messages broadcast to clients\n",
        );
        output.push_str("# TYPE lineage_messages_broadcast_total counter\n");
        output.push_str(&format!("lineage_messages_broadcast_total {}\n\n", messages));

        // Gauge: Uptime
        let uptime = self
            .uptime_start
            .elapsed()
            .unwrap_or_default()
            .as_secs();
        output.push_str("# HELP lineage_uptime_seconds Server uptime in seconds\n");
        output.push_str("# TYPE lineage_uptime_seconds gauge\n");
        output.push_str(&format!("lineage_uptime_seconds {}\n", uptime));

        output
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize metrics
    let metrics = Arc::new(LineageMetrics::new());

    println!("📊 Prometheus Metrics Server");
    println!("   Listening on http://127.0.0.1:9184");
    println!("   Scrape endpoint: http://127.0.0.1:9184/metrics");
    println!();

    // Populate initial sample data
    {
        let mut agent_capital = metrics.agent_capital.lock().unwrap();
        agent_capital.insert("Momentum".to_string(), 128_960.0);
        agent_capital.insert("Conservative".to_string(), 113_666.0);
        agent_capital.insert("Balanced".to_string(), 108_135.0);
    }

    {
        let mut agent_trades = metrics.agent_trades_total.lock().unwrap();
        agent_trades.insert("Momentum".to_string(), 35);
        agent_trades.insert("Conservative".to_string(), 12);
        agent_trades.insert("Balanced".to_string(), 28);
    }

    {
        let mut agent_wins = metrics.agent_win_rate.lock().unwrap();
        agent_wins.insert("Momentum".to_string(), 0.58);
        agent_wins.insert("Conservative".to_string(), 0.52);
        agent_wins.insert("Balanced".to_string(), 0.62);
    }

    {
        let mut agent_scars = metrics.agent_scars_total.lock().unwrap();
        agent_scars.insert("Momentum".to_string(), 8);
        agent_scars.insert("Conservative".to_string(), 6);
        agent_scars.insert("Balanced".to_string(), 3);
    }

    {
        let mut arena_wins = metrics.arena_agent_wins.lock().unwrap();
        arena_wins.insert("Balanced".to_string(), 12);
        arena_wins.insert("Momentum".to_string(), 8);
    }

    {
        let mut arena_capital = metrics.arena_agent_capital.lock().unwrap();
        arena_capital.insert("Balanced".to_string(), 128_960.0);
        arena_capital.insert("Momentum".to_string(), 120_500.0);
        arena_capital.insert("Conservative".to_string(), 113_200.0);
    }

    metrics.active_connections.store(3, Ordering::Relaxed);
    metrics.arena_rounds_total.store(20, Ordering::Relaxed);

    // Spawn background task to update metrics dynamically
    let metrics_clone = Arc::clone(&metrics);
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
        let mut tick = 0u32;

        loop {
            interval.tick().await;
            tick = tick.wrapping_add(1);

            let m = &metrics_clone;

            // Simulate dynamic metrics
            let cache_variation = 96.0 + ((tick as f64).sin() * 2.0);
            *m.cache_hit_rate.lock().unwrap() = cache_variation;

            let success_variation = 99.5 + ((tick as f64).cos() * 0.4);
            *m.request_success_rate.lock().unwrap() = success_variation;

            // Update agent capital (simulated)
            {
                let mut capital = m.agent_capital.lock().unwrap();
                let momentum_cap = 128_960.0 + ((tick as f64).sin() * 5_000.0);
                capital.insert("Momentum".to_string(), momentum_cap.max(10_000.0));

                let conservative_cap = 113_666.0 + ((tick as f64).cos() * 3_000.0);
                capital.insert("Conservative".to_string(), conservative_cap.max(10_000.0));

                let balanced_cap = 108_135.0 + ((tick as f64).sin() * 2_000.0);
                capital.insert("Balanced".to_string(), balanced_cap.max(10_000.0));
            }

            // Increment message counter
            m.messages_broadcast_total
                .fetch_add(5, Ordering::Relaxed);
        }
    });

    // HTTP server setup
    let addr = SocketAddr::from(([127, 0, 0, 1], 9184));
    let make_svc = make_service_fn(|_conn| {
        let metrics = Arc::clone(&metrics);
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req: Request<Body>| {
                let metrics = Arc::clone(&metrics);
                async move {
                    match req.uri().path() {
                        "/metrics" => {
                            let body = metrics.gather();
                            Ok::<_, hyper::Error>(
                                Response::builder()
                                    .header("Content-Type", "text/plain; charset=utf-8")
                                    .body(Body::from(body))
                                    .unwrap(),
                            )
                        }
                        "/health" => {
                            let body = r#"{"status":"ok","service":"lineage-metrics"}"#;
                            Ok::<_, hyper::Error>(
                                Response::builder()
                                    .header("Content-Type", "application/json")
                                    .body(Body::from(body))
                                    .unwrap(),
                            )
                        }
                        _ => {
                            let body = r#"
Lineage Prometheus Metrics Server
==================================

Available endpoints:
  GET /metrics     - Prometheus metrics (text format)
  GET /health      - Health check (JSON)

Example queries:
  curl http://127.0.0.1:9184/metrics
  curl http://127.0.0.1:9184/health

Configure Prometheus to scrape:
  scrape_configs:
    - job_name: 'lineage'
      static_configs:
        - targets: ['127.0.0.1:9184']
      metrics_path: '/metrics'
      scrape_interval: 15s
"#;
                            Ok::<_, hyper::Error>(
                                Response::builder()
                                    .header("Content-Type", "text/plain")
                                    .body(Body::from(body))
                                    .unwrap(),
                            )
                        }
                    }
                }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);
    println!("✅ Server ready");
    println!();

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }

    Ok(())
}
