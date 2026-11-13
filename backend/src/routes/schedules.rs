use crate::{
    app_state::AppState,
    dto::*,
    errors::AppError,
    middleware::AuthUser,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, patch, post},
    Json, Router,
};
use uuid::Uuid;

pub fn schedule_routes() -> Router<AppState> {
    Router::new()
        .route("/collection-schedules", get(list_schedules).post(create_schedule))
        .route("/collection-schedules/:id", patch(update_schedule).delete(delete_schedule))
}

async fn list_schedules(
    State(state): State<AppState>,
    _auth: AuthUser,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, AppError> {
    let rows = sqlx::query!(
        r#"
        SELECT 
            cs.id, cs.title, cs.scheduled_at, cs.status, cs.notes,
            reg.id as region_id, reg.name as region_name,
            u.full_name as responsible_name
        FROM collection_schedules cs
        JOIN regions reg ON reg.id = cs.region_id
        LEFT JOIN users u ON u.id = cs.responsible_user_id
        ORDER BY cs.scheduled_at ASC
        "#
    )
    .fetch_all(&state.db)
    .await?;

    let result = rows.into_iter().map(|r| {
        serde_json::json!({
            "id": r.id,
            "title": r.title,
            "scheduled_at": r.scheduled_at,
            "status": r.status,
            "notes": r.notes,
            "region_id": r.region_id,
            "region_name": r.region_name,
            "responsible_name": r.responsible_name,
        })
    }).collect();

    Ok(Json(ApiResponse { data: result }))
}

async fn create_schedule(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(payload): Json<CreateScheduleRequest>,
) -> Result<(StatusCode, Json<ApiResponse<serde_json::Value>>), AppError> {
    auth.require_role("operator")?;

    let row = sqlx::query!(
        r#"
        INSERT INTO collection_schedules (region_id, responsible_user_id, title, scheduled_at, notes, created_by)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, title, scheduled_at, status
        "#,
        payload.region_id,
        payload.responsible_user_id,
        payload.title,
        payload.scheduled_at,
        payload.notes,
        auth.id
    )
    .fetch_one(&state.db)
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse {
            data: serde_json::json!({
                "id": row.id,
                "title": row.title,
                "scheduled_at": row.scheduled_at,
                "status": row.status
            }),
        }),
    ))
}

async fn update_schedule(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    auth.require_role("operator")?;

    let status = payload.get("status").and_then(|v| v.as_str());
    let notes = payload.get("notes").and_then(|v| v.as_str());

    sqlx::query!(
        r#"
        UPDATE collection_schedules
        SET status = COALESCE($1, status),
            notes = COALESCE($2, notes)
        WHERE id = $3
        "#,
        status,
        notes,
        id
    )
    .execute(&state.db)
    .await?;

    Ok(Json(ApiResponse {
        data: serde_json::json!({ "message": "Jadwal berhasil diperbarui." }),
    }))
}

async fn delete_schedule(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    auth.require_role("operator")?;

    sqlx::query!("DELETE FROM collection_schedules WHERE id = $1", id)
        .execute(&state.db)
        .await?;

    Ok(Json(ApiResponse {
        data: serde_json::json!({ "message": "Jadwal berhasil dihapus." }),
    }))
}
