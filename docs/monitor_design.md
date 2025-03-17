# 监控告警设计文档

## 1. 概述

本设计文档详细描述了分布式配置中心的监控告警系统，采用Prometheus + Grafana + Alertmanager的技术栈，实现全方位的系统监控和智能告警。

### 1.1 设计目标

- **全面性：** 覆盖系统各个层面的监控
- **实时性：** 及时发现和处理异常
- **可扩展性：** 支持自定义指标和告警规则
- **可视化：** 直观展示系统状态

### 1.2 核心组件

- **指标收集：** Prometheus
- **数据存储：** Prometheus TSDB
- **可视化：** Grafana
- **告警管理：** Alertmanager
- **日志收集：** ELK Stack

## 2. 监控指标

### 2.1 系统指标

1. **资源使用：**

   ```rust
   struct SystemMetrics {
       cpu_usage: f64,        // CPU使用率
       memory_usage: f64,     // 内存使用率
       disk_usage: f64,       // 磁盘使用率
       network_io: NetworkIO, // 网络IO
   }

   struct NetworkIO {
       bytes_sent: u64,       // 发送字节数
       bytes_recv: u64,       // 接收字节数
       packets_sent: u64,     // 发送包数
       packets_recv: u64,     // 接收包数
   }
   ```

2. **进程状态：**

   ```rust
   struct ProcessMetrics {
       pid: u32,              // 进程ID
       status: String,        // 进程状态
       cpu_percent: f64,      // CPU百分比
       memory_percent: f64,   // 内存百分比
       thread_count: u32,     // 线程数
       open_files: u32,       // 打开文件数
   }
   ```

### 2.2 应用指标

1. **配置服务：**

   ```rust
   struct ConfigServiceMetrics {
       request_count: Counter,    // 请求总数
       request_latency: Histogram, // 请求延迟
       error_count: Counter,      // 错误总数
       cache_hits: Counter,       // 缓存命中数
       cache_misses: Counter,     // 缓存未命中数
       active_connections: Gauge, // 活动连接数
   }
   ```

2. **Raft集群：**

   ```rust
   struct RaftMetrics {
       leader_id: Gauge,          // 当前Leader ID
       term: Gauge,               // 当前任期
       commit_index: Gauge,       // 提交索引
       applied_index: Gauge,      // 应用索引
       follower_lag: Gauge,       // Follower延迟
       election_count: Counter,   // 选举次数
   }
   ```

### 2.3 业务指标

1. **配置管理：**

   ```rust
   struct ConfigMetrics {
       total_configs: Gauge,      // 配置总数
       config_updates: Counter,   // 配置更新次数
       config_deletes: Counter,   // 配置删除次数
       config_reads: Counter,     // 配置读取次数
       config_writes: Counter,    // 配置写入次数
   }
   ```

2. **用户操作：**

   ```rust
   struct UserMetrics {
       active_users: Gauge,       // 活跃用户数
       login_attempts: Counter,   // 登录尝试次数
       failed_logins: Counter,    // 登录失败次数
       api_calls: Counter,        // API调用次数
       error_rate: Gauge,         // 错误率
   }
   ```

## 3. 指标收集

### 3.1 Prometheus配置

1. **采集配置：**

   ```yaml
   global:
     scrape_interval: 15s
     evaluation_interval: 15s

   scrape_configs:
     - job_name: 'config-center'
       static_configs:
         - targets: ['localhost:9090']
       metrics_path: '/metrics'
       scrape_interval: 5s
   ```

2. **标签配置：**

   ```yaml
   relabel_configs:
     - source_labels: [__address__]
       target_label: instance
       replacement: '${HOSTNAME}'
     - source_labels: [__name__]
       regex: 'node_(.*)'
       target_label: metric_type
       replacement: '$1'
   ```

### 3.2 指标暴露

1. **HTTP端点：**

   ```rust
   struct MetricsExporter {
       registry: Registry,
       metrics: Arc<Metrics>,
   }

   impl MetricsExporter {
       async fn handle_metrics(&self, req: Request<Body>) -> Response<Body> {
           let mut buffer = Vec::new();
           let encoder = TextEncoder::new();
           
           // 收集系统指标
           self.collect_system_metrics(&mut buffer, &encoder)?;
           
           // 收集应用指标
           self.collect_app_metrics(&mut buffer, &encoder)?;
           
           // 收集业务指标
           self.collect_business_metrics(&mut buffer, &encoder)?;
           
           Response::new(Body::from(buffer))
       }
   }
   ```

2. **指标格式：**

   ```text
   # HELP config_center_request_total Total number of requests
   # TYPE config_center_request_total counter
   config_center_request_total{method="GET",endpoint="/api/v1/configs"} 1234
   config_center_request_total{method="POST",endpoint="/api/v1/configs"} 567

   # HELP config_center_request_duration_seconds Request duration in seconds
   # TYPE config_center_request_duration_seconds histogram
   config_center_request_duration_seconds_bucket{le="0.1"} 100
   config_center_request_duration_seconds_bucket{le="0.5"} 200
   config_center_request_duration_seconds_bucket{le="1.0"} 300
   config_center_request_duration_seconds_bucket{le="+Inf"} 400
   ```

## 4. 告警规则

### 4.1 系统告警

1. **资源告警：**

   ```yaml
   groups:
     - name: system
       rules:
         - alert: HighCPUUsage
           expr: cpu_usage > 80
           for: 5m
           labels:
             severity: warning
           annotations:
             summary: High CPU usage
             description: CPU usage is above 80% for 5 minutes

         - alert: HighMemoryUsage
           expr: memory_usage > 85
           for: 5m
           labels:
             severity: warning
           annotations:
             summary: High memory usage
             description: Memory usage is above 85% for 5 minutes
   ```

2. **进程告警：**

   ```yaml
   groups:
     - name: process
       rules:
         - alert: ProcessDown
           expr: up == 0
           for: 1m
           labels:
             severity: critical
           annotations:
             summary: Process is down
             description: Process {{ $labels.instance }} is down

         - alert: HighThreadCount
           expr: thread_count > 1000
           for: 5m
           labels:
             severity: warning
           annotations:
             summary: High thread count
             description: Thread count is above 1000 for 5 minutes
   ```

### 4.2 应用告警

1. **服务告警：**

   ```yaml
   groups:
     - name: service
       rules:
         - alert: HighErrorRate
           expr: rate(error_count[5m]) / rate(request_count[5m]) > 0.05
           for: 5m
           labels:
             severity: warning
           annotations:
             summary: High error rate
             description: Error rate is above 5% for 5 minutes

         - alert: HighLatency
           expr: histogram_quantile(0.95, rate(request_duration_seconds_bucket[5m])) > 1
           for: 5m
           labels:
             severity: warning
           annotations:
             summary: High latency
             description: 95th percentile latency is above 1s for 5 minutes
   ```

2. **Raft告警：**

   ```yaml
   groups:
     - name: raft
       rules:
         - alert: NoLeader
           expr: leader_id == -1
           for: 1m
           labels:
             severity: critical
           annotations:
             summary: No Raft leader
             description: Raft cluster has no leader for 1 minute

         - alert: HighFollowerLag
           expr: follower_lag > 1000
           for: 5m
           labels:
             severity: warning
           annotations:
             summary: High follower lag
             description: Follower lag is above 1000 for 5 minutes
   ```

### 4.3 业务告警

1. **配置告警：**

   ```yaml
   groups:
     - name: config
       rules:
         - alert: ConfigUpdateRate
           expr: rate(config_updates[5m]) > 100
           for: 5m
           labels:
             severity: warning
           annotations:
             summary: High config update rate
             description: Config update rate is above 100/min for 5 minutes

         - alert: ConfigDeleteRate
           expr: rate(config_deletes[5m]) > 50
           for: 5m
           labels:
             severity: warning
           annotations:
             summary: High config delete rate
             description: Config delete rate is above 50/min for 5 minutes
   ```

2. **用户告警：**

   ```yaml
   groups:
     - name: user
       rules:
         - alert: HighFailedLogins
           expr: rate(failed_logins[5m]) > 10
           for: 5m
           labels:
             severity: warning
           annotations:
             summary: High failed login rate
             description: Failed login rate is above 10/min for 5 minutes

         - alert: HighErrorRate
           expr: error_rate > 0.05
           for: 5m
           labels:
             severity: warning
           annotations:
             summary: High API error rate
             description: API error rate is above 5% for 5 minutes
   ```

## 5. 告警通知

### 5.1 Alertmanager配置

1. **路由配置：**

   ```yaml
   global:
     resolve_timeout: 5m

   route:
     group_by: ['alertname', 'instance']
     group_wait: 10s
     group_interval: 5m
     repeat_interval: 4h
     receiver: 'team-mail'

   receivers:
     - name: 'team-mail'
       email_configs:
         - to: 'team@example.com'
           send_resolved: true
     - name: 'slack-notifications'
       slack_configs:
         - api_url: 'https://hooks.slack.com/services/xxx/yyy/zzz'
           channel: '#alerts'
   ```

2. **抑制规则：**

   ```yaml
   inhibit_rules:
     - source_match:
         severity: 'critical'
       target_match:
         severity: 'warning'
       equal: ['alertname', 'instance']
   ```

### 5.2 通知模板

1. **邮件模板：**

   ```html
   {{ define "email.html" }}
   <html>
     <head>
       <title>{{ template "email.title" . }}</title>
     </head>
     <body>
       <h2>{{ template "email.title" . }}</h2>
       <table>
         <tr>
           <td>Alert Name:</td>
           <td>{{ .AlertName }}</td>
         </tr>
         <tr>
           <td>Severity:</td>
           <td>{{ .Severity }}</td>
         </tr>
         <tr>
           <td>Description:</td>
           <td>{{ .Description }}</td>
         </tr>
       </table>
     </body>
   </html>
   {{ end }}
   ```

2. **Slack模板：**

   ```json
   {
     "blocks": [
       {
         "type": "section",
         "text": {
           "type": "mrkdwn",
           "text": "*{{ .AlertName }}*"
         }
       },
       {
         "type": "section",
         "fields": [
           {
             "type": "mrkdwn",
             "text": "*Severity:*\n{{ .Severity }}"
           },
           {
             "type": "mrkdwn",
             "text": "*Description:*\n{{ .Description }}"
           }
         ]
       }
     ]
   }
   ```

## 6. 可视化

### 6.1 Grafana仪表板

1. **系统概览：**

   ```json
   {
     "dashboard": {
       "id": null,
       "title": "System Overview",
       "panels": [
         {
           "title": "CPU Usage",
           "type": "graph",
           "datasource": "Prometheus",
           "targets": [
             {
               "expr": "cpu_usage",
               "legendFormat": "CPU"
             }
           ]
         },
         {
           "title": "Memory Usage",
           "type": "graph",
           "datasource": "Prometheus",
           "targets": [
             {
               "expr": "memory_usage",
               "legendFormat": "Memory"
             }
           ]
         }
       ]
     }
   }
   ```

2. **应用监控：**

   ```json
   {
     "dashboard": {
       "id": null,
       "title": "Application Metrics",
       "panels": [
         {
           "title": "Request Rate",
           "type": "graph",
           "datasource": "Prometheus",
           "targets": [
             {
               "expr": "rate(request_count[5m])",
               "legendFormat": "{{method}} {{endpoint}}"
             }
           ]
         },
         {
           "title": "Error Rate",
           "type": "graph",
           "datasource": "Prometheus",
           "targets": [
             {
               "expr": "rate(error_count[5m]) / rate(request_count[5m])",
               "legendFormat": "Error Rate"
             }
           ]
         }
       ]
     }
   }
   ```

### 6.2 告警面板

1. **告警列表：**

   ```json
   {
     "dashboard": {
       "id": null,
       "title": "Alerts",
       "panels": [
         {
           "title": "Active Alerts",
           "type": "table",
           "datasource": "Prometheus",
           "targets": [
             {
               "expr": "ALERTS",
               "format": "table"
             }
           ]
         }
       ]
     }
   }
   ```

2. **告警统计：**

   ```json
   {
     "dashboard": {
       "id": null,
       "title": "Alert Statistics",
       "panels": [
         {
           "title": "Alert Rate",
           "type": "graph",
           "datasource": "Prometheus",
           "targets": [
             {
               "expr": "rate(ALERTS[5m])",
               "legendFormat": "{{alertname}}"
             }
           ]
         }
       ]
     }
   }
   ```

## 7. 日志收集

### 7.1 ELK配置

1. **Logstash配置：**

   ```yaml
   input {
     file {
       path => "/var/log/config-center/*.log"
       type => "config-center"
       codec => json
     }
   }

   filter {
     if [type] == "config-center" {
       date {
         match => [ "timestamp", "ISO8601" ]
         target => "@timestamp"
       }
     }
   }

   output {
     elasticsearch {
       hosts => ["localhost:9200"]
       index => "config-center-%{+YYYY.MM.dd}"
     }
   }
   ```

2. **索引模板：**

   ```json
   {
     "template": "config-center-*",
     "settings": {
       "number_of_shards": 3,
       "number_of_replicas": 2
     },
     "mappings": {
       "properties": {
         "timestamp": { "type": "date" },
         "level": { "type": "keyword" },
         "message": { "type": "text" },
         "service": { "type": "keyword" },
         "trace_id": { "type": "keyword" }
       }
     }
   }
   ```

### 7.2 日志分析

1. **日志查询：**

   ```json
   {
     "query": {
       "bool": {
         "must": [
           { "match": { "level": "ERROR" } },
           { "range": {
               "@timestamp": {
                 "gte": "now-1h",
                 "lte": "now"
               }
             }
           }
         ]
       }
     },
     "aggs": {
       "service_errors": {
         "terms": {
           "field": "service",
           "size": 10
         }
       }
     }
   }
   ```

2. **告警关联：**

   ```json
   {
     "query": {
       "bool": {
         "must": [
           { "match": { "trace_id": "{{ .TraceID }}" } }
         ]
       }
     }
   }
   ```

## 8. 性能优化

### 8.1 采集优化

1. **采集间隔：**
   - 系统指标：15s
   - 应用指标：5s
   - 业务指标：1m

2. **标签优化：**
   - 减少标签基数
   - 使用标签过滤
   - 合理设置采集范围

### 8.2 存储优化

1. **数据压缩：**
   - 启用数据压缩
   - 设置合理的保留时间
   - 定期清理过期数据

2. **查询优化：**
   - 使用预计算
   - 优化查询语句
   - 合理使用缓存

## 9. 高可用设计

### 9.1 监控高可用

1. **Prometheus集群：**

   ```yaml
   global:
     external_labels:
       region: '{{ region }}'
       replica: '{{ replica }}'

   rule_files:
     - 'rules/*.yml'

   alerting:
     alertmanagers:
       - static_configs:
           - targets:
             - 'alertmanager:9093'
   ```

2. **数据备份：**
   - 定期备份TSDB数据
   - 多副本存储
   - 跨区域复制

### 9.2 告警高可用

1. **Alertmanager集群：**

   ```yaml
   global:
     resolve_timeout: 5m

   route:
     receiver: 'default-receiver'
     group_by: ['alertname', 'instance']
     group_wait: 30s
     group_interval: 5m
     repeat_interval: 4h

   receivers:
     - name: 'default-receiver'
       email_configs:
         - to: 'team@example.com'
       slack_configs:
         - api_url: 'https://hooks.slack.com/services/xxx/yyy/zzz'
           channel: '#alerts'
   ```

2. **通知冗余：**
   - 多通道通知
   - 通知重试机制
   - 通知确认机制

## 10. 总结

监控告警系统通过Prometheus + Grafana + Alertmanager实现了全方位的系统监控和智能告警，通过合理的配置和优化确保了系统的高可用性和可维护性。
