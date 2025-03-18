mod api;
mod config;
mod db;
mod cache;
mod raft;
mod auth;
// mod monitor;
mod audit;
mod types;
mod utils;
mod proto;

use std::sync::Arc;

use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_ansi(true)
        .pretty()
        .finish();
        
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting configuration center...");

    // Load configuration
    let config = config::Config::load()?;
    info!("Configuration loaded successfully");

    // Initialize database connection
    let db = db::Database::new(&config.database).await?;
    info!("Database connection established");

    // Initialize Redis cache
    let cache = cache::RedisCache::new(&config.redis).await?;
    info!("Redis cache initialized");

    // Initialize Raft cluster
    let raft = raft::RaftNode::new(config.raft).await?;
    info!("Raft cluster initialized");

    // Initialize authentication
    let auth = auth::Auth::new(&config.auth)?;
    info!("Authentication initialized");

    // Initialize monitoring
    // let monitor = monitor::Monitor::new(&config.monitor)?;
    // info!("Monitoring initialized");

    // Initialize audit logging
    let audit = audit::Audit::new(&config.audit).await?;
    info!("Audit logging initialized");
    // Start API server
    let api = api::rest::RestServer::new(
        config.api,
        Arc::new(db),
        Arc::new(cache),
        Arc::new(raft),
        Arc::new(auth),
        // monitor,
        Arc::new(audit),
    );
    api.start().await?;

    Ok(())
}
