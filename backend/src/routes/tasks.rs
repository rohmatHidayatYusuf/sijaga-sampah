use crate::{
    app_state::AppState,
    dto::*,
    errors::AppError,
    middleware::AuthUser,
    repositories::ReportRepository,
    services::ReportService,
};
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use uuid::Uuid;

pub fn task_routes() -> Router<AppState> {
    Router::new()
        .route("/tasks/my", get(get_my_tasks))
        .route("/tasks/:id", get(get_task_detail))
        .route("/tasks/:id/accept", post(accept_task))
        .route("/tasks/:id/evidence", post(submit_evidence))
}

async fn get_my_tasks(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, AppError> {
    let rows = sqlx::query!(
        r#"
        SELECT 
            a.id as assignment_id, a.instructions, a.assigned_at, a.accepted_at, a.completed_at,
            r.id as report_id, r.ticket_number, r.title, r.description, r.status, r.priority,
            r.address_text, r.latitude::float8 as "latitude?", r.longitude::float8 as "longitude?",
            wc.name as category_name, reg.name as region_name
        FROM assignments a
        JOIN reports r ON r.id = a.report_id
        JOIN waste_categories wc ON wc.id = r.category_id
        LEFT JOIN regions reg ON reg.id = r.region_id
        WHERE a.assignee_id = $1 AND a.is_active = TRUE
        ORDER BY a.assigned_at DESC
        "#,
        auth.id
    )
    .fetch_all(&state.db)
    .await?;

    let tasks: Vec<serde_json::Value> = rows.into_iter().map(|row| {
        serde_json::json!({
            "assignment_id": row.assignment_id,
            "report_id": row.report_id,
            "ticket_number": row.ticket_number,
            "title": row.title,
            "description": row.description,
            "status": row.status,
            "priority": row.priority,
            "address_text": row.address_text,
            "latitude": row.latitude,
            "longitude": row.longitude,
            "category_name": row.category_name,
            "region_name": row.region_name,
            "instructions": row.instructions,
            "assigned_at": row.assigned_at,
            "accepted_at": row.accepted_at,
            "completed_at": row.completed_at,
        })
    }).collect();

    Ok(Json(ApiResponse { data: tasks }))
}

async fn get_task_detail(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let row = sqlx::query!(
        r#"
        SELECT 
            a.id as assignment_id, a.instructions, a.assigned_at, a.accepted_at, a.completed_at,
            r.id as report_id, r.ticket_number, r.title, r.description, r.status, r.priority,
            r.address_text, r.latitude::float8 as "latitude?", r.longitude::float8 as "longitude?",
            wc.name as category_name, reg.name as region_name,
            op.full_name as assigned_by_name
        FROM assignments a
        JOIN reports r ON r.id = a.report_id
        JOIN waste_categories wc ON wc.id = r.category_id
        LEFT JOIN regions reg ON reg.id = r.region_id
        JOIN users op ON op.id = a.assigned_by
        WHERE a.id = $1 AND (a.assignee_id = $2 OR $3 = TRUE)
        "#,
        id,
        auth.id,
        auth.has_role("operator")
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Penugasan tidak ditemukan.".to_string()))?;

    Ok(Json(ApiResponse {
        data: serde_json::json!({
            "assignment_id": row.assignment_id,
            "report_id": row.report_id,
            "ticket_number": row.ticket_number,
            "title": row.title,
            "description": row.description,
            "status": row.status,
            "priority": row.priority,
            "address_text": row.address_text,
            "latitude": row.latitude,
            "longitude": row.longitude,
            "category_name": row.category_name,
            "region_name": row.region_name,
            "instructions": row.instructions,
            "assigned_by_name": row.assigned_by_name,
            "assigned_at": row.assigned_at,
            "accepted_at": row.accepted_at,
            "completed_at": row.completed_at,
        }),
    }))
}

async fn accept_task(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let mut tx = state.db.begin().await?;

    let assignment = sqlx::query!(
        "SELECT report_id, assignee_id FROM assignments WHERE id = $1 FOR UPDATE",
        id
    )
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| AppError::NotFound("Tugas tidak ditemukan.".to_string()))?;

    if assignment.assignee_id != auth.id && !auth.has_role("operator") {
        return Err(AppError::Forbidden("Anda bukan petugas yang ditugaskan untuk tugas ini.".to_string()));
    }

    let report = sqlx::query!("SELECT status FROM reports WHERE id = $1 FOR UPDATE", assignment.report_id)
        .fetch_one(&mut *tx)
        .await?;

    ReportService::validate_status_transition(&report.status, "in_progress")?;

    sqlx::query!("UPDATE assignments SET accepted_at = now() WHERE id = $1", id)
        .execute(&mut *tx)
        .await?;

    sqlx::query!("UPDATE reports SET status = 'in_progress', updated_at = now() WHERE id = $1", assignment.report_id)
        .execute(&mut *tx)
        .await?;

    ReportRepository::record_status_history(
        &mut tx,
        assignment.report_id,
        Some(auth.id),
        Some(&report.status),
        "in_progress",
        Some("Tugas diterima dan sedang dikerjakan oleh petugas di lokasi."),
    )
    .await?;

    tx.commit().await?;

    Ok(Json(ApiResponse {
        data: serde_json::json!({ "message": "Tugas berhasil diterima dan dimulai." }),
    }))
}

async fn submit_evidence(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<CompleteTaskRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let mut tx = state.db.begin().await?;

    let assignment = sqlx::query!(
        "SELECT report_id, assignee_id FROM assignments WHERE id = $1 FOR UPDATE",
        id
    )
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| AppError::NotFound("Tugas tidak ditemukan.".to_string()))?;

    if assignment.assignee_id != auth.id && !auth.has_role("operator") {
        return Err(AppError::Forbidden("Anda tidak berhak menyelesaikan tugas ini.".to_string()));
    }

    let report = sqlx::query!("SELECT status FROM reports WHERE id = $1 FOR UPDATE", assignment.report_id)
        .fetch_one(&mut *tx)
        .await?;

    ReportService::validate_status_transition(&report.status, "resolved")?;

    sqlx::query!("UPDATE assignments SET completed_at = now() WHERE id = $1", id)
        .execute(&mut *tx)
        .await?;

    sqlx::query!("UPDATE reports SET status = 'resolved', updated_at = now() WHERE id = $1", assignment.report_id)
        .execute(&mut *tx)
        .await?;

    let note_text = payload.notes.unwrap_or_else(|| "Pekerjaan pembersihan telah diselesaikan oleh petugas.".to_string());
    ReportRepository::record_status_history(
        &mut tx,
        assignment.report_id,
        Some(auth.id),
        Some(&report.status),
        "resolved",
        Some(&note_text),
    )
    .await?;

    tx.commit().await?;

    Ok(Json(ApiResponse {
        data: serde_json::json!({ "message": "Bukti pekerjaan berhasil diunggah dan status menjadi selesai (resolved)." }),
    }))
}
