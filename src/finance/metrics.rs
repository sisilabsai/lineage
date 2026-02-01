//! Market Data Metrics & Monitoring
//!
//! Comprehensive metrics collection for market data operations:
//! - API call tracking (total, rate-limited, errors)
//! - Cache performance (hits, misses, hit rate)
//! - Response latency (min, max, average)
//! - Error categorization and frequency
//! - Circuit breaker state tracking

use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Market metrics collector
#[derive(Clone)]
pub struct MetricsCollector {
    state: Arc<Mutex<MetricsState>>,
}

/// Internal metrics state
struct MetricsState {
    // API call metrics
    total_api_calls: u64,
    successful_calls: u64,
    failed_calls: u64,
    rate_limited_calls: u64,
    
    // Cache metrics
    cache_hits: u64,
    cache_misses: u64,
    
    // Latency metrics (milliseconds)
    response_latencies: Vec<u64>,
    min_latency_ms: u64,
    max_latency_ms: u64,
    
    // Error categorization
    error_counts: HashMap<String, u64>,
    
    // Circuit breaker metrics
    circuit_breaker_trips: u64,
    circuit_breaker_recoveries: u64,
    
    // Timestamps
    created_at: Instant,
    last_reset: Instant,
}

/// Serializable metrics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    pub total_api_calls: u64,
    pub successful_calls: u64,
    pub failed_calls: u64,
    pub rate_limited_calls: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub cache_hit_rate: f64,
    pub avg_latency_ms: f64,
    pub min_latency_ms: u64,
    pub max_latency_ms: u64,
    pub error_counts: HashMap<String, u64>,
    pub circuit_breaker_trips: u64,
    pub circuit_breaker_recoveries: u64,
    pub uptime_secs: u64,
}

impl MetricsCollector {
    /// Create new metrics collector
    pub fn new() -> Self {
        MetricsCollector {
            state: Arc::new(Mutex::new(MetricsState {
                total_api_calls: 0,
                successful_calls: 0,
                failed_calls: 0,
                rate_limited_calls: 0,
                cache_hits: 0,
                cache_misses: 0,
                response_latencies: Vec::new(),
                min_latency_ms: u64::MAX,
                max_latency_ms: 0,
                error_counts: HashMap::new(),
                circuit_breaker_trips: 0,
                circuit_breaker_recoveries: 0,
                created_at: Instant::now(),
                last_reset: Instant::now(),
            })),
        }
    }
    
    /// Record successful API call
    pub fn record_api_success(&self, latency_ms: u64) {
        if let Ok(mut state) = self.state.lock() {
            state.total_api_calls += 1;
            state.successful_calls += 1;
            state.response_latencies.push(latency_ms);
            state.min_latency_ms = state.min_latency_ms.min(latency_ms);
            state.max_latency_ms = state.max_latency_ms.max(latency_ms);
        }
    }
    
    /// Record failed API call
    pub fn record_api_failure(&self, error_type: &str, latency_ms: u64) {
        if let Ok(mut state) = self.state.lock() {
            state.total_api_calls += 1;
            state.failed_calls += 1;
            state.response_latencies.push(latency_ms);
            state.min_latency_ms = state.min_latency_ms.min(latency_ms);
            state.max_latency_ms = state.max_latency_ms.max(latency_ms);
            
            *state.error_counts.entry(error_type.to_string()).or_insert(0) += 1;
        }
    }
    
    /// Record rate limited call
    pub fn record_rate_limited(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.rate_limited_calls += 1;
        }
    }
    
    /// Record cache hit
    pub fn record_cache_hit(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.cache_hits += 1;
        }
    }
    
    /// Record cache miss
    pub fn record_cache_miss(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.cache_misses += 1;
        }
    }
    
    /// Record circuit breaker trip
    pub fn record_circuit_break(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.circuit_breaker_trips += 1;
        }
    }
    
    /// Record circuit breaker recovery
    pub fn record_circuit_recovery(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.circuit_breaker_recoveries += 1;
        }
    }
    
    /// Get current metrics snapshot
    pub fn snapshot(&self) -> MetricsSnapshot {
        if let Ok(state) = self.state.lock() {
            let total_cache = state.cache_hits + state.cache_misses;
            let cache_hit_rate = if total_cache > 0 {
                state.cache_hits as f64 / total_cache as f64
            } else {
                0.0
            };
            
            let avg_latency_ms = if !state.response_latencies.is_empty() {
                state.response_latencies.iter().sum::<u64>() as f64
                    / state.response_latencies.len() as f64
            } else {
                0.0
            };
            
            let uptime_secs = state.created_at.elapsed().as_secs();
            
            MetricsSnapshot {
                total_api_calls: state.total_api_calls,
                successful_calls: state.successful_calls,
                failed_calls: state.failed_calls,
                rate_limited_calls: state.rate_limited_calls,
                cache_hits: state.cache_hits,
                cache_misses: state.cache_misses,
                cache_hit_rate,
                avg_latency_ms,
                min_latency_ms: if state.min_latency_ms == u64::MAX {
                    0
                } else {
                    state.min_latency_ms
                },
                max_latency_ms: state.max_latency_ms,
                error_counts: state.error_counts.clone(),
                circuit_breaker_trips: state.circuit_breaker_trips,
                circuit_breaker_recoveries: state.circuit_breaker_recoveries,
                uptime_secs,
            }
        } else {
            MetricsSnapshot::default()
        }
    }
    
    /// Reset all metrics
    pub fn reset(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.total_api_calls = 0;
            state.successful_calls = 0;
            state.failed_calls = 0;
            state.rate_limited_calls = 0;
            state.cache_hits = 0;
            state.cache_misses = 0;
            state.response_latencies.clear();
            state.min_latency_ms = u64::MAX;
            state.max_latency_ms = 0;
            state.error_counts.clear();
            state.circuit_breaker_trips = 0;
            state.circuit_breaker_recoveries = 0;
            state.last_reset = Instant::now();
        }
    }
    
    /// Print metrics report to console
    pub fn print_report(&self) {
        let snapshot = self.snapshot();
        println!("\n╔════════════════════════════════════════════════════════════╗");
        println!("║           MARKET DATA METRICS REPORT                       ║");
        println!("╚════════════════════════════════════════════════════════════╝");
        
        println!("\n📊 API Call Metrics:");
        println!("  Total API Calls:     {}", snapshot.total_api_calls);
        println!("  Successful:          {}", snapshot.successful_calls);
        println!("  Failed:              {}", snapshot.failed_calls);
        println!("  Rate Limited:        {}", snapshot.rate_limited_calls);
        
        println!("\n💾 Cache Performance:");
        println!("  Cache Hits:          {}", snapshot.cache_hits);
        println!("  Cache Misses:        {}", snapshot.cache_misses);
        println!("  Hit Rate:            {:.2}%", snapshot.cache_hit_rate * 100.0);
        
        println!("\n⚡ Response Latency:");
        println!("  Average:             {:.2} ms", snapshot.avg_latency_ms);
        println!("  Min:                 {} ms", snapshot.min_latency_ms);
        println!("  Max:                 {} ms", snapshot.max_latency_ms);
        
        if !snapshot.error_counts.is_empty() {
            println!("\n❌ Error Breakdown:");
            for (error_type, count) in &snapshot.error_counts {
                println!("  {}: {}", error_type, count);
            }
        }
        
        println!("\n🔄 Circuit Breaker:");
        println!("  Trips:               {}", snapshot.circuit_breaker_trips);
        println!("  Recoveries:          {}", snapshot.circuit_breaker_recoveries);
        
        println!("\n⏱️  Uptime:             {} seconds", snapshot.uptime_secs);
        println!("╚════════════════════════════════════════════════════════════╝\n");
    }
}

impl Default for MetricsSnapshot {
    fn default() -> Self {
        MetricsSnapshot {
            total_api_calls: 0,
            successful_calls: 0,
            failed_calls: 0,
            rate_limited_calls: 0,
            cache_hits: 0,
            cache_misses: 0,
            cache_hit_rate: 0.0,
            avg_latency_ms: 0.0,
            min_latency_ms: 0,
            max_latency_ms: 0,
            error_counts: HashMap::new(),
            circuit_breaker_trips: 0,
            circuit_breaker_recoveries: 0,
            uptime_secs: 0,
        }
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_metrics_collection() {
        let metrics = MetricsCollector::new();
        
        metrics.record_api_success(100);
        metrics.record_cache_hit();
        
        let snapshot = metrics.snapshot();
        assert_eq!(snapshot.total_api_calls, 1);
        assert_eq!(snapshot.successful_calls, 1);
        assert_eq!(snapshot.cache_hits, 1);
    }
    
    #[test]
    fn test_cache_hit_rate() {
        let metrics = MetricsCollector::new();
        
        metrics.record_cache_hit();
        metrics.record_cache_hit();
        metrics.record_cache_miss();
        
        let snapshot = metrics.snapshot();
        assert_eq!(snapshot.cache_hit_rate, 2.0 / 3.0);
    }
}
