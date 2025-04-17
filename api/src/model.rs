use common::{ConfigContent, ConfigMeta};
use serde::{Deserialize, Serialize};

/// REST API request and response types
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateConfigRequest {
    pub name: String,
    pub namespace: String,
    pub department: String,
    pub application: String,
    pub environment: String,
    pub description: Option<String>,
    pub content: ConfigContent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateConfigRequest {
    pub description: Option<String>,
    pub content: ConfigContent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListConfigsRequest {
    pub namespace: Option<String>,
    pub department: Option<String>,
    pub application: Option<String>,
    pub environment: Option<String>,
    pub page_size: Option<i32>,
    pub page_number: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ListConfigsResponse {
    pub configs: Vec<ConfigMeta>,
    pub total: i32,
}
