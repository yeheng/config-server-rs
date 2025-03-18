use crate::{
    audit::Audit,
    auth::Auth,
    cache::RedisCache,
    config::ApiConfig,
    db::Database,
    raft::RaftNode,
    types::{ConfigItem, ConfigNamespace, Permission, Role, User},
};
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use std::sync::Arc;

pub struct RestServer {
    config: ApiConfig,
    db: Arc<Database>,
    cache: Arc<RedisCache>,
    raft: Arc<RaftNode>,
    auth: Arc<Auth>,
    audit: Arc<Audit>,
}

impl RestServer {
    pub fn new(
        config: ApiConfig,
        db: Arc<Database>,
        cache: Arc<RedisCache>,
        raft: Arc<RaftNode>,
        auth: Arc<Auth>,
        audit: Arc<Audit>,
    ) -> Self {
        Self {
            config,
            db,
            cache,
            raft,
            auth,
            audit,
        }
    }
    pub async fn start(&self) -> Result<()> {
        let db = self.db.clone();
        let cache = self.cache.clone();
        let raft = self.raft.clone();
        let auth = self.auth.clone();
        let audit = self.audit.clone();

        let server = HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .wrap(Cors::default())
                .app_data(web::Data::new(db.clone()))
                .app_data(web::Data::new(cache.clone()))
                .app_data(web::Data::new(raft.clone()))
                .app_data(web::Data::new(auth.clone()))
                .app_data(web::Data::new(audit.clone()))
                .service(
                    web::scope("/api/v1")
                        .service(
                            web::scope("/configs")
                                .route("", web::post().to(create_config))
                                .route("/{key}", web::get().to(get_config))
                                .route("/{key}", web::put().to(update_config))
                                .route("/{key}", web::delete().to(delete_config))
                                .route("/namespace/{namespace}", web::get().to(list_configs)),
                        )
                        .service(
                            web::scope("/namespaces")
                                .route("", web::post().to(create_namespace))
                                .route("/{name}", web::get().to(get_namespace))
                                .route("/{name}", web::put().to(update_namespace))
                                .route("/{name}", web::delete().to(delete_namespace))
                                .route("", web::get().to(list_namespaces)),
                        )
                        .service(
                            web::scope("/users")
                                .route("", web::post().to(create_user))
                                .route("/{id}", web::get().to(get_user))
                                .route("/{id}", web::put().to(update_user))
                                .route("/{id}", web::delete().to(delete_user))
                                .route("", web::get().to(list_users)),
                        )
                        .service(
                            web::scope("/roles")
                                .route("", web::post().to(create_role))
                                .route("/{id}", web::get().to(get_role))
                                .route("/{id}", web::put().to(update_role))
                                .route("/{id}", web::delete().to(delete_role))
                                .route("", web::get().to(list_roles)),
                        )
                        .service(
                            web::scope("/permissions")
                                .route("", web::post().to(create_permission))
                                .route("/{id}", web::get().to(get_permission))
                                .route("/{id}", web::put().to(update_permission))
                                .route("/{id}", web::delete().to(delete_permission))
                                .route("", web::get().to(list_permissions)),
                        ),
                )
        });

        server
            .bind((self.config.host.clone(), self.config.port))?
            .run()
            .await?;

        Ok(())
    }
}

// Configuration endpoints
async fn create_config(
    db: web::Data<Arc<Database>>,
    cache: web::Data<Arc<RedisCache>>,
    raft: web::Data<Arc<RaftNode>>,
    auth: web::Data<Arc<Auth>>,
    audit: web::Data<Arc<Audit>>,
    config: web::Json<ConfigItem>,
) -> impl Responder {
    // TODO: Implement create_config
    HttpResponse::Created().json(config.0)
}

async fn get_config(
    db: web::Data<Arc<Database>>,
    cache: web::Data<Arc<RedisCache>>,
    key: web::Path<String>,
) -> impl Responder {
    // TODO: Implement get_config
    HttpResponse::Ok().json(ConfigItem {
        id: uuid::Uuid::new_v4(),
        key: key.into_inner(),
        value: "test_value".to_string(),
        version: 1,
        created_at: std::time::SystemTime::now(),
        updated_at: std::time::SystemTime::now(),
        created_by: "test_user".to_string(),
        updated_by: "test_user".to_string(),
        description: None,
        tags: vec![],
        is_encrypted: false,
    })
}

async fn update_config(
    db: web::Data<Arc<Database>>,
    cache: web::Data<Arc<RedisCache>>,
    raft: web::Data<Arc<RaftNode>>,
    auth: web::Data<Arc<Auth>>,

    audit: web::Data<Arc<Audit>>,
    key: web::Path<String>,
    config: web::Json<ConfigItem>,
) -> impl Responder {
    // TODO: Implement update_config
    HttpResponse::Ok().json(config.0)
}

async fn delete_config(
    db: web::Data<Arc<Database>>,
    cache: web::Data<Arc<RedisCache>>,
    raft: web::Data<Arc<RaftNode>>,
    auth: web::Data<Arc<Auth>>,

    audit: web::Data<Arc<Audit>>,
    key: web::Path<String>,
) -> impl Responder {
    // TODO: Implement delete_config
    HttpResponse::NoContent()
}

async fn list_configs(
    db: web::Data<Arc<Database>>,
    cache: web::Data<Arc<RedisCache>>,
    namespace: web::Path<String>,
) -> impl Responder {
    // TODO: Implement list_configs
    HttpResponse::Ok().json(Vec::<ConfigItem>::new())
}

// Namespace endpoints
async fn create_namespace(
    db: web::Data<Arc<Database>>,
    raft: web::Data<Arc<RaftNode>>,
    auth: web::Data<Arc<Auth>>,

    audit: web::Data<Arc<Audit>>,
    namespace: web::Json<ConfigNamespace>,
) -> impl Responder {
    // TODO: Implement create_namespace
    HttpResponse::Created().json(namespace.0)
}

async fn get_namespace(db: web::Data<Arc<Database>>, name: web::Path<String>) -> impl Responder {
    // TODO: Implement get_namespace
    HttpResponse::Ok().json(ConfigNamespace {
        id: uuid::Uuid::new_v4(),
        name: name.into_inner(),
        description: None,
        created_at: std::time::SystemTime::now(),
        updated_at: std::time::SystemTime::now(),
        created_by: "test_user".to_string(),
        updated_by: "test_user".to_string(),
    })
}

async fn update_namespace(
    db: web::Data<Arc<Database>>,
    raft: web::Data<Arc<RaftNode>>,
    auth: web::Data<Arc<Auth>>,

    audit: web::Data<Arc<Audit>>,
    name: web::Path<String>,
    namespace: web::Json<ConfigNamespace>,
) -> impl Responder {
    // TODO: Implement update_namespace
    HttpResponse::Ok().json(namespace.0)
}

async fn delete_namespace(
    db: web::Data<Arc<Database>>,
    raft: web::Data<Arc<RaftNode>>,
    auth: web::Data<Arc<Auth>>,

    audit: web::Data<Arc<Audit>>,
    name: web::Path<String>,
) -> impl Responder {
    // TODO: Implement delete_namespace
    HttpResponse::NoContent()
}

async fn list_namespaces(db: web::Data<Arc<Database>>) -> impl Responder {
    // TODO: Implement list_namespaces
    HttpResponse::Ok().json(Vec::<ConfigNamespace>::new())
}

// User endpoints
async fn create_user(
    db: web::Data<Arc<Database>>,
    auth: web::Data<Arc<Auth>>,

    audit: web::Data<Arc<Audit>>,
    user: web::Json<User>,
) -> impl Responder {
    // TODO: Implement create_user
    HttpResponse::Created().json(user.0)
}

async fn get_user(db: web::Data<Arc<Database>>, id: web::Path<uuid::Uuid>) -> impl Responder {
    // TODO: Implement get_user
    HttpResponse::Ok().json(User {
        id: id.into_inner(),
        username: "test_user".to_string(),
        email: "test@example.com".to_string(),
        password_hash: "".to_string(),
        is_active: true,
        created_at: std::time::SystemTime::now(),
        updated_at: std::time::SystemTime::now(),
        last_login: None,
        roles: vec![],
    })
}

async fn update_user(
    db: web::Data<Arc<Database>>,
    auth: web::Data<Arc<Auth>>,

    audit: web::Data<Arc<Audit>>,
    id: web::Path<uuid::Uuid>,
    user: web::Json<User>,
) -> impl Responder {
    // TODO: Implement update_user
    HttpResponse::Ok().json(user.0)
}

async fn delete_user(
    db: web::Data<Arc<Database>>,
    auth: web::Data<Arc<Auth>>,

    audit: web::Data<Arc<Audit>>,
    id: web::Path<uuid::Uuid>,
) -> impl Responder {
    // TODO: Implement delete_user
    HttpResponse::NoContent()
}

async fn list_users(db: web::Data<Arc<Database>>) -> impl Responder {
    // TODO: Implement list_users
    HttpResponse::Ok().json(Vec::<User>::new())
}

// Role endpoints
async fn create_role(
    db: web::Data<Arc<Database>>,
    auth: web::Data<Arc<Auth>>,

    audit: web::Data<Arc<Audit>>,
    role: web::Json<Role>,
) -> impl Responder {
    // TODO: Implement create_role
    HttpResponse::Created().json(role.0)
}

async fn get_role(db: web::Data<Arc<Database>>, id: web::Path<uuid::Uuid>) -> impl Responder {
    // TODO: Implement get_role
    HttpResponse::Ok().json(Role {
        id: id.into_inner(),
        name: "test_role".to_string(),
        description: None,
        permissions: vec![],
        created_at: std::time::SystemTime::now(),
        updated_at: std::time::SystemTime::now(),
    })
}

async fn update_role(
    db: web::Data<Arc<Database>>,
    auth: web::Data<Arc<Auth>>,

    audit: web::Data<Arc<Audit>>,
    id: web::Path<uuid::Uuid>,
    role: web::Json<Role>,
) -> impl Responder {
    // TODO: Implement update_role
    HttpResponse::Ok().json(role.0)
}

async fn delete_role(
    db: web::Data<Arc<Database>>,
    auth: web::Data<Arc<Auth>>,

    audit: web::Data<Arc<Audit>>,
    id: web::Path<uuid::Uuid>,
) -> impl Responder {
    // TODO: Implement delete_role
    HttpResponse::NoContent()
}

async fn list_roles(db: web::Data<Arc<Database>>) -> impl Responder {
    // TODO: Implement list_roles
    HttpResponse::Ok().json(Vec::<Role>::new())
}

// Permission endpoints
async fn create_permission(
    db: web::Data<Arc<Database>>,
    auth: web::Data<Arc<Auth>>,

    audit: web::Data<Arc<Audit>>,
    permission: web::Json<Permission>,
) -> impl Responder {
    // TODO: Implement create_permission
    HttpResponse::Created().json(permission.0)
}

async fn get_permission(db: web::Data<Arc<Database>>, id: web::Path<uuid::Uuid>) -> impl Responder {
    // TODO: Implement get_permission
    HttpResponse::Ok().json(Permission {
        id: id.into_inner(),
        name: "test_permission".to_string(),
        description: None,
        resource: "test_resource".to_string(),
        action: "test_action".to_string(),
        created_at: std::time::SystemTime::now(),
        updated_at: std::time::SystemTime::now(),
    })
}

async fn update_permission(
    db: web::Data<Arc<Database>>,
    auth: web::Data<Arc<Auth>>,

    audit: web::Data<Arc<Audit>>,
    id: web::Path<uuid::Uuid>,
    permission: web::Json<Permission>,
) -> impl Responder {
    // TODO: Implement update_permission
    HttpResponse::Ok().json(permission.0)
}

async fn delete_permission(
    db: web::Data<Arc<Database>>,
    auth: web::Data<Arc<Auth>>,

    audit: web::Data<Arc<Audit>>,
    id: web::Path<uuid::Uuid>,
) -> impl Responder {
    // TODO: Implement delete_permission
    HttpResponse::NoContent()
}

async fn list_permissions(db: web::Data<Arc<Database>>) -> impl Responder {
    // TODO: Implement list_permissions
    HttpResponse::Ok().json(Vec::<Permission>::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_web::test]
    async fn test_rest_server_creation() {
        let config = ApiConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            grpc_port: 50051,
            tls: None,
        };

        let db = Arc::new(
            Database::new(&crate::config::DatabaseConfig {
                host: "localhost".to_string(),
                port: 5432,
                username: "postgres".to_string(),
                password: "postgres".to_string(),
                database: "config_center_test".to_string(),
                max_connections: 10,
                idle_timeout: 300,
            })
            .await
            .unwrap(),
        );

        let cache = Arc::new(
            RedisCache::new(&crate::config::RedisConfig {
                host: "localhost".to_string(),
                port: 6379,
                password: None,
                database: 0,
                pool_size: 10,
                connection_timeout: 5,
            })
            .await
            .unwrap(),
        );

        let raft = Arc::new(
            RaftNode::new(crate::config::RaftConfig {
                node_id: "node1".to_string(),
                data_dir: std::path::PathBuf::from("/tmp/raft"),
                peers: vec!["node2".to_string(), "node3".to_string()],
                heartbeat_interval: 100,
                election_timeout: 1000,
            })
            .await
            .unwrap(),
        );

        let auth = Arc::new(
            Auth::new(&crate::config::AuthConfig {
                jwt_secret: "test_secret".to_string(),
                token_expiration: 3600,
                password_hash_cost: 10,
                rbac_model: std::path::PathBuf::from("config/rbac_model.conf"),
            })
            .unwrap(),
        );

        // let monitor = Arc::new(Monitor::new(&crate::config::MonitorConfig {
        //     metrics_port: 9090,
        //     prometheus_path: "/metrics".to_string(),
        //     alert_rules: std::path::PathBuf::from("config/alert_rules.yml"),
        // }).unwrap());

        let audit = Arc::new(
            Audit::new(&crate::config::AuditConfig {
                log_dir: std::path::PathBuf::from("/tmp/audit"),
                max_size: 1024,
                max_files: 3,
                compression: false,
            })
            .await
            .unwrap(),
        );

        let server = RestServer::new(config, db, cache, raft, auth, audit);

        assert!(server.start().await.is_ok());
    }
}
