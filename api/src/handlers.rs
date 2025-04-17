use actix_web::{web, HttpResponse};

use crate::model::*;
use core::{ConfigFilter, ConfigManager};

/// REST API handlers

pub async fn get_config(
    id: web::Path<String>,
    config_manager: web::Data<dyn ConfigManager>,
) -> common::Result<HttpResponse> {
    let (meta, content) = config_manager.get_config(&id).await?;
    Ok(HttpResponse::Ok().json((meta, content)))
}

pub async fn create_config(
    req: web::Json<CreateConfigRequest>,
    user: String,
    config_manager: web::Data<dyn ConfigManager>,
) -> common::Result<HttpResponse> {
    let meta = config_manager
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
    id: web::Path<String>,
    req: web::Json<UpdateConfigRequest>,
    user: String,
    config_manager: web::Data<dyn ConfigManager>,
) -> common::Result<HttpResponse> {
    let meta = config_manager
        .update_config(&id, req.description.as_deref(), req.content.clone(), &user)
        .await?;
    Ok(HttpResponse::Ok().json(meta))
}

pub async fn delete_config(
    id: web::Path<String>,
    config_manager: web::Data<dyn ConfigManager>,
) -> common::Result<HttpResponse> {
    config_manager.delete_config(&id).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn list_configs(
    req: web::Query<ListConfigsRequest>,
    config_manager: web::Data<dyn ConfigManager>,
) -> common::Result<HttpResponse> {
    let filter = ConfigFilter {
        namespace: req.namespace.clone(),
        department: req.department.clone(),
        application: req.application.clone(),
        environment: req.environment.clone(),
    };

    let page_size = req.page_size.unwrap_or(10);
    let page_number = req.page_number.unwrap_or(1);

    let (configs, total) = config_manager
        .list_configs(filter, page_size, page_number)
        .await?;

    Ok(HttpResponse::Ok().json(ListConfigsResponse { configs, total }))
}
