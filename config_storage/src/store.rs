use async_trait::async_trait;
use config_common::{ConfigContent, ConfigMeta, Result};
use config_core::{ConfigFilter, ConfigVersion};

/// Storage trait for configuration data
#[async_trait]
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
