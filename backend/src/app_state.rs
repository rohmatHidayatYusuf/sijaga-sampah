use crate::config::AppConfig;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Arc<AppConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // User ID (UUID)
    pub email: String,
    pub roles: Vec<String>,
    pub exp: usize,
}

impl AppState {
    pub fn new(db: PgPool, config: AppConfig) -> Self {
        Self {
            db,
            config: Arc::new(config),
        }
    }

    pub fn generate_jwt(&self, user_id: Uuid, email: &str, roles: Vec<String>) -> Result<String, jsonwebtoken::errors::Error> {
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(self.config.jwt_expiration_hours))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            roles,
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_bytes()),
        )
    }

    pub fn verify_jwt(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.jwt_secret.as_bytes()),
            &Validation::default(),
        )?;
        Ok(token_data.claims)
    }
}
