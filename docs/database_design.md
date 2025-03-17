# 数据库设计文档

## 1. 数据库概述

本设计文档详细描述了分布式配置中心的数据库设计方案。系统使用 PostgreSQL 作为主数据库，Redis 作为缓存数据库。

### 1.1 数据库选型

- **主数据库：** PostgreSQL
  - 原因：
    - 强大的事务支持
    - 优秀的并发控制
    - 丰富的数据类型
    - 强大的扩展性
    - 活跃的社区支持

- **缓存数据库：** Redis
  - 原因：
    - 高性能
    - 支持多种数据结构
    - 支持数据持久化
    - 支持主从复制

### 1.2 数据库版本

- PostgreSQL: 15.x
- Redis: 7.x

## 2. 数据库表设计

### 2.1 命名空间表 (namespace)

```sql
CREATE TABLE namespace (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_namespace_name ON namespace(name);
```

**字段说明：**

- `id`: 命名空间唯一标识符
- `name`: 命名空间名称
- `description`: 命名空间描述
- `created_at`: 创建时间
- `updated_at`: 更新时间

### 2.2 部门表 (department)

```sql
CREATE TABLE department (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_department_name ON department(name);
```

**字段说明：**

- `id`: 部门唯一标识符
- `name`: 部门名称
- `description`: 部门描述
- `created_at`: 创建时间
- `updated_at`: 更新时间

### 2.3 应用表 (application)

```sql
CREATE TABLE application (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_application_name ON application(name);
```

**字段说明：**

- `id`: 应用唯一标识符
- `name`: 应用名称
- `description`: 应用描述
- `created_at`: 创建时间
- `updated_at`: 更新时间

### 2.4 域表 (domain)

```sql
CREATE TABLE domain (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_domain_name ON domain(name);
```

**字段说明：**

- `id`: 域唯一标识符
- `name`: 域名称
- `description`: 域描述
- `created_at`: 创建时间
- `updated_at`: 更新时间

### 2.5 环境表 (environment)

```sql
CREATE TABLE environment (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_environment_name ON environment(name);
```

**字段说明：**

- `id`: 环境唯一标识符
- `name`: 环境名称
- `description`: 环境描述
- `created_at`: 创建时间
- `updated_at`: 更新时间

### 2.6 配置表 (config)

```sql
CREATE TABLE config (
    id VARCHAR(36) PRIMARY KEY,
    namespace_id VARCHAR(36) NOT NULL REFERENCES namespace(id),
    department_id VARCHAR(36) NOT NULL REFERENCES department(id),
    application_id VARCHAR(36) NOT NULL REFERENCES application(id),
    domain_id VARCHAR(36) NOT NULL REFERENCES domain(id),
    environment_id VARCHAR(36) NOT NULL REFERENCES environment(id),
    key VARCHAR(255) NOT NULL,
    value TEXT NOT NULL,
    format VARCHAR(50) NOT NULL,
    is_encrypted BOOLEAN NOT NULL DEFAULT FALSE,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(namespace_id, department_id, application_id, domain_id, environment_id, key)
);

CREATE INDEX idx_config_lookup ON config(namespace_id, department_id, application_id, domain_id, environment_id, key);
```

**字段说明：**

- `id`: 配置唯一标识符
- `namespace_id`: 命名空间ID
- `department_id`: 部门ID
- `application_id`: 应用ID
- `domain_id`: 域ID
- `environment_id`: 环境ID
- `key`: 配置键
- `value`: 配置值
- `format`: 配置格式（YAML、Properties等）
- `is_encrypted`: 是否加密
- `description`: 配置描述
- `created_at`: 创建时间
- `updated_at`: 更新时间

### 2.7 配置版本表 (config_version)

```sql
CREATE TABLE config_version (
    id VARCHAR(36) PRIMARY KEY,
    config_id VARCHAR(36) NOT NULL REFERENCES config(id),
    value TEXT NOT NULL,
    version VARCHAR(50) NOT NULL,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_config_version_lookup ON config_version(config_id, version);
```

**字段说明：**

- `id`: 版本唯一标识符
- `config_id`: 配置ID
- `value`: 配置值
- `version`: 版本号
- `description`: 版本描述
- `created_at`: 创建时间

### 2.8 角色表 (role)

```sql
CREATE TABLE role (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_role_name ON role(name);
```

**字段说明：**

- `id`: 角色唯一标识符
- `name`: 角色名称
- `description`: 角色描述
- `created_at`: 创建时间
- `updated_at`: 更新时间

### 2.9 用户表 (user)

```sql
CREATE TABLE user (
    id VARCHAR(36) PRIMARY KEY,
    username VARCHAR(100) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_user_username ON user(username);
CREATE INDEX idx_user_email ON user(email);
```

**字段说明：**

- `id`: 用户唯一标识符
- `username`: 用户名
- `password`: 密码（加密存储）
- `email`: 邮箱
- `created_at`: 创建时间
- `updated_at`: 更新时间

### 2.10 用户角色关联表 (user_role)

```sql
CREATE TABLE user_role (
    user_id VARCHAR(36) NOT NULL REFERENCES user(id),
    role_id VARCHAR(36) NOT NULL REFERENCES role(id),
    PRIMARY KEY (user_id, role_id)
);

CREATE INDEX idx_user_role_user ON user_role(user_id);
CREATE INDEX idx_user_role_role ON user_role(role_id);
```

**字段说明：**

- `user_id`: 用户ID
- `role_id`: 角色ID

### 2.11 Casbin规则表 (casbin_rule)

```sql
CREATE TABLE casbin_rule (
    id VARCHAR(36) PRIMARY KEY,
    ptype VARCHAR(10) NOT NULL,
    v0 VARCHAR(256),
    v1 VARCHAR(256),
    v2 VARCHAR(256),
    v3 VARCHAR(256),
    v4 VARCHAR(256),
    v5 VARCHAR(256)
);

CREATE INDEX idx_casbin_rule_ptype ON casbin_rule(ptype);
```

**字段说明：**

- `id`: 规则唯一标识符
- `ptype`: 规则类型
- `v0-v5`: 规则值

### 2.12 审计日志表 (audit_log)

```sql
CREATE TABLE audit_log (
    id VARCHAR(36) PRIMARY KEY,
    user_id VARCHAR(36) NOT NULL REFERENCES user(id),
    operation VARCHAR(50) NOT NULL,
    resource VARCHAR(255) NOT NULL,
    detail TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_audit_log_user ON audit_log(user_id);
CREATE INDEX idx_audit_log_created_at ON audit_log(created_at);
```

**字段说明：**

- `id`: 日志唯一标识符
- `user_id`: 用户ID
- `operation`: 操作类型
- `resource`: 操作资源
- `detail`: 详细信息
- `created_at`: 创建时间

## 3. Redis缓存设计

### 3.1 缓存键设计

#### 3.1.1 配置缓存

``` plaintext
config:{namespace}:{department}:{application}:{domain}:{environment}:{key}
```

**示例：**

``` plaintext
config:prod:devops:order:payment:prod:db.url
```

**缓存策略：**

- 缓存时间：5分钟
- 更新策略：Cache-Aside
- 失效策略：主动失效

#### 3.1.2 用户会话缓存

``` plaintext
session:{session_id}
```

**缓存策略：**

- 缓存时间：24小时
- 更新策略：Write-Through
- 失效策略：TTL

#### 3.1.3 权限缓存

``` plaintext
permission:{user_id}
```

**缓存策略：**

- 缓存时间：1小时
- 更新策略：Cache-Aside
- 失效策略：主动失效

### 3.2 Redis数据结构

#### 3.2.1 配置缓存

```json
{
    "value": "jdbc:postgresql://localhost:5432/order",
    "format": "properties",
    "is_encrypted": false,
    "version": "1.0.0",
    "updated_at": "2024-03-17T10:00:00Z"
}
```

#### 3.2.2 用户会话

```json
{
    "user_id": "123",
    "username": "admin",
    "roles": ["admin", "developer"],
    "expires_at": "2024-03-18T10:00:00Z"
}
```

#### 3.2.3 权限缓存

```json
{
    "permissions": [
        {
            "resource": "config",
            "actions": ["read", "write", "delete"]
        }
    ],
    "updated_at": "2024-03-17T10:00:00Z"
}
```

## 4. 数据库性能优化

### 4.1 索引优化

1. **联合索引：**
   - 配置表的多维度查询索引
   - 审计日志表的用户和时间索引

2. **前缀索引：**
   - 配置键的前缀索引
   - 用户名的前缀索引

### 4.2 查询优化

1. **分页查询：**
   - 使用 LIMIT 和 OFFSET
   - 使用游标分页

2. **批量操作：**
   - 使用 INSERT ON CONFLICT
   - 使用批量更新

### 4.3 缓存优化

1. **缓存预热：**
   - 系统启动时预热常用配置
   - 定时预热即将过期的缓存

2. **缓存更新：**
   - 使用消息队列异步更新缓存
   - 使用分布式锁避免缓存击穿

## 5. 数据库安全

### 5.1 访问控制

1. **用户权限：**
   - 最小权限原则
   - 角色分离

2. **连接安全：**
   - SSL/TLS 加密
   - IP 白名单

### 5.2 数据安全

1. **数据加密：**
   - 敏感数据加密存储
   - 传输加密

2. **数据备份：**
   - 定期全量备份
   - 实时增量备份

## 6. 数据库监控

### 6.1 性能监控

1. **查询性能：**
   - 慢查询日志
   - 查询计划分析

2. **资源使用：**
   - CPU 使用率
   - 内存使用率
   - 磁盘使用率

### 6.2 可用性监控

1. **连接状态：**
   - 连接数监控
   - 连接池状态

2. **复制状态：**
   - 主从延迟
   - 复制错误

## 7. 数据库维护

### 7.1 日常维护

1. **数据清理：**
   - 定期清理过期数据
   - 定期优化表结构

2. **性能优化：**
   - 定期分析索引使用情况
   - 定期更新统计信息

### 7.2 故障处理

1. **故障恢复：**
   - 主从切换
   - 数据恢复

2. **性能问题：**
   - 锁等待分析
   - 死锁处理
