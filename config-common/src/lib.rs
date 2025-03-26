use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Common result type used throughout the project
pub type Result<T> = std::result::Result<T, Error>;

/// Common error type for the configuration center
#[derive(Debug, Error)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Cache error: {0}")]
    Cache(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Prometheus error: {0}")]
    PrometheusError(String),
}

impl From<prometheus::Error> for Error {
    fn from(err: prometheus::Error) -> Self {
        Error::PrometheusError(err.to_string())
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::Database(err.to_string())
    }
}

/// Configuration metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigMeta {
    pub id: String,
    pub name: String,
    pub namespace: String,
    pub department: String,
    pub application: String,
    pub environment: String,
    pub version: String,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub created_by: String,
    pub updated_by: String,
}

/// Configuration content with type information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigContent {
    pub format: ConfigFormat,
    pub content: String,
    pub is_encrypted: bool,
}

/// Supported configuration formats
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ConfigFormat {
    Yaml,
    Properties,
    Json,
    Toml,
}

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: String,
    pub user: String,
    pub action: String,
    pub resource: String,
    pub details: String,
    pub timestamp: i64,
}

/// Role-based access control policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RbacPolicy {
    pub role: String,
    pub resource: String,
    pub action: String,
    pub effect: PolicyEffect,
}

/// Policy effect
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PolicyEffect {
    Allow,
    Deny,
}

/// Configuration change event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigEvent {
    pub config_id: String,
    pub event_type: ConfigEventType,
    pub version: String,
    pub timestamp: i64,
    pub user: String,
}

/// Configuration event types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ConfigEventType {
    Created,
    Updated,
    Deleted,
    Released,
    Rolled,
} 