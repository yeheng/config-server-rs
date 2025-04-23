use config_common::Result;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};

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
            .map_err(|e| config_common::Error::Database(e.to_string()))?;

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

        redis::Client::open(url).map_err(|e| config_common::Error::Cache(e.to_string()))
    }
}
