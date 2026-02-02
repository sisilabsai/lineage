// Prometheus metrics HTTP server for monitoring Lineage system
// Run: cargo run --example metrics_server --release
// Scrape: curl http://127.0.0.1:9184/metrics

use prometheus::{
    Counter, CounterVec, Gauge, GaugeVec, Histogram, HistogramVec, Registry, TextEncoder,
};
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use std::sync::Arc;
use std::net::SocketAddr;
use std::time::SystemTime;

/// Metrics collector for Lineage system
pub struct LineageMetrics {
    pub registry: Registry,

    // Market data metrics
    pub cache_hit_rate: Gauge,
    pub request_success_rate: Gauge,
    pub api_request_duration: Histogram,
    pub api_errors_total: Counter,

    // Agent metrics
    pub agent_capital: GaugeVec,
    pub agent_trades_total: CounterVec,
    pub agent_win_rate: GaugeVec,
    pub agent_scars_total: CounterVec,

    // Arena metrics
    pub arena_rounds_total: Counter,
    pub arena_agent_wins_total: CounterVec,
    pub arena_agent_final_capital: GaugeVec,

    // System metrics
    pub active_connections: Gauge,
    pub messages_broadcast_total: Counter,
    pub uptime_seconds: Gauge,
}

impl LineageMetrics {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let registry = Registry::new();

        // Market data metrics
        let cache_hit_rate = Gauge::new("lineage_cache_hit_rate", "Cache hit rate percentage")?;
        let request_success_rate =
            Gauge::new("lineage_request_success_rate", "Request success rate percentage")?;
        let api_request_duration = Histogram::with_opts(
            prometheus::HistogramOpts::new(
                "lineage_api_request_duration_seconds",
                "API request duration in seconds",
            ),
        )?;
        let api_errors_total = Counter::new("lineage_api_errors_total", "Total API errors")?;

        // Agent metrics
        let agent_capital = GaugeVec::new(
            prometheus::Opts::new("lineage_agent_capital", "Agent current capital"),
            &["agent"],
        )?;
        let agent_trades_total = CounterVec::new(
            prometheus::Opts::new("lineage_agent_trades_total", "Total agent trades"),
            &["agent"],
        )?;
        let agent_win_rate = GaugeVec::new(
            prometheus::Opts::new("lineage_agent_win_rate", "Agent win rate [0-1]"),
            &["agent"],
        )?;
        let agent_scars_total = CounterVec::new(
            prometheus::Opts::new("lineage_agent_scars_total", "Total agent scars/losses"),
            &["agent"],
        )?;

        // Arena metrics
        let arena_rounds_total = Counter::new("lineage_arena_rounds_total", "Total arena rounds")?;
        let arena_agent_wins_total = CounterVec::new(
            prometheus::Opts::new("lineage_arena_agent_wins_total", "Total agent arena wins"),
            &["agent"],
        )?;
        let arena_agent_final_capital = GaugeVec::new(
            prometheus::Opts::new(
                "lineage_arena_agent_final_capital",
                "Agent capital after arena round",
            ),
            &["agent"],
        )?;

        // System metrics
        let active_connections =
            Gauge::new("lineage_active_connections", "Active WebSocket connections")?;
        let messages_broadcast_total =
            Counter::new("lineage_messages_broadcast_total", "Total messages broadcast")?;
        let uptime_seconds = Gauge::new("lineage_uptime_seconds", "Server uptime in seconds")?;

        // Register all metrics
        registry.register(Box::new(cache_hit_rate.clone()))?;
        registry.register(Box::new(request_success_rate.clone()))?;
        registry.register(Box::new(api_request_duration.clone()))?;
        registry.register(Box::new(api_errors_total.clone()))?;
        registry.register(Box::new(agent_capital.clone()))?;
        registry.register(Box::new(agent_trades_total.clone()))?;
        registry.register(Box::new(agent_win_rate.clone()))?;
        registry.register(Box::new(agent_scars_total.clone()))?;
        registry.register(Box::new(arena_rounds_total.clone()))?;
        registry.register(Box::new(arena_agent_wins_total.clone()))?;
        registry.register(Box::new(arena_agent_final_capital.clone()))?;
        registry.register(Box::new(active_connections.clone()))?;
        registry.register(Box::new(messages_broadcast_total.clone()))?;
        registry.register(Box::new(uptime_seconds.clone()))?;

        Ok(LineageMetrics {
            registry,
            cache_hit_rate,
            request_success_rate,
            api_request_duration,
            api_errors_total,
            agent_capital,
            agent_trades_total,
            agent_win_rate,
            agent_scars_total,
            arena_rounds_total,
            arena_agent_wins_total,
            arena_agent_final_capital,
            active_connections,
            messages_broadcast_total,
            uptime_seconds,
        })
    }

    pub fn gather(&self) -> Result<String, Box<dyn std::error::Error>> {
        let encoder = TextEncoder::new();
        let metrics = self.registry.gather();
        Ok(encoder.encode_to_string(&metrics)?)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize metrics
    let metrics = Arc::new(LineageMetrics::new()?);
    let start_time = SystemTime::now();

    println!("📊 Prometheus Metrics Server");
    println!("   Listening on http://127.0.0.1:9184");
    println!("   Scrape endpoint: http://127.0.0.1:9184/metrics");
    println!();

    // Simulate some initial metrics for demo
    {
        let m = &metrics;
        m.cache_hit_rate.set(96.2);
        m.request_success_rate.set(99.8);
        m.active_connections.set(3);

        // Set initial agent metrics
        m.agent_capital.with_label_values(&["Momentum"]).set(128_960.0);
        m.agent_capital.with_label_values(&["Conservative"]).set(113_666.0);
        m.agent_capital.with_label_values(&["Balanced"]).set(108_135.0);

        m.agent_trades_total.with_label_values(&["Momentum"]).inc_by(35);
        m.agent_trades_total.with_label_values(&["Conservative"]).inc_by(12);
        m.agent_trades_total.with_label_values(&["Balanced"]).inc_by(28);

        m.agent_win_rate.with_label_values(&["Momentum"]).set(0.58);
        m.agent_win_rate.with_label_values(&["Conservative"]).set(0.52);
        m.agent_win_rate.with_label_values(&["Balanced"]).set(0.62);

        m.agent_scars_total.with_label_values(&["Momentum"]).inc_by(8);
        m.agent_scars_total.with_label_values(&["Conservative"]).inc_by(6);
        m.agent_scars_total.with_label_values(&["Balanced"]).inc_by(3);

        m.arena_agent_wins_total.with_label_values(&["Balanced"]).inc_by(12);
        m.arena_agent_wins_total.with_label_values(&["Momentum"]).inc_by(8);
    }

    // Spawn background task to update metrics
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
            m.cache_hit_rate.set(cache_variation);

            let success_variation = 99.5 + ((tick as f64).cos() * 0.4);
            m.request_success_rate.set(success_variation);

            // Update agent capital (simulated)
            let momentum_cap = 128_960.0 + ((tick as f64).sin() * 5_000.0);
            m.agent_capital.with_label_values(&["Momentum"]).set(momentum_cap.max(10_000.0));

            let conservative_cap = 113_666.0 + ((tick as f64).cos() * 3_000.0);
            m.agent_capital
                .with_label_values(&["Conservative"])
                .set(conservative_cap.max(10_000.0));

            let balanced_cap = 108_135.0 + ((tick as f64).sin() * 2_000.0);
            m.agent_capital
                .with_label_values(&["Balanced"])
                .set(balanced_cap.max(10_000.0));

            // Update uptime
            let uptime = start_time.elapsed().unwrap_or_default().as_secs();
            m.uptime_seconds.set(uptime as f64);

            // Increment message counter
            m.messages_broadcast_total.inc_by(5);
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
                            let body = metrics
                                .gather()
                                .unwrap_or_else(|_| "Failed to gather metrics".to_string());
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
