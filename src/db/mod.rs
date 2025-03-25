use anyhow::Result;
use sea_orm::DatabaseConnection;
use crate::config::DatabaseConfig;

#[derive(Debug, Clone)]
pub struct DatabasePool {
    db: DatabaseConnection,
}

impl DatabasePool {
    pub async fn new(config: &DatabaseConfig) -> Result<Self> {
        let url = format!(
            "postgres://{}:{}@{}:{}/{}",
            config.username,
            config.password,
            config.host,
            config.port,
            config.database
        );
        
        let db = sea_orm::Database::connect(&url).await?;
        Ok(Self { db })
    }

    pub async fn get(&self) -> Result<DatabaseConnection> {
        Ok(self.db.clone())
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

        let db = DatabasePool::new(&config).await.unwrap();
        let conn = db.get().await.unwrap();
        assert!(conn.ping().await.is_ok());
    }
}
