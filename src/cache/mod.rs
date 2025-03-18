use anyhow::Result;
use redis::{Client, AsyncCommands};
use crate::config::RedisConfig;

#[derive(Debug, Clone)]
pub struct RedisCache {
    client: Client,
}

impl RedisCache {
    pub async fn new(config: &RedisConfig) -> Result<Self> {
        let redis_url = format!(
            "redis://{}:{}@{}:{}/{}",
            config.password.as_deref().unwrap_or(""),
            config.password.as_deref().unwrap_or(""),
            config.host,
            config.port,
            config.database
        );

        let client = Client::open(redis_url)?;
        
        // Test connection
        let mut conn = client.get_multiplexed_async_connection().await?;
        redis::cmd("PING").query_async::<String>(&mut conn).await?;

        Ok(Self { client })
    }

    pub async fn get_connection(&self) -> Result<redis::aio::MultiplexedConnection> {
        Ok(self.client.get_multiplexed_async_connection().await?)
    }

    pub async fn get<T: serde::de::DeserializeOwned>(
        &self,
        key: &str,
    ) -> Result<Option<T>> {
        let mut conn = self.get_connection().await?;
        let value: Option<String> = conn.get(key).await?;
        
        match value {
            Some(v) => Ok(Some(serde_json::from_str(&v)?)),
            None => Ok(None),
        }
    }

    pub async fn set<T: serde::Serialize>(
        &self,
        key: &str,
        value: &T,
        ttl: Option<u64>,
    ) -> Result<()> {
        let mut conn = self.get_connection().await?;
        let value = serde_json::to_string(value)?;
        
        if let Some(ttl) = ttl {
            redis::cmd("SETEX")
                .arg(key)
                .arg(ttl)
                .arg(value)
                .query_async::<()>(&mut conn)
                .await?;
        } else {
            conn.set::<_, _, ()>(key, value).await?;
        }

        Ok(())
    }

    pub async fn delete(&self, key: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;
        conn.del::<_, ()>(key).await?;
        Ok(())
    }

    pub async fn health_check(&self) -> Result<bool> {
        let mut conn = self.get_connection().await?;
        let result: String = redis::cmd("PING")
            .query_async::<String>(&mut conn)
            .await?;
        Ok(result == "PONG")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_redis_connection() {
        let config = RedisConfig {
            host: "localhost".to_string(),
            port: 6379,
            password: None,
            database: 0,
            pool_size: 10,
            connection_timeout: 5,
        };

        let cache = RedisCache::new(&config).await;
        assert!(cache.is_ok());
    }

    #[tokio::test]
    async fn test_redis_operations() {
        let config = RedisConfig {
            host: "localhost".to_string(),
            port: 6379,
            password: None,
            database: 0,
            pool_size: 10,
            connection_timeout: 5,
        };

        let cache = RedisCache::new(&config).await.unwrap();
        
        // Test set and get
        let test_value = "test_value";
        cache.set("test_key", &test_value, None).await.unwrap();
        let result: Option<String> = cache.get("test_key").await.unwrap();
        assert_eq!(result, Some(test_value.to_string()));

        // Test delete
        cache.delete("test_key").await.unwrap();
        let result: Option<String> = cache.get("test_key").await.unwrap();
        assert_eq!(result, None);
    }
}
