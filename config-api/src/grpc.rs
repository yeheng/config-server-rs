use config_common::{ConfigContent, ConfigMeta};
use config_core::{ConfigFilter, ConfigManager};
use config_proto::config_service_server::{ConfigService, ConfigServiceServer};
use config_proto::{
    ConfigContent as ProtoConfigContent, ConfigMetadata as ProtoConfigMetadata,
    CreateConfigRequest, CreateConfigResponse, DeleteConfigRequest, DeleteConfigResponse,
    GetConfigRequest, GetConfigResponse, ListConfigsRequest, ListConfigsResponse,
    UpdateConfigRequest, UpdateConfigResponse, WatchConfigRequest, WatchConfigResponse,
};
use std::sync::Arc;
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};

pub struct GrpcApi {
    config_manager: Arc<dyn ConfigManager>,
}

impl GrpcApi {
    pub fn new(config_manager: Arc<dyn ConfigManager>) -> Self {
        Self { config_manager }
    }

    pub fn into_service(self) -> ConfigServiceServer<Self> {
        ConfigServiceServer::new(self)
    }

    fn convert_meta_to_proto(meta: ConfigMeta) -> ProtoConfigMetadata {
        ProtoConfigMetadata {
            id: meta.id,
            name: meta.name,
            namespace: meta.namespace,
            department: meta.department,
            application: meta.application,
            environment: meta.environment,
            version: meta.version,
            description: meta.description,
            created_at: meta.created_at,
            updated_at: meta.updated_at,
            created_by: meta.created_by,
            updated_by: meta.updated_by,
        }
    }

    fn convert_content_to_proto(content: ConfigContent) -> ProtoConfigContent {
        ProtoConfigContent {
            format: content.format as i32,
            content: content.content,
            is_encrypted: content.is_encrypted,
        }
    }
}

#[tonic::async_trait]
impl ConfigService for GrpcApi {
    async fn get_config(
        &self,
        request: Request<GetConfigRequest>,
    ) -> Result<Response<GetConfigResponse>, Status> {
        let id = request.into_inner().id;
        let (meta, content) = self
            .config_manager
            .get_config(&id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(GetConfigResponse {
            metadata: Some(Self::convert_meta_to_proto(meta)),
            content: Some(Self::convert_content_to_proto(content)),
        }))
    }

    async fn create_config(
        &self,
        request: Request<CreateConfigRequest>,
    ) -> Result<Response<CreateConfigResponse>, Status> {
        let req = request.into_inner();
        let content = ConfigContent {
            format: req.content.unwrap().format.into(),
            content: req.content.unwrap().content,
            is_encrypted: req.content.unwrap().is_encrypted,
        };

        let meta = self
            .config_manager
            .create_config(
                &req.name,
                &req.namespace,
                &req.department,
                &req.application,
                &req.environment,
                req.description.as_deref(),
                content,
                "system", // TODO: Get from context
            )
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(CreateConfigResponse {
            metadata: Some(Self::convert_meta_to_proto(meta)),
        }))
    }

    async fn update_config(
        &self,
        request: Request<UpdateConfigRequest>,
    ) -> Result<Response<UpdateConfigResponse>, Status> {
        let req = request.into_inner();
        let content = ConfigContent {
            format: req.content.unwrap().format.into(),
            content: req.content.unwrap().content,
            is_encrypted: req.content.unwrap().is_encrypted,
        };

        let meta = self
            .config_manager
            .update_config(
                &req.id,
                req.description.as_deref(),
                content,
                "system", // TODO: Get from context
            )
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(UpdateConfigResponse {
            metadata: Some(Self::convert_meta_to_proto(meta)),
        }))
    }

    async fn delete_config(
        &self,
        request: Request<DeleteConfigRequest>,
    ) -> Result<Response<DeleteConfigResponse>, Status> {
        let id = request.into_inner().id;
        let success = self
            .config_manager
            .delete_config(&id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(DeleteConfigResponse { success }))
    }

    async fn list_configs(
        &self,
        request: Request<ListConfigsRequest>,
    ) -> Result<Response<ListConfigsResponse>, Status> {
        let req = request.into_inner();
        let filter = ConfigFilter {
            namespace: req.namespace,
            department: req.department,
            application: req.application,
            environment: req.environment,
        };

        let (configs, total) = self
            .config_manager
            .list_configs(filter, req.page_size, req.page_number)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ListConfigsResponse {
            configs: configs
                .into_iter()
                .map(Self::convert_meta_to_proto)
                .collect(),
            total,
        }))
    }

    type WatchConfigStream = mpsc::Receiver<Result<WatchConfigResponse, Status>>;

    async fn watch_config(
        &self,
        _request: Request<WatchConfigRequest>,
    ) -> Result<Response<Self::WatchConfigStream>, Status> {
        // TODO: Implement watch functionality
        Err(Status::unimplemented("Watch functionality not implemented"))
    }
} 