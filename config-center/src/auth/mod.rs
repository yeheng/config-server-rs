use anyhow::Result;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::config::AuthConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
    pub roles: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Auth {
    config: AuthConfig,
}

impl Auth {
    pub fn new(config: &AuthConfig) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
        })
    }

    pub fn generate_token(&self, user_id: &str, roles: Vec<String>) -> Result<String> {
        let expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs() + self.config.token_expiration;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
            roles,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_bytes()),
        )?;

        Ok(token)
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let validation = Validation::default();
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.jwt_secret.as_bytes()),
            &validation,
        )?;

        Ok(token_data.claims)
    }
    pub fn hash_password(&self, password: &str) -> Result<String> {
        let hash = bcrypt::hash(password.as_bytes(), self.config.password_hash_cost)?;
        Ok(hash)
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        Ok(bcrypt::verify(password.as_bytes(), hash)?)
    }

    pub async fn check_permission(&self, user_id: &str, resource: &str, action: &str) -> Result<bool> {
        // TODO: Implement RBAC permission check using casbin
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_generation_and_validation() {
        let config = AuthConfig {
            jwt_secret: "test_secret".to_string(),
            token_expiration: 3600,
            password_hash_cost: 10,
            rbac_model: std::path::PathBuf::from("config/rbac_model.conf"),
        };

        let auth = Auth::new(&config).unwrap();
        let user_id = "test_user";
        let roles = vec!["admin".to_string()];

        let token = auth.generate_token(user_id, roles.clone()).unwrap();
        let claims = auth.validate_token(&token).unwrap();

        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.roles, roles);
    }

    #[test]
    fn test_password_hashing() {
        let config = AuthConfig {
            jwt_secret: "test_secret".to_string(),
            token_expiration: 3600,
            password_hash_cost: 10,
            rbac_model: std::path::PathBuf::from("config/rbac_model.conf"),
        };

        let auth = Auth::new(&config).unwrap();
        let password = "test_password";

        let hash = auth.hash_password(password).unwrap();
        assert!(auth.verify_password(password, &hash).unwrap());
        assert!(!auth.verify_password("wrong_password", &hash).unwrap());
    }
}
