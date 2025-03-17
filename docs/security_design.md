# 安全设计文档

## 1. 概述

本设计文档详细描述了分布式配置中心的安全机制，包括认证、授权、加密、审计等关键安全特性，确保系统的安全性和数据保护。

### 1.1 设计目标

- **安全性：** 确保系统和数据的安全性
- **可用性：** 安全机制不影响系统正常使用
- **可扩展性：** 支持灵活的安全策略配置
- **合规性：** 满足安全合规要求
- **可审计性：** 支持安全事件的追踪和审计

### 1.2 核心组件

- **认证服务：** 负责用户身份认证
- **授权服务：** 负责访问权限控制
- **加密服务：** 负责数据加密解密
- **审计服务：** 负责安全事件记录
- **安全监控：** 负责安全状态监控

## 2. 认证机制

### 2.1 认证方式

```rust
#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expiration: Duration,
    pub api_key_header: String,
    pub session_timeout: Duration,
}

#[derive(Debug, Clone)]
pub struct AuthService {
    config: Arc<AuthConfig>,
    db: Arc<Database>,
    cache: Arc<Redis>,
}

impl AuthService {
    pub async fn authenticate(&self, credentials: &Credentials) -> Result<AuthToken> {
        match credentials {
            Credentials::Password { username, password } => {
                self.authenticate_password(username, password).await
            }
            Credentials::ApiKey { key } => {
                self.authenticate_api_key(key).await
            }
            Credentials::OAuth2 { token } => {
                self.authenticate_oauth2(token).await
            }
        }
    }

    async fn authenticate_password(&self, username: &str, password: &str) -> Result<AuthToken> {
        // 获取用户信息
        let user = self.db.get_user_by_username(username).await?;
        
        // 验证密码
        if !self.verify_password(password, &user.password_hash)? {
            return Err(AuthError::InvalidCredentials);
        }
        
        // 生成JWT令牌
        let token = self.generate_jwt_token(&user)?;
        
        // 保存会话信息
        self.save_session(&user.id, &token).await?;
        
        Ok(token)
    }

    async fn authenticate_api_key(&self, key: &str) -> Result<AuthToken> {
        // 验证API密钥
        let api_key = self.db.get_api_key(key).await?;
        
        // 检查API密钥是否过期
        if api_key.is_expired() {
            return Err(AuthError::ApiKeyExpired);
        }
        
        // 生成JWT令牌
        let token = self.generate_jwt_token_for_api_key(&api_key)?;
        
        Ok(token)
    }

    async fn authenticate_oauth2(&self, token: &str) -> Result<AuthToken> {
        // 验证OAuth2令牌
        let oauth2_info = self.verify_oauth2_token(token).await?;
        
        // 生成JWT令牌
        let token = self.generate_jwt_token_for_oauth2(&oauth2_info)?;
        
        Ok(token)
    }

    fn generate_jwt_token(&self, user: &User) -> Result<AuthToken> {
        let claims = Claims {
            sub: user.id.clone(),
            exp: (Utc::now() + self.config.jwt_expiration).timestamp() as usize,
            roles: user.roles.clone(),
        };
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_bytes()),
        )?;
        
        Ok(AuthToken::Jwt(token))
    }

    async fn save_session(&self, user_id: &str, token: &AuthToken) -> Result<()> {
        let session = Session {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            token: token.clone(),
            created_at: Utc::now(),
            expires_at: Utc::now() + self.config.session_timeout,
        };
        
        // 保存会话到数据库
        self.db.save_session(&session).await?;
        
        // 保存会话到缓存
        self.cache.set(
            &format!("session:{}", session.id),
            &session,
            Some(self.config.session_timeout),
        ).await?;
        
        Ok(())
    }
}
```

### 2.2 令牌管理

```rust
impl AuthService {
    pub async fn validate_token(&self, token: &AuthToken) -> Result<Claims> {
        match token {
            AuthToken::Jwt(jwt) => {
                // 验证JWT令牌
                let claims = decode::<Claims>(
                    jwt,
                    &DecodingKey::from_secret(self.config.jwt_secret.as_bytes()),
                    &Validation::default(),
                )?;
                
                // 检查令牌是否过期
                if claims.claims.exp < Utc::now().timestamp() as usize {
                    return Err(AuthError::TokenExpired);
                }
                
                Ok(claims.claims)
            }
            AuthToken::ApiKey(key) => {
                // 验证API密钥
                let api_key = self.db.get_api_key(key).await?;
                
                // 检查API密钥是否过期
                if api_key.is_expired() {
                    return Err(AuthError::ApiKeyExpired);
                }
                
                Ok(Claims {
                    sub: api_key.user_id.clone(),
                    exp: api_key.expires_at.timestamp() as usize,
                    roles: api_key.roles.clone(),
                })
            }
        }
    }

    pub async fn refresh_token(&self, token: &AuthToken) -> Result<AuthToken> {
        // 验证原令牌
        let claims = self.validate_token(token).await?;
        
        // 生成新令牌
        let new_token = self.generate_jwt_token(&claims)?;
        
        // 更新会话
        self.update_session(&claims.sub, &new_token).await?;
        
        Ok(new_token)
    }

    pub async fn revoke_token(&self, token: &AuthToken) -> Result<()> {
        // 验证令牌
        let claims = self.validate_token(token).await?;
        
        // 删除会话
        self.delete_session(&claims.sub).await?;
        
        Ok(())
    }
}
```

## 3. 授权机制

### 3.1 RBAC模型

```rust
#[derive(Debug, Clone)]
pub struct RBACService {
    db: Arc<Database>,
    cache: Arc<Redis>,
    enforcer: Arc<RwLock<Enforcer>>,
}

impl RBACService {
    pub async fn check_permission(&self, user_id: &str, 
                                resource: &str, action: &str) -> Result<bool> {
        // 获取用户角色
        let roles = self.get_user_roles(user_id).await?;
        
        // 检查权限
        for role in roles {
            if self.enforcer.read().await
                .enforce(role, resource, action)? {
                return Ok(true);
            }
        }
        
        Ok(false)
    }

    async fn get_user_roles(&self, user_id: &str) -> Result<Vec<String>> {
        // 从缓存获取
        if let Some(roles) = self.cache.get(&format!("user:{}:roles", user_id)).await? {
            return Ok(roles);
        }
        
        // 从数据库获取
        let roles = self.db.get_user_roles(user_id).await?;
        
        // 更新缓存
        self.cache.set(
            &format!("user:{}:roles", user_id),
            &roles,
            Some(Duration::from_secs(3600)),
        ).await?;
        
        Ok(roles)
    }

    pub async fn add_role(&self, role: &Role) -> Result<()> {
        // 保存角色
        self.db.save_role(role).await?;
        
        // 更新权限缓存
        self.update_permission_cache().await?;
        
        Ok(())
    }

    pub async fn assign_role(&self, user_id: &str, role_id: &str) -> Result<()> {
        // 分配角色
        self.db.assign_role(user_id, role_id).await?;
        
        // 清除用户角色缓存
        self.cache.delete(&format!("user:{}:roles", user_id)).await?;
        
        Ok(())
    }

    async fn update_permission_cache(&self) -> Result<()> {
        // 获取所有权限规则
        let rules = self.db.get_permission_rules().await?;
        
        // 更新权限缓存
        self.cache.set(
            "permission_rules",
            &rules,
            Some(Duration::from_secs(3600)),
        ).await?;
        
        Ok(())
    }
}
```

### 3.2 权限策略

```rust
impl RBACService {
    pub async fn add_permission(&self, role: &str, resource: &str, 
                              action: &str) -> Result<()> {
        // 添加权限规则
        let rule = PermissionRule {
            role: role.to_string(),
            resource: resource.to_string(),
            action: action.to_string(),
            effect: Effect::Allow,
        };
        
        self.db.save_permission_rule(&rule).await?;
        
        // 更新权限缓存
        self.update_permission_cache().await?;
        
        Ok(())
    }

    pub async fn remove_permission(&self, role: &str, resource: &str, 
                                 action: &str) -> Result<()> {
        // 删除权限规则
        self.db.delete_permission_rule(role, resource, action).await?;
        
        // 更新权限缓存
        self.update_permission_cache().await?;
        
        Ok(())
    }

    pub async fn get_role_permissions(&self, role: &str) -> Result<Vec<PermissionRule>> {
        // 获取角色权限
        let rules = self.db.get_role_permissions(role).await?;
        
        Ok(rules)
    }

    pub async fn check_resource_permission(&self, user_id: &str, 
                                         resource: &str) -> Result<Vec<String>> {
        // 获取用户角色
        let roles = self.get_user_roles(user_id).await?;
        
        // 获取允许的操作
        let mut allowed_actions = Vec::new();
        
        for role in roles {
            let actions = self.db.get_role_resource_actions(role, resource).await?;
            allowed_actions.extend(actions);
        }
        
        Ok(allowed_actions)
    }
}
```

## 4. 数据加密

### 4.1 加密服务

```rust
#[derive(Debug, Clone)]
pub struct EncryptionService {
    config: Arc<EncryptionConfig>,
    key_manager: Arc<KeyManager>,
}

impl EncryptionService {
    pub async fn encrypt(&self, data: &str) -> Result<String> {
        // 获取加密密钥
        let key = self.key_manager.get_current_key().await?;
        
        // 加密数据
        let cipher = Aes256Gcm::new(key.as_slice());
        let nonce = Nonce::from_slice(&self.generate_nonce());
        
        let ciphertext = cipher.encrypt(nonce, data.as_bytes())?;
        
        // 组合加密结果
        let mut result = Vec::new();
        result.extend_from_slice(nonce);
        result.extend_from_slice(&ciphertext);
        
        Ok(base64::encode(&result))
    }

    pub async fn decrypt(&self, encrypted_data: &str) -> Result<String> {
        // 解码加密数据
        let data = base64::decode(encrypted_data)?;
        
        // 提取nonce和密文
        let (nonce, ciphertext) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce);
        
        // 获取解密密钥
        let key = self.key_manager.get_current_key().await?;
        
        // 解密数据
        let cipher = Aes256Gcm::new(key.as_slice());
        let plaintext = cipher.decrypt(nonce, ciphertext)?;
        
        Ok(String::from_utf8(plaintext)?)
    }

    async fn generate_nonce(&self) -> [u8; 12] {
        let mut nonce = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce);
        nonce
    }
}
```

### 4.2 密钥管理

```rust
#[derive(Debug, Clone)]
pub struct KeyManager {
    db: Arc<Database>,
    cache: Arc<Redis>,
}

impl KeyManager {
    pub async fn get_current_key(&self) -> Result<Vec<u8>> {
        // 从缓存获取
        if let Some(key) = self.cache.get("encryption_key").await? {
            return Ok(key);
        }
        
        // 从数据库获取
        let key = self.db.get_current_encryption_key().await?;
        
        // 更新缓存
        self.cache.set(
            "encryption_key",
            &key,
            Some(Duration::from_secs(3600)),
        ).await?;
        
        Ok(key)
    }

    pub async fn rotate_key(&self) -> Result<()> {
        // 生成新密钥
        let new_key = self.generate_key()?;
        
        // 保存新密钥
        self.db.save_encryption_key(&new_key).await?;
        
        // 更新缓存
        self.cache.set(
            "encryption_key",
            &new_key,
            Some(Duration::from_secs(3600)),
        ).await?;
        
        // 标记旧密钥为过期
        self.db.mark_old_key_expired().await?;
        
        Ok(())
    }

    fn generate_key(&self) -> Result<Vec<u8>> {
        let mut key = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut key);
        Ok(key)
    }
}
```

## 5. 安全审计

### 5.1 审计日志

```rust
#[derive(Debug, Clone)]
pub struct AuditService {
    db: Arc<Database>,
    cache: Arc<Redis>,
}

impl AuditService {
    pub async fn log_event(&self, event: &AuditEvent) -> Result<()> {
        // 创建审计日志
        let log = AuditLog {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            user_id: event.user_id.clone(),
            action: event.action.clone(),
            resource: event.resource.clone(),
            details: event.details.clone(),
            ip_address: event.ip_address.clone(),
            user_agent: event.user_agent.clone(),
        };
        
        // 保存日志
        self.db.save_audit_log(&log).await?;
        
        // 更新审计统计
        self.update_audit_stats(&log).await?;
        
        Ok(())
    }

    async fn update_audit_stats(&self, log: &AuditLog) -> Result<()> {
        // 更新用户操作统计
        self.update_user_stats(log).await?;
        
        // 更新资源访问统计
        self.update_resource_stats(log).await?;
        
        // 更新操作类型统计
        self.update_action_stats(log).await?;
        
        Ok(())
    }

    pub async fn query_audit_logs(&self, query: &AuditQuery) -> Result<Vec<AuditLog>> {
        // 构建查询条件
        let conditions = self.build_query_conditions(query)?;
        
        // 查询日志
        let logs = self.db.query_audit_logs(&conditions).await?;
        
        Ok(logs)
    }

    pub async fn export_audit_logs(&self, query: &AuditQuery) -> Result<Vec<u8>> {
        // 查询日志
        let logs = self.query_audit_logs(query).await?;
        
        // 导出为CSV格式
        let mut writer = Vec::new();
        let mut csv_writer = csv::Writer::from_writer(&mut writer);
        
        for log in logs {
            csv_writer.write_record(&[
                log.timestamp.to_rfc3339(),
                &log.user_id,
                &log.action,
                &log.resource,
                &log.details,
                &log.ip_address,
                &log.user_agent,
            ])?;
        }
        
        csv_writer.flush()?;
        
        Ok(writer)
    }
}
```

### 5.2 安全监控

```rust
#[derive(Debug, Clone)]
pub struct SecurityMonitor {
    metrics: Arc<Metrics>,
    alert_thresholds: HashMap<String, f64>,
}

impl SecurityMonitor {
    pub async fn monitor_security_status(&self) -> Result<()> {
        // 收集安全指标
        let metrics = self.collect_security_metrics().await?;
        
        // 检查告警阈值
        self.check_alert_thresholds(&metrics).await?;
        
        // 更新监控面板
        self.update_dashboard(&metrics).await?;
        
        Ok(())
    }

    async fn collect_security_metrics(&self) -> Result<HashMap<String, f64>> {
        let mut metrics = HashMap::new();
        
        // 收集认证失败次数
        metrics.insert("auth_failures".to_string(), 
            self.get_auth_failure_count().await?);
        
        // 收集权限检查失败次数
        metrics.insert("permission_denials".to_string(), 
            self.get_permission_denial_count().await?);
        
        // 收集异常访问次数
        metrics.insert("abnormal_accesses".to_string(), 
            self.get_abnormal_access_count().await?);
        
        // 收集加密操作次数
        metrics.insert("encryption_operations".to_string(), 
            self.get_encryption_operation_count().await?);
        
        Ok(metrics)
    }

    async fn check_alert_thresholds(&self, metrics: &HashMap<String, f64>) -> Result<()> {
        for (metric, value) in metrics {
            if let Some(threshold) = self.alert_thresholds.get(metric) {
                if value > threshold {
                    self.send_security_alert(metric, *value, *threshold).await?;
                }
            }
        }
        
        Ok(())
    }
}
```

## 6. 总结

安全设计通过认证、授权、加密和审计机制，确保了配置中心的安全性和数据保护，提供了全面的安全防护。
