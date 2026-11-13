use crate::{app_state::AppState, errors::AppError};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{header::AUTHORIZATION, request::Parts},
};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
    pub roles: Vec<String>,
}

impl AuthUser {
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role || r == "admin")
    }

    pub fn require_role(&self, role: &str) -> Result<(), AppError> {
        if self.has_role(role) {
            Ok(())
        } else {
            Err(AppError::Forbidden(format!(
                "Akses ditolak: role '{}' diperlukan.",
                role
            )))
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("Token otentikasi tidak ditemukan.".to_string()))?;

        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::Unauthorized("Format token harus 'Bearer <token>'.".to_string()));
        }

        let token = &auth_header["Bearer ".len()..];
        let claims = app_state
            .verify_jwt(token)
            .map_err(|_| AppError::Unauthorized("Token tidak valid atau sudah kedaluwarsa.".to_string()))?;

        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::Unauthorized("Format user id dalam token tidak valid.".to_string()))?;

        Ok(AuthUser {
            id: user_id,
            email: claims.email,
            roles: claims.roles,
        })
    }
}
