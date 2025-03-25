use serde::{Serialize, Deserialize};
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigItem {
    pub id: Uuid,
    pub key: String,
    pub value: String,
    pub version: u64,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub created_by: String,
    pub updated_by: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub is_encrypted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigNamespace {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub created_by: String,
    pub updated_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub is_active: bool,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub last_login: Option<SystemTime>,
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<String>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub resource: String,
    pub action: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigChange {
    pub id: Uuid,
    pub config_id: Uuid,
    pub old_value: Option<String>,
    pub new_value: String,
    pub change_type: ChangeType,
    pub created_at: SystemTime,
    pub created_by: String,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(i32)]
pub enum ChangeType {
    Create = 0,
    Update = 1,
    Delete = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigSnapshot {
    pub id: Uuid,
    pub namespace_id: Uuid,
    pub version: u64,
    pub created_at: SystemTime,
    pub created_by: String,
    pub items: Vec<ConfigItem>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigTemplate {
    pub id: Uuid,
    pub name: String,
    pub content: String,
    pub variables: Vec<String>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub created_by: String,
    pub updated_by: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigValidation {
    pub id: Uuid,
    pub config_id: Uuid,
    pub validator_type: ValidatorType,
    pub rule: String,
    pub error_message: String,
    pub created_at: SystemTime,
    pub created_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidatorType {
    Required,
    Format,
    Range,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigDependency {
    pub id: Uuid,
    pub config_id: Uuid,
    pub depends_on_id: Uuid,
    pub created_at: SystemTime,
    pub created_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigTag {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: SystemTime,
    pub created_by: String,
}