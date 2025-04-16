use common::Result;
use prometheus::{Counter, Histogram, HistogramOpts, IntGauge, Registry};
use tracing::{error, info, warn};

/// Monitoring service for collecting metrics and logs
pub struct MonitoringService {
    registry: Registry,
    config_operations: ConfigMetrics,
    system_metrics: SystemMetrics,
}

impl MonitoringService {
    pub fn new() -> Result<Self> {
        let registry = Registry::new();
        let config_operations = ConfigMetrics::new(&registry)?;
        let system_metrics = SystemMetrics::new(&registry)?;

        Ok(Self {
            registry,
            config_operations,
            system_metrics,
        })
    }

    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    pub fn metrics(&self) -> &ConfigMetrics {
        &self.config_operations
    }

    pub fn system(&self) -> &SystemMetrics {
        &self.system_metrics
    }
}

/// Configuration operation metrics
pub struct ConfigMetrics {
    get_total: Counter,
    create_total: Counter,
    update_total: Counter,
    delete_total: Counter,
    operation_duration: Histogram,
    error_total: Counter,
}

impl ConfigMetrics {
    fn new(registry: &Registry) -> Result<Self> {
        let get_total = Counter::new(
            "config_get_total",
            "Total number of configuration get operations",
        )?;
        let create_total = Counter::new(
            "config_create_total",
            "Total number of configuration create operations",
        )?;
        let update_total = Counter::new(
            "config_update_total",
            "Total number of configuration update operations",
        )?;
        let delete_total = Counter::new(
            "config_delete_total",
            "Total number of configuration delete operations",
        )?;
        let operation_duration = Histogram::with_opts(
            HistogramOpts::new(
                "config_operation_duration_seconds",
                "Configuration operation duration in seconds",
            )
            .buckets(vec![0.01, 0.05, 0.1, 0.5, 1.0, 5.0]),
        )?;
        let error_total = Counter::new(
            "config_error_total",
            "Total number of configuration operation errors",
        )?;

        registry.register(Box::new(get_total.clone()))?;
        registry.register(Box::new(create_total.clone()))?;
        registry.register(Box::new(update_total.clone()))?;
        registry.register(Box::new(delete_total.clone()))?;
        registry.register(Box::new(operation_duration.clone()))?;
        registry.register(Box::new(error_total.clone()))?;

        Ok(Self {
            get_total,
            create_total,
            update_total,
            delete_total,
            operation_duration,
            error_total,
        })
    }

    pub fn record_get(&self) {
        self.get_total.inc();
    }

    pub fn record_create(&self) {
        self.create_total.inc();
    }

    pub fn record_update(&self) {
        self.update_total.inc();
    }

    pub fn record_delete(&self) {
        self.delete_total.inc();
    }

    pub fn record_duration(&self, duration: f64) {
        self.operation_duration.observe(duration);
    }

    pub fn record_error(&self) {
        self.error_total.inc();
    }
}

/// System metrics
pub struct SystemMetrics {
    cpu_usage: IntGauge,
    memory_usage: IntGauge,
    open_connections: IntGauge,
    goroutines: IntGauge,
}

impl SystemMetrics {
    fn new(registry: &Registry) -> Result<Self> {
        let cpu_usage = IntGauge::new("system_cpu_usage", "Current CPU usage percentage")?;
        let memory_usage =
            IntGauge::new("system_memory_usage_bytes", "Current memory usage in bytes")?;
        let open_connections =
            IntGauge::new("system_open_connections", "Number of open connections")?;
        let goroutines = IntGauge::new("system_goroutines", "Number of active goroutines")?;

        registry.register(Box::new(cpu_usage.clone()))?;
        registry.register(Box::new(memory_usage.clone()))?;
        registry.register(Box::new(open_connections.clone()))?;
        registry.register(Box::new(goroutines.clone()))?;

        Ok(Self {
            cpu_usage,
            memory_usage,
            open_connections,
            goroutines,
        })
    }

    pub fn set_cpu_usage(&self, usage: i64) {
        self.cpu_usage.set(usage);
    }

    pub fn set_memory_usage(&self, usage: i64) {
        self.memory_usage.set(usage);
    }

    pub fn set_open_connections(&self, count: i64) {
        self.open_connections.set(count);
    }

    pub fn set_goroutines(&self, count: i64) {
        self.goroutines.set(count);
    }
}

/// Initialize logging with tracing
pub fn init_logging() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init()
        .map_err(|e| config_common::Error::Internal(e.to_string()))?;

    Ok(())
}

/// Log levels
pub fn log_info(message: &str) {
    info!("{}", message);
}

pub fn log_warn(message: &str) {
    warn!("{}", message);
}

pub fn log_error(message: &str) {
    error!("{}", message);
}
