mod handlers;
pub mod model;

use actix_web::web;
use config_core::ConfigManager;
use std::sync::Arc;

pub use crate::model::CreateConfigRequest;
pub use crate::model::ListConfigsRequest;
pub use crate::model::ListConfigsResponse;
pub use crate::model::UpdateConfigRequest;

/// Configure REST API routes
pub fn configure_routes(config: &mut web::ServiceConfig, config_manager: Arc<dyn ConfigManager>) {
    config.app_data(web::Data::new(config_manager));

    config.service(
        web::scope("/api/v1")
            .route("/configs", web::post().to(handlers::create_config))
            .route("/configs", web::get().to(handlers::list_configs))
            .route("/configs/{id}", web::get().to(handlers::get_config))
            .route("/configs/{id}", web::put().to(handlers::update_config))
            .route("/configs/{id}", web::delete().to(handlers::delete_config)),
    );
}
