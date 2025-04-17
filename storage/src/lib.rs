use common::{ConfigContent, ConfigMeta, Result};
use core::{ConfigFilter, ConfigVersion};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};

/// Storage trait for configuration data
pub trait ConfigStorage: Send + Sync {
    /// Get configuration by ID
    async fn get_config(&self, id: &str) -> Result<(ConfigMeta, ConfigContent)>;

    /// Create new configuration
    async fn create_config(&self, meta: ConfigMeta, content: ConfigContent) -> Result<ConfigMeta>;

    /// Update existing configuration
    async fn update_config(&self, meta: ConfigMeta, content: ConfigContent) -> Result<ConfigMeta>;

    /// Delete configuration
    async fn delete_config(&self, id: &str) -> Result<bool>;

    /// List configurations with filters
    async fn list_configs(
        &self,
        filter: ConfigFilter,
        page_size: i32,
        page_number: i32,
    ) -> Result<(Vec<ConfigMeta>, i32)>;

    /// Get configuration version history
    async fn get_version_history(&self, id: &str) -> Result<Vec<ConfigVersion>>;

    /// Create new version
    async fn create_version(
        &self,
        config_id: &str,
        version: ConfigVersion,
        content: ConfigContent,
    ) -> Result<()>;
}

/// Cache trait for configuration data
pub trait ConfigCache: Send + Sync {
    /// Get configuration from cache
    async fn get_config(&self, id: &str) -> Result<Option<(ConfigMeta, ConfigContent)>>;

    /// Set configuration in cache
    async fn set_config(&self, meta: &ConfigMeta, content: &ConfigContent) -> Result<()>;

    /// Delete configuration from cache
    async fn delete_config(&self, id: &str) -> Result<()>;

    /// Clear all cached configurations
    async fn clear_all(&self) -> Result<()>;
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub max_connections: u32,
}

impl DatabaseConfig {
    /// Create database connection pool
    pub async fn create_pool(&self) -> Result<PgPool> {
        let connection_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        );

        let pool = PgPoolOptions::new()
            .max_connections(self.max_connections)
            .connect(&connection_string)
            .await
            .map_err(|e| common::Error::Database(e.to_string()))?;

        Ok(pool)
    }
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
    pub db: i32,
    pub ttl: u64,
}

impl CacheConfig {
    /// Create Redis client
    pub fn create_client(&self) -> Result<redis::Client> {
        let url = match &self.password {
            Some(password) => format!(
                "redis://:{}@{}:{}/{}",
                password, self.host, self.port, self.db
            ),
            None => format!("redis://{}:{}/{}", self.host, self.port, self.db),
        };

        redis::Client::open(url).map_err(|e| common::Error::Cache(e.to_string()))
    }
}
