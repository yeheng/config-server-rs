use async_trait::async_trait;
use common::{ConfigContent, ConfigMeta, Result};
use serde::{Deserialize, Serialize};

/// Configuration manager trait defining core operations
#[async_trait]
pub trait ConfigManager: Send + Sync {
    /// Get configuration by ID
    async fn get_config(&self, id: &str) -> Result<(ConfigMeta, ConfigContent)>;

    /// Create new configuration
    async fn create_config(
        &self,
        name: &str,
        namespace: &str,
        department: &str,
        application: &str,
        environment: &str,
        description: Option<&str>,
        content: ConfigContent,
        created_by: &str,
    ) -> Result<ConfigMeta>;

    /// Update existing configuration
    async fn update_config(
        &self,
        id: &str,
        description: Option<&str>,
        content: ConfigContent,
        updated_by: &str,
    ) -> Result<ConfigMeta>;

    /// Delete configuration
    async fn delete_config(&self, id: &str) -> Result<bool>;

    /// List configurations with filters
    async fn list_configs(
        &self,
        filter: ConfigFilter,
        page_size: i32,
        page_number: i32,
    ) -> Result<(Vec<ConfigMeta>, i32)>;
}

/// Configuration filter for listing configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFilter {
    pub namespace: Option<String>,
    pub department: Option<String>,
    pub application: Option<String>,
    pub environment: Option<String>,
}

/// Configuration validator trait for validating configuration content
#[async_trait]
pub trait ConfigValidator: Send + Sync {
    /// Validate configuration content
    async fn validate(&self, content: &ConfigContent) -> Result<()>;
}

/// Configuration encryption trait for encrypting/decrypting configuration content
#[async_trait]
pub trait ConfigEncryption: Send + Sync {
    /// Encrypt configuration content
    async fn encrypt(&self, content: &str) -> Result<String>;

    /// Decrypt configuration content
    async fn decrypt(&self, content: &str) -> Result<String>;
}

/// Configuration version control trait
#[async_trait]
pub trait ConfigVersionControl: Send + Sync {
    /// Get configuration version history
    async fn get_version_history(&self, id: &str) -> Result<Vec<ConfigVersion>>;

    /// Roll back to specific version
    async fn rollback(&self, id: &str, version: &str, user: &str) -> Result<ConfigMeta>;
}

/// Configuration version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigVersion {
    pub version: String,
    pub created_at: i64,
    pub created_by: String,
    pub description: Option<String>,
}
