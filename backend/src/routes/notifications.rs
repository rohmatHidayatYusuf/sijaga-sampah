use crate::{
    app_state::AppState,
    dto::*,
    errors::AppError,
    middleware::AuthUser,
};
use axum::{
    extract::{Path, State},
    routing::{get, patch},
    Json, Router,
};
use uuid::Uuid;

pub fn notification_routes() -> Router<AppState> {
    Router::new()
        .route("/notifications", get(get_notifications))
        .route("/notifications/:id/read", patch(mark_as_read))
}

async fn get_notifications(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, AppError> {
    let rows = sqlx::query!(
        r#"
        SELECT id, title, body, target_type, target_id, read_at, created_at
        FROM notifications
        WHERE user_id = $1
        ORDER BY created_at DESC
        LIMIT 50
        "#,
        auth.id
    )
    .fetch_all(&state.db)
    .await?;

    let result = rows.into_iter().map(|n| {
        serde_json::json!({
            "id": n.id,
            "title": n.title,
            "body": n.body,
            "target_type": n.target_type,
            "target_id": n.target_id,
            "read_at": n.read_at,
            "created_at": n.created_at,
        })
    }).collect();

    Ok(Json(ApiResponse { data: result }))
}

async fn mark_as_read(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    sqlx::query!(
        "UPDATE notifications SET read_at = now() WHERE id = $1 AND user_id = $2",
        id,
        auth.id
    )
    .execute(&state.db)
    .await?;

    Ok(Json(ApiResponse {
        data: serde_json::json!({ "message": "Notifikasi ditandai telah dibaca." }),
    }))
}
