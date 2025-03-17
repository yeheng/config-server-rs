# 权限管理设计文档

## 1. 概述

本设计文档详细描述了分布式配置中心的权限管理系统，采用基于角色的访问控制（RBAC）模型，结合Casbin实现灵活的权限控制。

### 1.1 设计目标

- **安全性：** 确保配置数据的安全访问
- **灵活性：** 支持细粒度的权限控制
- **可扩展性：** 易于添加新的角色和权限
- **性能：** 高效的权限验证机制

### 1.2 核心组件

- **认证服务：** 用户身份验证
- **授权服务：** 权限验证和决策
- **角色管理：** 角色定义和分配
- **权限策略：** 访问控制规则

## 2. 认证机制

### 2.1 认证方式

1. **JWT认证：**

   ```rust
   struct JwtClaims {
       sub: String,      // 用户ID
       username: String, // 用户名
       roles: Vec<String>, // 角色列表
       exp: i64,        // 过期时间
       iat: i64,        // 签发时间
   }
   ```

2. **API Key认证：**

   ```rust
   struct ApiKey {
       key: String,     // API密钥
       user_id: String, // 用户ID
       name: String,    // 密钥名称
       created_at: DateTime,
       expires_at: Option<DateTime>,
       permissions: Vec<String>, // 权限列表
   }
   ```

### 2.2 认证流程

1. **JWT认证流程：**

   ```mermaid
   sequenceDiagram
       participant Client
       participant Auth
       participant DB
       Client->>Auth: 登录请求
       Auth->>DB: 验证用户
       DB-->>Auth: 用户信息
       Auth->>Auth: 生成JWT
       Auth-->>Client: 返回Token
       Client->>Auth: 请求资源
       Auth->>Auth: 验证Token
       Auth-->>Client: 验证结果
   ```

2. **API Key认证流程：**

   ```mermaid
   sequenceDiagram
       participant Client
       participant Auth
       participant Cache
       participant DB
       Client->>Auth: API Key请求
       Auth->>Cache: 查询Key
       alt 缓存命中
           Cache-->>Auth: Key信息
       else 缓存未命中
           Auth->>DB: 查询Key
           DB-->>Auth: Key信息
           Auth->>Cache: 缓存Key
       end
       Auth->>Auth: 验证Key
       Auth-->>Client: 验证结果
   ```

## 3. 授权机制

### 3.1 RBAC模型

1. **核心概念：**
   - 用户（User）
   - 角色（Role）
   - 权限（Permission）
   - 资源（Resource）

2. **关系模型：**

   ```mermaid
   erDiagram
       User ||--o{ UserRole : has
       Role ||--o{ UserRole : has
       Role ||--o{ RolePermission : has
       Permission ||--o{ RolePermission : has
       Permission ||--o{ Resource : controls
   ```

### 3.2 Casbin策略

1. **模型定义：**

   ```ini
   [request_definition]
   r = sub, dom, obj, act

   [policy_definition]
   p = sub, dom, obj, act

   [role_definition]
   g = _, _, _

   [policy_effect]
   e = some(where (p.eft == allow))

   [matchers]
   m = g(r.sub, p.sub, r.dom) && r.dom == p.dom && r.obj == p.obj && r.act == p.act
   ```

2. **策略示例：**

   ```csv
   p, admin, *, *, *
   p, developer, dev, config, read
   p, developer, dev, config, write
   p, operator, prod, config, read
   g, alice, admin, *
   g, bob, developer, dev
   g, charlie, operator, prod
   ```

### 3.3 权限验证

1. **验证流程：**

   ```rust
   struct PermissionChecker {
       enforcer: Arc<RwLock<Enforcer>>,
       cache: Arc<Cache>,
   }

   impl PermissionChecker {
       async fn check_permission(&self, 
           user_id: &str,
           domain: &str,
           resource: &str,
           action: &str
       ) -> Result<bool> {
           // 1. 检查缓存
           let cache_key = format!("perm:{}:{}:{}:{}", user_id, domain, resource, action);
           if let Some(result) = self.cache.get(&cache_key).await? {
               return Ok(result);
           }

           // 2. 验证权限
           let enforcer = self.enforcer.read().await;
           let result = enforcer.enforce((user_id, domain, resource, action))?;

           // 3. 缓存结果
           self.cache.set(&cache_key, result, Duration::from_secs(300)).await?;

           Ok(result)
       }
   }
   ```

## 4. 角色管理

### 4.1 角色定义

1. **系统角色：**
   - 超级管理员
   - 管理员
   - 开发者
   - 运维人员
   - 只读用户

2. **自定义角色：**

   ```rust
   struct Role {
       id: String,
       name: String,
       description: String,
       permissions: Vec<Permission>,
       created_at: DateTime,
       updated_at: DateTime,
   }
   ```

### 4.2 角色分配

1. **用户角色关联：**

   ```rust
   struct UserRole {
       user_id: String,
       role_id: String,
       domain: String,
       created_at: DateTime,
   }
   ```

2. **角色继承：**

   ```rust
   struct RoleHierarchy {
       parent_role: String,
       child_role: String,
       created_at: DateTime,
   }
   ```

## 5. 权限粒度

### 5.1 资源类型

1. **配置资源：**
   - 命名空间
   - 部门
   - 应用
   - 域
   - 环境
   - 配置项

2. **操作类型：**
   - 读取
   - 写入
   - 删除
   - 发布
   - 回滚
   - 审计

### 5.2 权限规则

1. **命名空间权限：**

   ```rust
   enum NamespacePermission {
       Read,
       Write,
       Delete,
       Publish,
       Rollback,
       Audit,
   }
   ```

2. **配置项权限：**

   ```rust
   enum ConfigPermission {
       Read,
       Write,
       Delete,
       Encrypt,
       Decrypt,
   }
   ```

## 6. 权限缓存

### 6.1 缓存策略

1. **缓存内容：**
   - 用户角色
   - 角色权限
   - 权限决策结果

2. **缓存更新：**

   ```rust
   struct PermissionCache {
       cache: Arc<Cache>,
       ttl: Duration,
   }

   impl PermissionCache {
       async fn invalidate_user_permissions(&self, user_id: &str) -> Result<()> {
           let pattern = format!("perm:{}:*", user_id);
           self.cache.delete_pattern(&pattern).await
       }

       async fn invalidate_role_permissions(&self, role_id: &str) -> Result<()> {
           let pattern = format!("perm:*:*:{}", role_id);
           self.cache.delete_pattern(&pattern).await
       }
   }
   ```

### 6.2 缓存优化

1. **多级缓存：**
   - 本地缓存
   - Redis缓存
   - 数据库

2. **缓存预热：**
   - 系统启动时加载
   - 定时刷新
   - 按需加载

## 7. 审计日志

### 7.1 审计内容

1. **认证审计：**
   - 登录尝试
   - 登录成功/失败
   - Token使用

2. **授权审计：**
   - 权限验证
   - 权限变更
   - 角色变更

### 7.2 审计记录

```rust
struct AuditLog {
    id: String,
    user_id: String,
    action: String,
    resource: String,
    status: String,
    details: String,
    ip: String,
    timestamp: DateTime,
}
```

## 8. 安全措施

### 8.1 密码安全

1. **密码策略：**
   - 最小长度
   - 复杂度要求
   - 定期更换
   - 历史密码限制

2. **密码存储：**
   - 使用bcrypt加密
   - 加盐处理
   - 防止彩虹表攻击

### 8.2 访问控制

1. **IP限制：**
   - IP白名单
   - 访问频率限制
   - 地理位置限制

2. **会话管理：**
   - 会话超时
   - 并发登录控制
   - 异常登录检测

## 9. 性能优化

### 9.1 验证优化

1. **批量验证：**

   ```rust
   async fn batch_check_permissions(
       &self,
       user_id: &str,
       domain: &str,
       resources: &[(String, String)]
   ) -> Result<HashMap<String, bool>> {
       let mut results = HashMap::new();
       
       // 并行验证多个权限
       let futures: Vec<_> = resources
           .iter()
           .map(|(resource, action)| {
               self.check_permission(user_id, domain, resource, action)
           })
           .collect();
           
       let results = futures::future::join_all(futures).await;
       
       // 处理结果
       for ((resource, action), result) in resources.iter().zip(results) {
           results.insert(
               format!("{}:{}", resource, action),
               result?
           );
       }
       
       Ok(results)
   }
   ```

2. **缓存优化：**
   - 预加载常用权限
   - 批量缓存更新
   - 智能缓存失效

### 9.2 存储优化

1. **数据库优化：**
   - 索引优化
   - 查询优化
   - 连接池管理

2. **缓存优化：**
   - 缓存分片
   - 缓存预热
   - 缓存更新策略

## 10. 监控告警

### 10.1 监控指标

1. **认证指标：**
   - 登录成功率
   - 认证延迟
   - Token使用率

2. **授权指标：**
   - 权限验证延迟
   - 权限拒绝率
   - 缓存命中率

### 10.2 告警规则

1. **安全告警：**
   - 异常登录
   - 权限变更
   - 暴力破解

2. **性能告警：**
   - 验证延迟过高
   - 缓存命中率低
   - 数据库压力大

## 11. 总结

权限管理系统通过RBAC模型和Casbin实现了灵活的权限控制，通过多级缓存和性能优化确保了系统的高效运行，同时通过完善的审计和监控机制保证了系统的安全性。
