use config::File;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub api: ApiConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub raft: RaftConfig,
    pub auth: AuthConfig,
    pub monitor: MonitorConfig,
    pub audit: AuditConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
    pub grpc_port: u16,
    pub tls: Option<TlsConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub max_connections: u32,
    pub idle_timeout: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
    pub database: i64,
    pub pool_size: u32,
    pub connection_timeout: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RaftConfig {
    pub node_id: String,
    pub data_dir: PathBuf,
    pub peers: Vec<String>,
    pub heartbeat_interval: u64,
    pub election_timeout: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub token_expiration: u64,
    pub password_hash_cost: u32,
    pub rbac_model: PathBuf,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MonitorConfig {
    pub metrics_port: u16,
    pub prometheus_path: String,
    pub alert_rules: PathBuf,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuditConfig {
    pub log_dir: PathBuf,
    pub max_size: u64,
    pub max_files: u32,
    pub compression: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TlsConfig {
    pub cert_file: PathBuf,
    pub key_file: PathBuf,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let config_path = std::env::var("CONFIG_PATH")
            .unwrap_or_else(|_| "config/default.toml".to_string());
        let config = config::Config::builder()
            .add_source(File::from(std::path::Path::new(&config_path)))
            .add_source(config::Environment::with_prefix("CONFIG"))
            .build()?;

        Ok(config.try_deserialize()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_loading() {
        let config = Config::load();
        assert!(config.is_ok());
    }
}
