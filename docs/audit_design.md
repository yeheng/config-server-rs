# 审计日志设计文档

## 1. 概述

本设计文档详细描述了分布式配置中心的审计日志系统，用于记录和追踪系统中的所有重要操作，确保系统的安全性和可追溯性。

### 1.1 设计目标

- **完整性：** 记录所有关键操作
- **准确性：** 确保日志信息的准确性和一致性
- **可追溯性：** 支持操作追踪和问题定位
- **性能：** 不影响系统正常性能
- **安全性：** 防止日志被篡改和泄露

### 1.2 核心组件

- **日志收集：** 审计日志收集器
- **日志存储：** PostgreSQL + Elasticsearch
- **日志分析：** 日志分析工具
- **日志展示：** 审计日志查询界面

## 2. 审计日志模型

### 2.1 基础模型

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: String,                    // 日志ID
    pub timestamp: DateTime<Utc>,      // 操作时间
    pub user_id: String,               // 操作用户ID
    pub username: String,              // 操作用户名
    pub operation: Operation,          // 操作类型
    pub resource_type: ResourceType,   // 资源类型
    pub resource_id: String,           // 资源ID
    pub details: Value,                // 详细信息
    pub ip_address: String,            // IP地址
    pub user_agent: String,            // 用户代理
    pub trace_id: String,              // 追踪ID
    pub status: OperationStatus,       // 操作状态
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Operation {
    Create,    // 创建
    Read,      // 读取
    Update,    // 更新
    Delete,    // 删除
    Login,     // 登录
    Logout,    // 登出
    Grant,     // 授权
    Revoke,    // 撤销授权
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceType {
    Config,        // 配置
    Namespace,     // 命名空间
    Department,    // 部门
    Application,   // 应用
    Domain,        // 域
    Environment,   // 环境
    User,          // 用户
    Role,          // 角色
    Permission,    // 权限
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OperationStatus {
    Success,   // 成功
    Failed,    // 失败
    Pending,   // 待处理
}
```

### 2.2 详细模型

1. **配置操作日志：**

   ```rust
   #[derive(Debug, Serialize, Deserialize)]
   pub struct ConfigAuditLog {
       pub base: AuditLog,
       pub old_value: Option<String>,    // 旧值
       pub new_value: Option<String>,    // 新值
       pub version: Option<String>,      // 版本号
       pub is_encrypted: bool,           // 是否加密
       pub format: Option<String>,       // 格式
   }
   ```

2. **用户操作日志：**

   ```rust
   #[derive(Debug, Serialize, Deserialize)]
   pub struct UserAuditLog {
       pub base: AuditLog,
       pub action: UserAction,           // 用户动作
       pub target_user_id: Option<String>, // 目标用户ID
       pub role_changes: Option<Vec<RoleChange>>, // 角色变更
   }

   #[derive(Debug, Serialize, Deserialize)]
   pub enum UserAction {
       Create,           // 创建用户
       Update,           // 更新用户
       Delete,           // 删除用户
       ChangePassword,   // 修改密码
       ResetPassword,    // 重置密码
       Lock,            // 锁定用户
       Unlock,          // 解锁用户
   }

   #[derive(Debug, Serialize, Deserialize)]
   pub struct RoleChange {
       pub role_id: String,              // 角色ID
       pub action: RoleAction,           // 角色动作
   }

   #[derive(Debug, Serialize, Deserialize)]
   pub enum RoleAction {
       Add,    // 添加角色
       Remove, // 移除角色
   }
   ```

3. **权限操作日志：**

   ```rust
   #[derive(Debug, Serialize, Deserialize)]
   pub struct PermissionAuditLog {
       pub base: AuditLog,
       pub policy_type: PolicyType,      // 策略类型
       pub policy_id: String,            // 策略ID
       pub old_policy: Option<String>,   // 旧策略
       pub new_policy: Option<String>,   // 新策略
   }

   #[derive(Debug, Serialize, Deserialize)]
   pub enum PolicyType {
       RBAC,    // 基于角色的访问控制
       ABAC,    // 基于属性的访问控制
   }
   ```

## 3. 日志收集

### 3.1 日志收集器

```rust
pub struct AuditLogger {
    db: Arc<Database>,
    es: Arc<Elasticsearch>,
    buffer: Arc<Mutex<Vec<AuditLog>>>,
    batch_size: usize,
    flush_interval: Duration,
}

impl AuditLogger {
    pub async fn log(&self, log: AuditLog) -> Result<()> {
        // 添加到缓冲区
        let mut buffer = self.buffer.lock().await;
        buffer.push(log);

        // 如果缓冲区满，立即刷新
        if buffer.len() >= self.batch_size {
            self.flush().await?;
        }

        Ok(())
    }

    async fn flush(&self) -> Result<()> {
        let mut buffer = self.buffer.lock().await;
        if buffer.is_empty() {
            return Ok(());
        }

        // 批量写入数据库
        self.db.batch_insert(&buffer).await?;

        // 批量写入Elasticsearch
        self.es.batch_index(&buffer).await?;

        // 清空缓冲区
        buffer.clear();

        Ok(())
    }

    pub async fn start_flush_task(&self) {
        let logger = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(logger.flush_interval);
            loop {
                interval.tick().await;
                if let Err(e) = logger.flush().await {
                    error!("Failed to flush audit logs: {}", e);
                }
            }
        });
    }
}
```

### 3.2 日志过滤器

```rust
pub struct AuditLogFilter {
    pub min_level: LogLevel,
    pub exclude_operations: HashSet<Operation>,
    pub exclude_users: HashSet<String>,
    pub exclude_resources: HashSet<ResourceType>,
}

impl AuditLogFilter {
    pub fn should_log(&self, log: &AuditLog) -> bool {
        // 检查日志级别
        if log.level < self.min_level {
            return false;
        }

        // 检查操作类型
        if self.exclude_operations.contains(&log.operation) {
            return false;
        }

        // 检查用户
        if self.exclude_users.contains(&log.user_id) {
            return false;
        }

        // 检查资源类型
        if self.exclude_resources.contains(&log.resource_type) {
            return false;
        }

        true
    }
}
```

## 4. 日志存储

### 4.1 PostgreSQL存储

1. **表结构：**

   ```sql
   CREATE TABLE audit_logs (
       id VARCHAR(36) PRIMARY KEY,
       timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
       user_id VARCHAR(36) NOT NULL,
       username VARCHAR(255) NOT NULL,
       operation VARCHAR(50) NOT NULL,
       resource_type VARCHAR(50) NOT NULL,
       resource_id VARCHAR(36) NOT NULL,
       details JSONB,
       ip_address VARCHAR(45),
       user_agent TEXT,
       trace_id VARCHAR(36),
       status VARCHAR(20) NOT NULL,
       created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
   );

   -- 索引
   CREATE INDEX idx_audit_logs_timestamp ON audit_logs(timestamp);
   CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
   CREATE INDEX idx_audit_logs_operation ON audit_logs(operation);
   CREATE INDEX idx_audit_logs_resource_type ON audit_logs(resource_type);
   CREATE INDEX idx_audit_logs_resource_id ON audit_logs(resource_id);
   ```

2. **分区策略：**

   ```sql
   -- 按时间范围分区
   CREATE TABLE audit_logs (
       LIKE audit_logs_template INCLUDING ALL
   ) PARTITION BY RANGE (timestamp);

   -- 创建分区
   CREATE TABLE audit_logs_y2024m01 PARTITION OF audit_logs
       FOR VALUES FROM ('2024-01-01') TO ('2024-02-01');
   CREATE TABLE audit_logs_y2024m02 PARTITION OF audit_logs
       FOR VALUES FROM ('2024-02-01') TO ('2024-03-01');
   ```

### 4.2 Elasticsearch存储

1. **索引模板：**

   ```json
   {
     "template": "audit-logs-*",
     "settings": {
       "number_of_shards": 3,
       "number_of_replicas": 2,
       "refresh_interval": "5s"
     },
     "mappings": {
       "properties": {
         "timestamp": { "type": "date" },
         "user_id": { "type": "keyword" },
         "username": { "type": "keyword" },
         "operation": { "type": "keyword" },
         "resource_type": { "type": "keyword" },
         "resource_id": { "type": "keyword" },
         "details": { "type": "object" },
         "ip_address": { "type": "ip" },
         "user_agent": { "type": "text" },
         "trace_id": { "type": "keyword" },
         "status": { "type": "keyword" }
       }
     }
   }
   ```

2. **索引策略：**

   ```json
   {
     "policy": {
       "phases": {
         "hot": {
           "min_age": "0ms",
           "actions": {
             "rollover": {
               "max_age": "7d",
               "max_size": "50gb"
             }
           }
         },
         "warm": {
           "min_age": "7d",
           "actions": {
             "allocate": {
               "require": {
                 "data": "warm"
               }
             }
           }
         },
         "cold": {
           "min_age": "30d",
           "actions": {
             "allocate": {
               "require": {
                 "data": "cold"
               }
             }
           }
         },
         "delete": {
           "min_age": "90d",
           "actions": {
             "delete": {}
           }
         }
       }
     }
   }
   ```

## 5. 日志查询

### 5.1 查询接口

```rust
pub struct AuditLogQuery {
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub user_id: Option<String>,
    pub operation: Option<Operation>,
    pub resource_type: Option<ResourceType>,
    pub resource_id: Option<String>,
    pub status: Option<OperationStatus>,
    pub page: u32,
    pub page_size: u32,
}

impl AuditLogger {
    pub async fn query(&self, query: AuditLogQuery) -> Result<QueryResult<AuditLog>> {
        // 构建查询条件
        let mut conditions = Vec::new();
        
        if let Some(start_time) = query.start_time {
            conditions.push(format!("timestamp >= '{}'", start_time));
        }
        
        if let Some(end_time) = query.end_time {
            conditions.push(format!("timestamp <= '{}'", end_time));
        }
        
        if let Some(user_id) = query.user_id {
            conditions.push(format!("user_id = '{}'", user_id));
        }
        
        if let Some(operation) = query.operation {
            conditions.push(format!("operation = '{}'", operation));
        }
        
        if let Some(resource_type) = query.resource_type {
            conditions.push(format!("resource_type = '{}'", resource_type));
        }
        
        if let Some(resource_id) = query.resource_id {
            conditions.push(format!("resource_id = '{}'", resource_id));
        }
        
        if let Some(status) = query.status {
            conditions.push(format!("status = '{}'", status));
        }

        // 构建SQL查询
        let sql = format!(
            "SELECT * FROM audit_logs WHERE {} ORDER BY timestamp DESC LIMIT {} OFFSET {}",
            conditions.join(" AND "),
            query.page_size,
            query.page * query.page_size
        );

        // 执行查询
        let logs = self.db.query(&sql).await?;
        
        // 获取总数
        let count_sql = format!(
            "SELECT COUNT(*) FROM audit_logs WHERE {}",
            conditions.join(" AND ")
        );
        let total = self.db.query_one(&count_sql).await?;

        Ok(QueryResult {
            items: logs,
            total,
            page: query.page,
            page_size: query.page_size,
        })
    }
}
```

### 5.2 高级查询

1. **全文搜索：**

   ```rust
   pub struct FullTextQuery {
       pub query: String,
       pub fields: Vec<String>,
       pub operator: SearchOperator,
       pub fuzziness: Option<Fuzziness>,
   }

   impl AuditLogger {
       pub async fn full_text_search(&self, query: FullTextQuery) -> Result<Vec<AuditLog>> {
           let search_query = json!({
               "query": {
                   "multi_match": {
                       "query": query.query,
                       "fields": query.fields,
                       "operator": query.operator,
                       "fuzziness": query.fuzziness
                   }
               }
           });

           self.es.search(&search_query).await
       }
   }
   ```

2. **聚合查询：**

   ```rust
   pub struct AggregationQuery {
       pub group_by: Vec<String>,
       pub metrics: Vec<AggregationMetric>,
       pub filters: Option<Vec<QueryFilter>>,
   }

   impl AuditLogger {
       pub async fn aggregate(&self, query: AggregationQuery) -> Result<Value> {
           let agg_query = json!({
               "size": 0,
               "query": {
                   "bool": {
                       "must": query.filters
                   }
               },
               "aggs": {
                   "group_by": {
                       "terms": {
                           "field": query.group_by.join(".")
                       },
                       "aggs": {
                           "metrics": query.metrics
                       }
                   }
               }
           });

           self.es.aggregate(&agg_query).await
       }
   }
   ```

## 6. 日志分析

### 6.1 统计分析

1. **操作统计：**

   ```rust
   pub struct OperationStats {
       pub total_operations: u64,
       pub success_rate: f64,
       pub error_rate: f64,
       pub avg_response_time: f64,
       pub operation_distribution: HashMap<Operation, u64>,
   }

   impl AuditLogger {
       pub async fn get_operation_stats(&self, time_range: TimeRange) -> Result<OperationStats> {
           let query = json!({
               "size": 0,
               "query": {
                   "range": {
                       "timestamp": {
                           "gte": time_range.start,
                           "lte": time_range.end
                       }
                   }
               },
               "aggs": {
                   "total_operations": { "value_count": { "field": "_id" } },
                   "success_rate": {
                       "avg": {
                           "script": {
                               "source": "doc['status'].value == 'Success' ? 1 : 0"
                           }
                       }
                   },
                   "operation_distribution": {
                       "terms": { "field": "operation" }
                   }
               }
           });

           let result = self.es.aggregate(&query).await?;
           Ok(OperationStats::from(result))
       }
   }
   ```

2. **用户行为分析：**

   ```rust
   pub struct UserBehavior {
       pub user_id: String,
       pub operation_count: u64,
       pub resource_access: HashMap<ResourceType, u64>,
       pub error_rate: f64,
       pub active_time: Vec<TimeRange>,
   }

   impl AuditLogger {
       pub async fn analyze_user_behavior(&self, user_id: String) -> Result<UserBehavior> {
           let query = json!({
               "query": {
                   "term": { "user_id": user_id }
               },
               "aggs": {
                   "operation_count": { "value_count": { "field": "_id" } },
                   "resource_access": {
                       "terms": { "field": "resource_type" }
                   },
                   "error_rate": {
                       "avg": {
                           "script": {
                               "source": "doc['status'].value == 'Failed' ? 1 : 0"
                           }
                       }
                   },
                   "active_time": {
                       "date_histogram": {
                           "field": "timestamp",
                           "calendar_interval": "hour"
                       }
                   }
               }
           });

           let result = self.es.aggregate(&query).await?;
           Ok(UserBehavior::from(result))
       }
   }
   ```

### 6.2 异常检测

1. **异常模式识别：**

   ```rust
   pub struct AnomalyDetection {
       pub threshold: f64,
       pub window_size: Duration,
       pub metrics: Vec<String>,
   }

   impl AuditLogger {
       pub async fn detect_anomalies(&self, config: AnomalyDetection) -> Result<Vec<Anomaly>> {
           let query = json!({
               "size": 0,
               "query": {
                   "range": {
                       "timestamp": {
                           "gte": "now-1h",
                           "lte": "now"
                       }
                   }
               },
               "aggs": {
                   "time_buckets": {
                       "date_histogram": {
                           "field": "timestamp",
                           "fixed_interval": config.window_size.as_secs().to_string() + "s"
                       },
                       "aggs": {
                           "metrics": config.metrics.iter().map(|metric| {
                               json!({
                                   metric: {
                                       "avg": { "field": metric }
                                   }
                               })
                           }).collect::<Vec<_>>()
                       }
                   }
               }
           });

           let result = self.es.aggregate(&query).await?;
           Ok(Anomaly::detect(result, config.threshold))
       }
   }
   ```

2. **安全事件分析：**

   ```rust
   pub struct SecurityEvent {
       pub event_type: SecurityEventType,
       pub severity: SecuritySeverity,
       pub timestamp: DateTime<Utc>,
       pub user_id: String,
       pub details: Value,
   }

   impl AuditLogger {
       pub async fn analyze_security_events(&self) -> Result<Vec<SecurityEvent>> {
           let query = json!({
               "query": {
                   "bool": {
                       "must": [
                           { "term": { "status": "Failed" } },
                           { "range": {
                               "timestamp": {
                                   "gte": "now-24h",
                                   "lte": "now"
                               }
                           }}
                       ]
                   }
               },
               "aggs": {
                   "event_types": {
                       "terms": { "field": "operation" }
                   },
                   "severity_distribution": {
                       "terms": { "field": "severity" }
                   }
               }
           });

           let result = self.es.aggregate(&query).await?;
           Ok(SecurityEvent::from(result))
       }
   }
   ```

## 7. 日志展示

### 7.1 审计日志界面

1. **日志列表：**

   ```typescript
   interface AuditLogList {
     logs: AuditLog[];
     total: number;
     page: number;
     pageSize: number;
     filters: AuditLogFilter;
   }

   interface AuditLogFilter {
     timeRange: TimeRange;
     userId?: string;
     operation?: Operation;
     resourceType?: ResourceType;
     status?: OperationStatus;
   }
   ```

2. **统计面板：**

   ```typescript
   interface AuditDashboard {
     operationStats: OperationStats;
     userBehavior: UserBehavior[];
     securityEvents: SecurityEvent[];
     anomalies: Anomaly[];
   }
   ```

### 7.2 报表导出

1. **报表格式：**

   ```rust
   pub enum ReportFormat {
       CSV,
       Excel,
       PDF,
   }

   pub struct ReportConfig {
       pub format: ReportFormat,
       pub time_range: TimeRange,
       pub filters: Vec<QueryFilter>,
       pub columns: Vec<String>,
   }
   ```

2. **导出实现：**

   ```rust
   impl AuditLogger {
       pub async fn export_report(&self, config: ReportConfig) -> Result<Vec<u8>> {
           // 查询数据
           let logs = self.query(AuditLogQuery::from(config)).await?;

           // 根据格式导出
           match config.format {
               ReportFormat::CSV => self.export_csv(logs, config.columns),
               ReportFormat::Excel => self.export_excel(logs, config.columns),
               ReportFormat::PDF => self.export_pdf(logs, config.columns),
           }
       }
   }
   ```

## 8. 性能优化

### 8.1 写入优化

1. **批量写入：**

   ```rust
   impl AuditLogger {
       pub async fn batch_write(&self, logs: Vec<AuditLog>) -> Result<()> {
           // 分批处理
           for chunk in logs.chunks(1000) {
               // 并行写入数据库和ES
               let (db_result, es_result) = join!(
                   self.db.batch_insert(chunk),
                   self.es.batch_index(chunk)
               );

               // 处理结果
               db_result?;
               es_result?;
           }

           Ok(())
       }
   }
   ```

2. **异步写入：**

   ```rust
   impl AuditLogger {
       pub async fn async_write(&self, log: AuditLog) -> Result<()> {
           // 发送到消息队列
           self.queue.send(log).await?;

           Ok(())
       }
   }
   ```

### 8.2 查询优化

1. **缓存策略：**

   ```rust
   pub struct AuditLogCache {
       cache: Arc<Cache<String, Vec<AuditLog>>>,
       ttl: Duration,
   }

   impl AuditLogCache {
       pub async fn get(&self, key: &str) -> Option<Vec<AuditLog>> {
           self.cache.get(key).await
       }

       pub async fn set(&self, key: &str, value: Vec<AuditLog>) {
           self.cache.set(key, value, self.ttl).await;
       }
   }
   ```

2. **索引优化：**

   ```sql
   -- 创建复合索引
   CREATE INDEX idx_audit_logs_composite ON audit_logs(timestamp, user_id, operation);
   CREATE INDEX idx_audit_logs_resource ON audit_logs(resource_type, resource_id);
   ```

## 9. 安全设计

### 9.1 日志安全

1. **敏感信息脱敏：**

   ```rust
   pub struct SensitiveDataMasker {
       patterns: Vec<Regex>,
   }

   impl SensitiveDataMasker {
       pub fn mask(&self, data: &str) -> String {
           let mut masked = data.to_string();
           for pattern in &self.patterns {
               masked = pattern.replace_all(&masked, "***").to_string();
           }
           masked
       }
   }
   ```

2. **访问控制：**

   ```rust
   pub struct AuditLogAccessControl {
       permissions: HashMap<String, Vec<Permission>>,
   }

   impl AuditLogAccessControl {
       pub fn can_access(&self, user_id: &str, operation: &str) -> bool {
           if let Some(user_permissions) = self.permissions.get(user_id) {
               user_permissions.iter().any(|p| p.matches(operation))
           } else {
               false
           }
       }
   }
   ```

### 9.2 数据保护

1. **加密存储：**

   ```rust
   pub struct EncryptedStorage {
       encryption_key: Vec<u8>,
   }

   impl EncryptedStorage {
       pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
           // 使用AES-GCM加密
           let cipher = Aes256Gcm::new(Key::from_slice(&self.encryption_key));
           let nonce = Nonce::from_slice(b"unique nonce");
           cipher.encrypt(nonce, data)
       }

       pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
           // 使用AES-GCM解密
           let cipher = Aes256Gcm::new(Key::from_slice(&self.encryption_key));
           let nonce = Nonce::from_slice(b"unique nonce");
           cipher.decrypt(nonce, data)
       }
   }
   ```

2. **数据备份：**

   ```rust
   pub struct BackupManager {
       backup_path: PathBuf,
       retention_days: u32,
   }

   impl BackupManager {
       pub async fn backup(&self) -> Result<()> {
           // 创建备份
           let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
           let backup_file = self.backup_path.join(format!("audit_logs_{}.sql", timestamp));
           
           // 导出数据
           self.export_data(&backup_file).await?;
           
           // 压缩备份
           self.compress_backup(&backup_file).await?;
           
           // 清理旧备份
           self.cleanup_old_backups().await?;
           
           Ok(())
       }
   }
   ```

## 10. 总结

审计日志系统通过完整的日志模型、高效的存储方案、强大的查询能力、丰富的分析功能，以及完善的安全机制，为系统提供了全面的审计能力，确保了系统的安全性和可追溯性。
