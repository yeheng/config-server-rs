use async_trait::async_trait;
use config_common::{AuditLog, Result};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

pub mod repository;
pub mod service;

/// Audit service trait
#[async_trait]
pub trait AuditService: Send + Sync {
    /// Record an audit log entry
    async fn record(&self, log: AuditLog) -> Result<()>;

    /// Get audit logs with filters
    async fn get_logs(
        &self,
        filter: AuditFilter,
        page_size: i32,
        page_number: i32,
    ) -> Result<(Vec<AuditLog>, i32)>;
}

/// Audit log filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditFilter {
    pub user: Option<String>,
    pub action: Option<String>,
    pub resource: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

/// Database-backed audit service implementation
pub struct DbAuditService {
    pool: Arc<PgPool>,
}

impl DbAuditService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuditService for DbAuditService {
    async fn record(&self, log: AuditLog) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO audit_logs (id, user_id, action, resource, details, timestamp)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            log.id,
            log.user,
            log.action,
            log.resource,
            log.details,
            log.timestamp,
        )
        .execute(&*self.pool)
        .await
        .map_err(|e| config_common::Error::Database(e.to_string()))?;

        Ok(())
    }

    async fn get_logs(
        &self,
        filter: AuditFilter,
        page_size: i32,
        page_number: i32,
    ) -> Result<(Vec<AuditLog>, i32)> {
        let offset = (page_number - 1) * page_size;

        let mut query = sqlx::QueryBuilder::new(
            "SELECT id, user_id, action, resource, details, timestamp FROM audit_logs WHERE 1=1",
        );

        if let Some(user) = filter.user {
            query.push(" AND user_id = ");
            query.push_bind(user);
        }

        if let Some(action) = filter.action {
            query.push(" AND action = ");
            query.push_bind(action);
        }

        if let Some(resource) = filter.resource {
            query.push(" AND resource = ");
            query.push_bind(resource);
        }

        if let Some(start_time) = filter.start_time {
            query.push(" AND timestamp >= ");
            query.push_bind(start_time);
        }

        if let Some(end_time) = filter.end_time {
            query.push(" AND timestamp <= ");
            query.push_bind(end_time);
        }

        // Get total count
        let count_query = query
            .clone()
            .push(" ORDER BY timestamp DESC")
            .build_query_as::<(i64,)>();

        let total = sqlx::query_as::<_, (i64,)>(
            &format!("SELECT COUNT(*) FROM ({}) AS t", count_query.sql()),
        )
        .bind(count_query.into_arguments())
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| config_common::Error::Database(e.to_string()))?
        .0 as i32;

        // Get paginated results
        query.push(" ORDER BY timestamp DESC LIMIT ");
        query.push_bind(page_size);
        query.push(" OFFSET ");
        query.push_bind(offset);

        let logs = query
            .build_query_as::<AuditLog>()
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| config_common::Error::Database(e.to_string()))?;

        Ok((logs, total))
    }
}

/// Initialize audit database schema
pub async fn init_schema(pool: &PgPool) -> Result<()> {
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS audit_logs (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            action TEXT NOT NULL,
            resource TEXT NOT NULL,
            details TEXT NOT NULL,
            timestamp BIGINT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS audit_logs_user_id_idx ON audit_logs (user_id);
        CREATE INDEX IF NOT EXISTS audit_logs_action_idx ON audit_logs (action);
        CREATE INDEX IF NOT EXISTS audit_logs_resource_idx ON audit_logs (resource);
        CREATE INDEX IF NOT EXISTS audit_logs_timestamp_idx ON audit_logs (timestamp);
        "#
    )
    .execute(pool)
    .await
    .map_err(|e| config_common::Error::Database(e.to_string()))?;

    Ok(())
} 