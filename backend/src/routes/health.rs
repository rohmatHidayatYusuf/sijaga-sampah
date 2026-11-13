use crate::{app_state::AppState, dto::ApiResponse, errors::AppError};
use axum::{extract::State, routing::get, Json, Router};

pub fn health_routes() -> Router<AppState> {
    Router::new()
        .route("/health/live", get(liveness))
        .route("/health/ready", get(readiness))
}

async fn liveness() -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse {
        data: serde_json::json!({
            "status": "UP",
            "service": "sijaga_backend",
            "timestamp": chrono::Utc::now()
        }),
    })
}

async fn readiness(State(state): State<AppState>) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    sqlx::query("SELECT 1").execute(&state.db).await?;

    Ok(Json(ApiResponse {
        data: serde_json::json!({
            "status": "READY",
            "database": "CONNECTED",
            "timestamp": chrono::Utc::now()
        }),
    }))
}
