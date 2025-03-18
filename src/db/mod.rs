use anyhow::Result;
use deadpool_postgres::{tokio_postgres::NoTls, Config, Pool, PoolConfig, Runtime};
use crate::config::DatabaseConfig;

#[derive(Debug, Clone)]
pub struct Database {
    pool: Pool,
}

impl Database {
    pub async fn new(config: &DatabaseConfig) -> Result<Self> {
        let mut cfg = Config::new();
        cfg.host = Some(config.host.clone());
        cfg.port = Some(config.port);
        cfg.user = Some(config.username.clone());
        cfg.password = Some(config.password.clone());
        cfg.dbname = Some(config.database.clone());

        let pool_config = PoolConfig {
            max_size: config.max_connections as usize,
            timeouts: deadpool_postgres::Timeouts {
                wait: Some(std::time::Duration::from_secs(config.idle_timeout)),
                ..Default::default()
            },
            ..Default::default()
        };

        cfg.pool = Some(pool_config);
        let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;
        
        Ok(Self { pool })
    }

    pub async fn get_client(&self) -> Result<deadpool_postgres::Client> {
        Ok(self.pool.get().await?)
    }

    pub async fn health_check(&self) -> Result<bool> {
        let client = self.get_client().await?;
        let result = client.query_one("SELECT 1", &[]).await?;
        Ok(result.get::<_, i32>(0) == 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_database_connection() {
        let config = DatabaseConfig {
            host: "localhost".to_string(),
            port: 5432,
            username: "postgres".to_string(),
            password: "postgres".to_string(),
            database: "config_center_test".to_string(),
            max_connections: 10,
            idle_timeout: 300,
        };

        let db = Database::new(&config).await;
        assert!(db.is_ok());
    }
}
