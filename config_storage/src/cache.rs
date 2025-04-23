use async_trait::async_trait;
use config_common::{ConfigContent, ConfigMeta, Result};

/// Cache trait for configuration data
#[async_trait]
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
