use crate::config::MonitorConfig;
use anyhow::Result;
use metrics::*;
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::net::SocketAddr;
use warp::Filter;

pub struct Monitor {
    config: MonitorConfig,
    handle: PrometheusHandle,
}

impl Monitor {
    pub fn new(config: &MonitorConfig) -> Result<Self> {
        let (recorder, _) = PrometheusBuilder::new().build()?;
        // 提前获取handle
        let handle = recorder.handle(); // 关键修改点
        metrics::set_global_recorder(recorder)?;

        Ok(Self {
            config: config.clone(),
            handle, // 直接使用已获取的handle
        })
    }

    pub async fn start(&self) -> Result<()> {
        let addr = SocketAddr::from(([0, 0, 0, 0], self.config.metrics_port));
        let handle = self.handle.clone();
        let path = self.config.prometheus_path.clone();

        let metrics_route = warp::path(path)
            .and(warp::get())
            .map(move || handle.render());

        let server = warp::serve(metrics_route);
        tokio::spawn(server.run(addr));
        Ok(())
    }

    // API metrics
    pub fn record_api_request(&self, endpoint: &str, method: &str, status: u16) {
        let counter = counter!("api_requests_total");
        counter.increment(1);
    }

    pub fn record_api_latency(&self, endpoint: &str, method: &str, duration: f64) {
        histogram!("api_request_duration_seconds").record(duration);
    }

    // Cache metrics
    pub fn record_cache_hit(&self) {
        counter!("cache_hits_total").increment(1);
    }

    pub fn record_cache_miss(&self) {
        counter!("cache_misses_total").increment(1);
    }

    pub fn record_cache_size(&self, size: f64) {
        gauge!("cache_size_bytes").set(size);
    }

    // Database metrics
    pub fn record_db_query(&self, query_type: &str, duration: f64) {
        histogram!("db_query_duration_seconds").record(duration);
    }

    pub fn record_db_connections(&self, active: f64) {
        gauge!("db_connections_active").set(active);
    }

    pub fn record_raft_term(&self, term: f64) {
        gauge!("raft_term").set(term);
    }

    // System metrics
    pub fn record_memory_usage(&self, bytes: f64) {
        gauge!("memory_usage_bytes").set(bytes);
    }

    pub fn record_cpu_usage(&self, percentage: f64) {
        gauge!("cpu_usage_percentage").set(percentage);
    }

    pub fn record_disk_usage(&self, bytes: f64) {
        gauge!("disk_usage_bytes").set(bytes);
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
        // monitor.record_raft_state("leader");
        monitor.record_raft_term(1.0);
        monitor.record_memory_usage(1024.0 * 1024.0);
        monitor.record_cpu_usage(50.0);
        monitor.record_disk_usage(1024.0 * 1024.0 * 1024.0);
    }
}
