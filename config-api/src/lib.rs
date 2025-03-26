use actix_web::{web, HttpResponse};
use config_common::{ConfigContent, ConfigMeta, Result};
use config_core::{ConfigFilter, ConfigManager};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod grpc;

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

/// REST API handlers
pub struct RestApi {
    config_manager: Arc<dyn ConfigManager>,
}

impl RestApi {
    pub fn new(config_manager: Arc<dyn ConfigManager>) -> Self {
        Self { config_manager }
    }

    pub async fn get_config(&self, id: web::Path<String>) -> Result<HttpResponse> {
        let (meta, content) = self.config_manager.get_config(&id).await?;
        Ok(HttpResponse::Ok().json((meta, content)))
    }

    pub async fn create_config(
        &self,
        req: web::Json<CreateConfigRequest>,
        user: String,
    ) -> Result<HttpResponse> {
        let meta = self
            .config_manager
            .create_config(
                &req.name,
                &req.namespace,
                &req.department,
                &req.application,
                &req.environment,
                req.description.as_deref(),
                req.content.clone(),
                &user,
            )
            .await?;
        Ok(HttpResponse::Created().json(meta))
    }

    pub async fn update_config(
        &self,
        id: web::Path<String>,
        req: web::Json<UpdateConfigRequest>,
        user: String,
    ) -> Result<HttpResponse> {
        let meta = self
            .config_manager
            .update_config(&id, req.description.as_deref(), req.content.clone(), &user)
            .await?;
        Ok(HttpResponse::Ok().json(meta))
    }

    pub async fn delete_config(&self, id: web::Path<String>) -> Result<HttpResponse> {
        self.config_manager.delete_config(&id).await?;
        Ok(HttpResponse::NoContent().finish())
    }

    pub async fn list_configs(&self, req: web::Query<ListConfigsRequest>) -> Result<HttpResponse> {
        let filter = ConfigFilter {
            namespace: req.namespace.clone(),
            department: req.department.clone(),
            application: req.application.clone(),
            environment: req.environment.clone(),
        };

        let page_size = req.page_size.unwrap_or(10);
        let page_number = req.page_number.unwrap_or(1);

        let (configs, total) = self
            .config_manager
            .list_configs(filter, page_size, page_number)
            .await?;

        Ok(HttpResponse::Ok().json(ListConfigsResponse { configs, total }))
    }
}

/// Configure REST API routes
pub fn configure_routes(
    config: &mut web::ServiceConfig,
    config_manager: Arc<dyn ConfigManager>,
) {
    let api = RestApi::new(config_manager);
    
    config.service(
        web::scope("/api/v1")
            .route("/configs", web::post().to(move |req, user| api.create_config(req, user)))
            .route("/configs", web::get().to(move |req| api.list_configs(req)))
            .route("/configs/{id}", web::get().to(move |id| api.get_config(id)))
            .route(
                "/configs/{id}",
                web::put().to(move |id, req, user| api.update_config(id, req, user)),
            )
            .route(
                "/configs/{id}",
                web::delete().to(move |id| api.delete_config(id)),
            ),
    );
} 