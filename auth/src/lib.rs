use async_trait::async_trait;
use casbin::prelude::*;
use common::Result;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub role: String,
}

/// Authentication service trait
#[async_trait]
pub trait AuthService: Send + Sync {
    /// Generate JWT token
    async fn generate_token(&self, user_id: &str, role: &str) -> Result<String>;

    /// Validate JWT token
    async fn validate_token(&self, token: &str) -> Result<Claims>;
}

/// Authorization service trait
#[async_trait]
pub trait AuthzService: Send + Sync {
    /// Check if user has permission to perform action on resource
    async fn check_permission(&self, user: &str, resource: &str, action: &str) -> Result<bool>;

    /// Add policy for role
    async fn add_policy(
        &self,
        role: &str,
        resource: &str,
        action: &str,
        effect: &str,
    ) -> Result<bool>;

    /// Remove policy for role
    async fn remove_policy(
        &self,
        role: &str,
        resource: &str,
        action: &str,
        effect: &str,
    ) -> Result<bool>;
}

/// JWT authentication service implementation
pub struct JwtAuthService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    token_expiry: i64,
}

impl JwtAuthService {
    pub fn new(secret: &[u8], token_expiry: i64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
            token_expiry,
        }
    }
}

#[async_trait]
impl AuthService for JwtAuthService {
    async fn generate_token(&self, user_id: &str, role: &str) -> Result<String> {
        let claims = Claims {
            sub: user_id.to_string(),
            exp: chrono::Utc::now().timestamp() + self.token_expiry,
            role: role.to_string(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| common::Error::Auth(e.to_string()))
    }

    async fn validate_token(&self, token: &str) -> Result<Claims> {
        let validation = Validation::default();
        decode::<Claims>(token, &self.decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|e| common::Error::Auth(e.to_string()))
    }
}

/// Casbin authorization service implementation
pub struct CasbinAuthzService {
    enforcer: Arc<Enforcer>,
}

impl CasbinAuthzService {
    pub async fn new(model: &str, policy: &str) -> Result<Self> {
        let enforcer = Enforcer::new(model, policy)
            .await
            .map_err(|e| common::Error::Authorization(e.to_string()))?;

        Ok(Self {
            enforcer: Arc::new(enforcer),
        })
    }
}

#[async_trait]
impl AuthzService for CasbinAuthzService {
    async fn check_permission(&self, user: &str, resource: &str, action: &str) -> Result<bool> {
        self.enforcer
            .enforce((user, resource, action))
            .map_err(|e| common::Error::Authorization(e.to_string()))
    }

    async fn add_policy(
        &self,
        role: &str,
        resource: &str,
        action: &str,
        effect: &str,
    ) -> Result<bool> {
        self.enforcer
            .add_policy(vec![
                role.to_string(),
                resource.to_string(),
                action.to_string(),
                effect.to_string(),
            ])
            .await
            .map_err(|e| common::Error::Authorization(e.to_string()))
    }

    async fn remove_policy(
        &self,
        role: &str,
        resource: &str,
        action: &str,
        effect: &str,
    ) -> Result<bool> {
        self.enforcer
            .remove_policy(vec![
                role.to_string(),
                resource.to_string(),
                action.to_string(),
                effect.to_string(),
            ])
            .await
            .map_err(|e| common::Error::Authorization(e.to_string()))
    }
}
