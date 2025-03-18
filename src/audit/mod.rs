use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncWriteExt, BufWriter};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::config::AuditConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditEvent {
    pub timestamp: DateTime<Utc>,
    pub user_id: String,
    pub action: String,
    pub resource: String,
    pub details: serde_json::Value,
    pub ip_address: String,
}

#[derive(Debug, Clone)]
pub struct Audit {
    config: AuditConfig,
    writer: Arc<Mutex<BufWriter<File>>>,
}

impl Audit {
    pub async fn new(config: &AuditConfig) -> Result<Self> {
        // Create log directory if it doesn't exist
        tokio::fs::create_dir_all(&config.log_dir).await?;

        // Open log file
        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(config.log_dir.join("audit.log"))
            .await?;
        Ok(Self {
            config: config.clone(),
            writer: Arc::new(Mutex::new(BufWriter::new(log_file))),
        })
    }

    pub async fn log_event(&self, event: AuditEvent) -> Result<()> {
        let json = serde_json::to_string(&event)?;
        let mut writer = self.writer.lock().await;
        writer.write_all(json.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        writer.flush().await?;

        // Rotate log file if size exceeds max_size
        if writer.get_ref().metadata().await?.len() > self.config.max_size {
            self.rotate_log_file().await?;
        }

        Ok(())
    }

    async fn rotate_log_file(&self) -> Result<()> {
        // Close current file
        let mut writer = self.writer.lock().await;
        writer.flush().await?;
        let writer = std::mem::replace(&mut *writer, 
            BufWriter::new(File::create("/dev/null").await?));
        drop(writer);

        // Rename current file with timestamp
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
        let old_path = self.config.log_dir.join("audit.log");
        let new_path = self.config.log_dir.join(format!("audit_{}.log", timestamp));
        tokio::fs::rename(old_path, new_path).await?;

        // Create new log file
        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.config.log_dir.join("audit.log"))
            .await?;

        let mut writer = self.writer.lock().await;
        *writer = BufWriter::new(log_file);

        // Clean up old log files if count exceeds max_files
        self.cleanup_old_logs().await?;

        Ok(())
    }

    async fn cleanup_old_logs(&self) -> Result<()> {
        let mut entries = tokio::fs::read_dir(&self.config.log_dir).await?;
        let mut log_files = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            if entry.file_name().to_string_lossy().starts_with("audit_") {
                log_files.push(entry.path());
            }
        }

        // Get metadata for all files
        let mut file_metadata = Vec::new();
        for path in &log_files {
            let metadata = tokio::fs::metadata(path).await?;
            file_metadata.push((path.clone(), metadata));
        }

        // Sort by modification time
        file_metadata.sort_by(|a, b| {
            b.1.modified().unwrap().cmp(&a.1.modified().unwrap())
        });

        // Remove old files
        for (path, _) in file_metadata.iter().skip(self.config.max_files as usize) {
            tokio::fs::remove_file(path).await?;
        }

        Ok(())
    }

    pub async fn query_events(
        &self,
        user_id: Option<&str>,
        action: Option<&str>,
        resource: Option<&str>,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<Vec<AuditEvent>> {
        let mut events = Vec::new();
        let mut entries = tokio::fs::read_dir(&self.config.log_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            if entry.file_name().to_string_lossy().starts_with("audit") {
                let content = tokio::fs::read_to_string(entry.path()).await?;
                for line in content.lines() {
                    let event: AuditEvent = serde_json::from_str(line)?;

                    // Apply filters
                    if let Some(uid) = user_id {
                        if event.user_id != uid {
                            continue;
                        }
                    }
                    if let Some(act) = action {
                        if event.action != act {
                            continue;
                        }
                    }
                    if let Some(res) = resource {
                        if event.resource != res {
                            continue;
                        }
                    }
                    if let Some(start) = start_time {
                        if event.timestamp < start {
                            continue;
                        }
                    }
                    if let Some(end) = end_time {
                        if event.timestamp > end {
                            continue;
                        }
                    }

                    events.push(event);
                }
            }
        }

        // Sort by timestamp
        events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_audit_logging() {
        let temp_dir = tempdir().unwrap();
        let config = AuditConfig {
            log_dir: temp_dir.path().to_path_buf(),
            max_size: 1024,
            max_files: 3,
            compression: false,
        };

        let audit = Audit::new(&config).await.unwrap();

        let event = AuditEvent {
            timestamp: Utc::now(),
            user_id: "test_user".to_string(),
            action: "create".to_string(),
            resource: "config".to_string(),
            details: serde_json::json!({"key": "value"}),
            ip_address: "127.0.0.1".to_string(),
        };

        assert!(audit.log_event(event).await.is_ok());
    }

    #[tokio::test]
    async fn test_audit_query() {
        let temp_dir = tempdir().unwrap();
        let config = AuditConfig {
            log_dir: temp_dir.path().to_path_buf(),
            max_size: 1024,
            max_files: 3,
            compression: false,
        };

        let audit = Audit::new(&config).await.unwrap();

        // Log some test events
        let events = vec![
            AuditEvent {
                timestamp: Utc::now(),
                user_id: "user1".to_string(),
                action: "create".to_string(),
                resource: "config1".to_string(),
                details: serde_json::json!({"key": "value1"}),
                ip_address: "127.0.0.1".to_string(),
            },
            AuditEvent {
                timestamp: Utc::now(),
                user_id: "user2".to_string(),
                action: "update".to_string(),
                resource: "config2".to_string(),
                details: serde_json::json!({"key": "value2"}),
                ip_address: "127.0.0.2".to_string(),
            },
        ];

        for event in events {
            audit.log_event(event).await.unwrap();
        }

        // Query events
        let results = audit
            .query_events(Some("user1"), None, None, None, None)
            .await
            .unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].user_id, "user1");
    }
}
