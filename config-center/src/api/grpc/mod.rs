use anyhow::Result;
use tonic::{transport::Server, Request, Response, Status};
use std::sync::Arc;
use crate::{
    config::ApiConfig,
    db::Database,
    cache::RedisCache,
    raft::RaftNode,
    auth::Auth,
    monitor::Monitor,
    audit::Audit,
    types::*,
};

// Generated protobuf code
pub mod config_service {
    tonic::include_proto!("config_service");
}

use config_service::{
    config_service_server::{ConfigService, ConfigServiceServer},
    CreateConfigRequest, CreateConfigResponse,
    GetConfigRequest, GetConfigResponse,
    UpdateConfigRequest, UpdateConfigResponse,
    DeleteConfigRequest, DeleteConfigResponse,
    ListConfigsRequest, ListConfigsResponse,
    CreateNamespaceRequest, CreateNamespaceResponse,
    GetNamespaceRequest, GetNamespaceResponse,
    UpdateNamespaceRequest, UpdateNamespaceResponse,
    DeleteNamespaceRequest, DeleteNamespaceResponse,
    ListNamespacesRequest, ListNamespacesResponse,
    CreateUserRequest, CreateUserResponse,
    GetUserRequest, GetUserResponse,
    UpdateUserRequest, UpdateUserResponse,
    DeleteUserRequest, DeleteUserResponse,
    ListUsersRequest, ListUsersResponse,
    CreateRoleRequest, CreateRoleResponse,
    GetRoleRequest, GetRoleResponse,
    UpdateRoleRequest, UpdateRoleResponse,
    DeleteRoleRequest, DeleteRoleResponse,
    ListRolesRequest, ListRolesResponse,
    CreatePermissionRequest, CreatePermissionResponse,
    GetPermissionRequest, GetPermissionResponse,
    UpdatePermissionRequest, UpdatePermissionResponse,
    DeletePermissionRequest, DeletePermissionResponse,
    ListPermissionsRequest, ListPermissionsResponse,
};

pub struct GrpcServer {
    config: ApiConfig,
    db: Arc<Database>,
    cache: Arc<RedisCache>,
    raft: Arc<RaftNode>,
    auth: Arc<Auth>,
    monitor: Arc<Monitor>,
    audit: Arc<Audit>,
}

impl GrpcServer {
    pub fn new(
        config: ApiConfig,
        db: Arc<Database>,
        cache: Arc<RedisCache>,
        raft: Arc<RaftNode>,
        auth: Arc<Auth>,
        monitor: Arc<Monitor>,
        audit: Arc<Audit>,
    ) -> Self {
        Self {
            config,
            db,
            cache,
            raft,
            auth,
            monitor,
            audit,
        }
    }

    pub async fn start(&self) -> Result<()> {
        let addr = format!("{}:{}", self.config.host, self.config.grpc_port).parse()?;

        Server::builder()
            .add_service(ConfigServiceServer::new(self.clone()))
            .serve(addr)
            .await?;

        Ok(())
    }
}

#[tonic::async_trait]
impl ConfigService for GrpcServer {
    async fn create_config(
        &self,
        request: Request<CreateConfigRequest>,
    ) -> Result<Response<CreateConfigResponse>, Status> {
        // TODO: Implement create_config
        Ok(Response::new(CreateConfigResponse {
            config: Some(config_service::Config {
                id: uuid::Uuid::new_v4().to_string(),
                key: request.into_inner().key,
                value: "test_value".to_string(),
                version: 1,
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
                created_by: "test_user".to_string(),
                updated_by: "test_user".to_string(),
                description: None,
                tags: vec![],
                is_encrypted: false,
            }),
        }))
    }

    async fn get_config(
        &self,
        request: Request<GetConfigRequest>,
    ) -> Result<Response<GetConfigResponse>, Status> {
        // TODO: Implement get_config
        Ok(Response::new(GetConfigResponse {
            config: Some(config_service::Config {
                id: uuid::Uuid::new_v4().to_string(),
                key: request.into_inner().key,
                value: "test_value".to_string(),
                version: 1,
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
                created_by: "test_user".to_string(),
                updated_by: "test_user".to_string(),
                description: None,
                tags: vec![],
                is_encrypted: false,
            }),
        }))
    }

    async fn update_config(
        &self,
        request: Request<UpdateConfigRequest>,
    ) -> Result<Response<UpdateConfigResponse>, Status> {
        // TODO: Implement update_config
        Ok(Response::new(UpdateConfigResponse {
            config: Some(config_service::Config {
                id: uuid::Uuid::new_v4().to_string(),
                key: request.into_inner().key,
                value: "test_value".to_string(),
                version: 1,
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
                created_by: "test_user".to_string(),
                updated_by: "test_user".to_string(),
                description: None,
                tags: vec![],
                is_encrypted: false,
            }),
        }))
    }

    async fn delete_config(
        &self,
        request: Request<DeleteConfigRequest>,
    ) -> Result<Response<DeleteConfigResponse>, Status> {
        // TODO: Implement delete_config
        Ok(Response::new(DeleteConfigResponse {}))
    }

    async fn list_configs(
        &self,
        request: Request<ListConfigsRequest>,
    ) -> Result<Response<ListConfigsResponse>, Status> {
        // TODO: Implement list_configs
        Ok(Response::new(ListConfigsResponse {
            configs: vec![],
        }))
    }

    async fn create_namespace(
        &self,
        request: Request<CreateNamespaceRequest>,
    ) -> Result<Response<CreateNamespaceResponse>, Status> {
        // TODO: Implement create_namespace
        Ok(Response::new(CreateNamespaceResponse {
            namespace: Some(config_service::Namespace {
                id: uuid::Uuid::new_v4().to_string(),
                name: request.into_inner().name,
                description: None,
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
                created_by: "test_user".to_string(),
                updated_by: "test_user".to_string(),
            }),
        }))
    }

    async fn get_namespace(
        &self,
        request: Request<GetNamespaceRequest>,
    ) -> Result<Response<GetNamespaceResponse>, Status> {
        // TODO: Implement get_namespace
        Ok(Response::new(GetNamespaceResponse {
            namespace: Some(config_service::Namespace {
                id: uuid::Uuid::new_v4().to_string(),
                name: request.into_inner().name,
                description: None,
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
                created_by: "test_user".to_string(),
                updated_by: "test_user".to_string(),
            }),
        }))
    }

    async fn update_namespace(
        &self,
        request: Request<UpdateNamespaceRequest>,
    ) -> Result<Response<UpdateNamespaceResponse>, Status> {
        // TODO: Implement update_namespace
        Ok(Response::new(UpdateNamespaceResponse {
            namespace: Some(config_service::Namespace {
                id: uuid::Uuid::new_v4().to_string(),
                name: request.into_inner().name,
                description: None,
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
                created_by: "test_user".to_string(),
                updated_by: "test_user".to_string(),
            }),
        }))
    }

    async fn delete_namespace(
        &self,
        request: Request<DeleteNamespaceRequest>,
    ) -> Result<Response<DeleteNamespaceResponse>, Status> {
        // TODO: Implement delete_namespace
        Ok(Response::new(DeleteNamespaceResponse {}))
    }

    async fn list_namespaces(
        &self,
        request: Request<ListNamespacesRequest>,
    ) -> Result<Response<ListNamespacesResponse>, Status> {
        // TODO: Implement list_namespaces
        Ok(Response::new(ListNamespacesResponse {
            namespaces: vec![],
        }))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        // TODO: Implement create_user
        Ok(Response::new(CreateUserResponse {
            user: Some(config_service::User {
                id: uuid::Uuid::new_v4().to_string(),
                username: request.into_inner().username,
                email: "test@example.com".to_string(),
                is_active: true,
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
                last_login: None,
                roles: vec![],
            }),
        }))
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        // TODO: Implement get_user
        Ok(Response::new(GetUserResponse {
            user: Some(config_service::User {
                id: request.into_inner().id,
                username: "test_user".to_string(),
                email: "test@example.com".to_string(),
                is_active: true,
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
                last_login: None,
                roles: vec![],
            }),
        }))
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UpdateUserResponse>, Status> {
        // TODO: Implement update_user
        Ok(Response::new(UpdateUserResponse {
            user: Some(config_service::User {
                id: request.into_inner().id,
                username: "test_user".to_string(),
                email: "test@example.com".to_string(),
                is_active: true,
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
                last_login: None,
                roles: vec![],
            }),
        }))
    }

    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, Status> {
        // TODO: Implement delete_user
        Ok(Response::new(DeleteUserResponse {}))
    }

    async fn list_users(
        &self,
        request: Request<ListUsersRequest>,
    ) -> Result<Response<ListUsersResponse>, Status> {
        // TODO: Implement list_users
        Ok(Response::new(ListUsersResponse {
            users: vec![],
        }))
    }

    async fn create_role(
        &self,
        request: Request<CreateRoleRequest>,
    ) -> Result<Response<CreateRoleResponse>, Status> {
        // TODO: Implement create_role
        Ok(Response::new(CreateRoleResponse {
            role: Some(config_service::Role {
                id: uuid::Uuid::new_v4().to_string(),
                name: request.into_inner().name,
                description: None,
                permissions: vec![],
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            }),
        }))
    }

    async fn get_role(
        &self,
        request: Request<GetRoleRequest>,
    ) -> Result<Response<GetRoleResponse>, Status> {
        // TODO: Implement get_role
        Ok(Response::new(GetRoleResponse {
            role: Some(config_service::Role {
                id: request.into_inner().id,
                name: "test_role".to_string(),
                description: None,
                permissions: vec![],
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            }),
        }))
    }

    async fn update_role(
        &self,
        request: Request<UpdateRoleRequest>,
    ) -> Result<Response<UpdateRoleResponse>, Status> {
        // TODO: Implement update_role
        Ok(Response::new(UpdateRoleResponse {
            role: Some(config_service::Role {
                id: request.into_inner().id,
                name: "test_role".to_string(),
                description: None,
                permissions: vec![],
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            }),
        }))
    }

    async fn delete_role(
        &self,
        request: Request<DeleteRoleRequest>,
    ) -> Result<Response<DeleteRoleResponse>, Status> {
        // TODO: Implement delete_role
        Ok(Response::new(DeleteRoleResponse {}))
    }

    async fn list_roles(
        &self,
        request: Request<ListRolesRequest>,
    ) -> Result<Response<ListRolesResponse>, Status> {
        // TODO: Implement list_roles
        Ok(Response::new(ListRolesResponse {
            roles: vec![],
        }))
    }

    async fn create_permission(
        &self,
        request: Request<CreatePermissionRequest>,
    ) -> Result<Response<CreatePermissionResponse>, Status> {
        // TODO: Implement create_permission
        Ok(Response::new(CreatePermissionResponse {
            permission: Some(config_service::Permission {
                id: uuid::Uuid::new_v4().to_string(),
                name: request.into_inner().name,
                description: None,
                resource: "test_resource".to_string(),
                action: "test_action".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            }),
        }))
    }

    async fn get_permission(
        &self,
        request: Request<GetPermissionRequest>,
    ) -> Result<Response<GetPermissionResponse>, Status> {
        // TODO: Implement get_permission
        Ok(Response::new(GetPermissionResponse {
            permission: Some(config_service::Permission {
                id: request.into_inner().id,
                name: "test_permission".to_string(),
                description: None,
                resource: "test_resource".to_string(),
                action: "test_action".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            }),
        }))
    }

    async fn update_permission(
        &self,
        request: Request<UpdatePermissionRequest>,
    ) -> Result<Response<UpdatePermissionResponse>, Status> {
        // TODO: Implement update_permission
        Ok(Response::new(UpdatePermissionResponse {
            permission: Some(config_service::Permission {
                id: request.into_inner().id,
                name: "test_permission".to_string(),
                description: None,
                resource: "test_resource".to_string(),
                action: "test_action".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            }),
        }))
    }

    async fn delete_permission(
        &self,
        request: Request<DeletePermissionRequest>,
    ) -> Result<Response<DeletePermissionResponse>, Status> {
        // TODO: Implement delete_permission
        Ok(Response::new(DeletePermissionResponse {}))
    }

    async fn list_permissions(
        &self,
        request: Request<ListPermissionsRequest>,
    ) -> Result<Response<ListPermissionsResponse>, Status> {
        // TODO: Implement list_permissions
        Ok(Response::new(ListPermissionsResponse {
            permissions: vec![],
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_grpc_server_creation() {
        let config = ApiConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            grpc_port: 50051,
            tls: None,
        };

        let db = Arc::new(Database::new(&crate::config::DatabaseConfig {
            host: "localhost".to_string(),
            port: 5432,
            username: "postgres".to_string(),
            password: "postgres".to_string(),
            database: "config_center_test".to_string(),
            max_connections: 10,
            idle_timeout: 300,
        }).await.unwrap());

        let cache = Arc::new(RedisCache::new(&crate::config::RedisConfig {
            host: "localhost".to_string(),
            port: 6379,
            password: None,
            database: 0,
            pool_size: 10,
            connection_timeout: 5,
        }).await.unwrap());

        let raft = Arc::new(RaftNode::new(&crate::config::RaftConfig {
            node_id: "node1".to_string(),
            data_dir: std::path::PathBuf::from("/tmp/raft"),
            peers: vec!["node2".to_string(), "node3".to_string()],
            heartbeat_interval: 100,
            election_timeout: 1000,
        }).await.unwrap());

        let auth = Arc::new(Auth::new(&crate::config::AuthConfig {
            jwt_secret: "test_secret".to_string(),
            token_expiration: 3600,
            password_hash_cost: 10,
            rbac_model: std::path::PathBuf::from("config/rbac_model.conf"),
        }).unwrap());

        let monitor = Arc::new(Monitor::new(&crate::config::MonitorConfig {
            metrics_port: 9090,
            prometheus_path: "/metrics".to_string(),
            alert_rules: std::path::PathBuf::from("config/alert_rules.yml"),
        }).unwrap());

        let audit = Arc::new(Audit::new(&crate::config::AuditConfig {
            log_dir: std::path::PathBuf::from("/tmp/audit"),
            max_size: 1024,
            max_files: 3,
            compression: false,
        }).await.unwrap());

        let server = GrpcServer::new(
            config,
            db,
            cache,
            raft,
            auth,
            monitor,
            audit,
        );

        assert!(server.start().await.is_ok());
    }
}
