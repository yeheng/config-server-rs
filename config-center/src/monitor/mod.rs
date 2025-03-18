use anyhow::Result;
use metrics::{counter, gauge, histogram};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::net::SocketAddr;
use crate::config::MonitorConfig;

pub struct Monitor {
    config: MonitorConfig,
    handle: PrometheusHandle,
}

impl Monitor {
    pub fn new(config: &MonitorConfig) -> Result<Self> {
        // Initialize Prometheus metrics exporter
        let (recorder, handle) = PrometheusBuilder::new().build()?;
        metrics::set_boxed_recorder(Box::new(recorder))?;

        Ok(Self {
            config: config.clone(),
            handle,
        })
    }

    pub async fn start(&self) -> Result<()> {
        let addr = SocketAddr::from(([0, 0, 0, 0], self.config.metrics_port));
        let handle = self.handle.clone();
        let server = warp::serve(warp::path(self.config.prometheus_path.clone())
            .map(move || handle.render()));

        tokio::spawn(server.run(addr));
        Ok(())
    }

    // API metrics
    pub fn record_api_request(&self, endpoint: &str, method: &str, status: u16) {
        counter!("api_requests_total", 1, &[("endpoint", endpoint), ("method", method), ("status", &status.to_string())]);
    }

    pub fn record_api_latency(&self, endpoint: &str, method: &str, duration: f64) {
        histogram!("api_request_duration_seconds", duration, &[("endpoint", endpoint), ("method", method)]);
    }

    // Cache metrics
    pub fn record_cache_hit(&self) {
        counter!("cache_hits_total", 1);
    }

    pub fn record_cache_miss(&self) {
        counter!("cache_misses_total", 1);
    }

    pub fn record_cache_size(&self, size: f64) {
        gauge!("cache_size_bytes", size);
    }

    // Database metrics
    pub fn record_db_query(&self, query_type: &str, duration: f64) {
        histogram!("db_query_duration_seconds", duration, &[("query_type", query_type)]);
    }

    pub fn record_db_connections(&self, active: f64) {
        gauge!("db_connections_active", active);
    }

    // Raft metrics
    pub fn record_raft_state(&self, state: &str) {
        gauge!("raft_state", 1.0, &[("state", state)]);
    }

    pub fn record_raft_term(&self, term: f64) {
        gauge!("raft_term", term);
    }

    // System metrics
    pub fn record_memory_usage(&self, bytes: f64) {
        gauge!("memory_usage_bytes", bytes);
    }

    pub fn record_cpu_usage(&self, percentage: f64) {
        gauge!("cpu_usage_percentage", percentage);
    }

    pub fn record_disk_usage(&self, bytes: f64) {
        gauge!("disk_usage_bytes", bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_monitor_initialization() {
        let config = MonitorConfig {
            metrics_port: 9090,
            prometheus_path: "/metrics".to_string(),
            alert_rules: std::path::PathBuf::from("config/alert_rules.yml"),
        };

        let monitor = Monitor::new(&config).unwrap();
        assert!(monitor.start().await.is_ok());
    }

    #[test]
    fn test_metrics_recording() {
        let config = MonitorConfig {
            metrics_port: 9090,
            prometheus_path: "/metrics".to_string(),
            alert_rules: std::path::PathBuf::from("config/alert_rules.yml"),
        };

        let monitor = Monitor::new(&config).unwrap();

        // Record some test metrics
        monitor.record_api_request("/test", "GET", 200);
        monitor.record_api_latency("/test", "GET", 0.1);
        monitor.record_cache_hit();
        monitor.record_cache_miss();
        monitor.record_cache_size(1024.0);
        monitor.record_db_query("SELECT", 0.05);
        monitor.record_db_connections(5.0);
        monitor.record_raft_state("leader");
        monitor.record_raft_term(1.0);
        monitor.record_memory_usage(1024.0 * 1024.0);
        monitor.record_cpu_usage(50.0);
        monitor.record_disk_usage(1024.0 * 1024.0 * 1024.0);
    }
}
