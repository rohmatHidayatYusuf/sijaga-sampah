use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Validation error: {0}")]
    Validation(String),
}

#[derive(Serialize, Deserialize)]
pub struct ErrorDetail {
    pub field: Option<String>,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorPayload {
    pub code: String,
    pub message: String,
    pub details: Option<Vec<ErrorDetail>>,
    pub request_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorEnvelope {
    pub error: ErrorPayload,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg.clone()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg.clone()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, "FORBIDDEN", msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "CONFLICT", msg.clone()),
            AppError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, "VALIDATION_ERROR", msg.clone()),
            AppError::Database(err) => {
                tracing::error!("Database query error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR",
                    "Terjadi kesalahan saat memproses data di server.".to_string(),
                )
            }
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_SERVER_ERROR",
                    "Terjadi gangguan pada layanan server.".to_string(),
                )
            }
        };

        let body = Json(ErrorEnvelope {
            error: ErrorPayload {
                code: code.to_string(),
                message,
                details: None,
                request_id: Some(uuid::Uuid::new_v4().to_string()),
            },
        });

        (status, body).into_response()
    }
}
