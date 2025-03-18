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
pub enum ChangeType {
    Create,
    Update,
    Delete,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    #[test]
    fn test_config_item_serialization() {
        let item = ConfigItem {
            id: Uuid::new_v4(),
            key: "test.key".to_string(),
            value: "test_value".to_string(),
            version: 1,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
            created_by: "test_user".to_string(),
            updated_by: "test_user".to_string(),
            description: Some("Test config item".to_string()),
            tags: vec!["test".to_string()],
            is_encrypted: false,
        };

        let serialized = serde_json::to_string(&item).unwrap();
        let deserialized: ConfigItem = serde_json::from_str(&serialized).unwrap();
        assert_eq!(item.key, deserialized.key);
        assert_eq!(item.value, deserialized.value);
    }

    #[test]
    fn test_config_change_serialization() {
        let change = ConfigChange {
            id: Uuid::new_v4(),
            config_id: Uuid::new_v4(),
            old_value: Some("old_value".to_string()),
            new_value: "new_value".to_string(),
            change_type: ChangeType::Update,
            created_at: SystemTime::now(),
            created_by: "test_user".to_string(),
            reason: Some("Test change".to_string()),
        };

        let serialized = serde_json::to_string(&change).unwrap();
        let deserialized: ConfigChange = serde_json::from_str(&serialized).unwrap();
        assert_eq!(change.new_value, deserialized.new_value);
    }
}
