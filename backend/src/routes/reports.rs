use crate::{
    app_state::AppState,
    dto::*,
    errors::AppError,
    middleware::AuthUser,
    repositories::ReportRepository,
    services::ReportService,
};
use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ReportFilterQuery {
    pub status: Option<String>,
    pub category_id: Option<Uuid>,
    pub region_id: Option<Uuid>,
    pub search: Option<String>,
}

pub fn report_routes() -> Router<AppState> {
    Router::new()
        .route("/reports", post(create_report).get(list_reports))
        .route("/reports/export", get(export_reports_csv))
        .route("/reports/:id", get(get_report_detail))
        .route("/reports/:id/verify", post(verify_report))
        .route("/reports/:id/reject", post(reject_report))
        .route("/reports/:id/assignments", post(assign_report))
        .route("/reports/:id/status", post(update_status))
        .route("/reports/:id/reopen", post(reopen_report))
        .route("/reports/:id/history", get(get_report_history))
}

async fn create_report(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(payload): Json<CreateReportRequest>,
) -> Result<(StatusCode, Json<ApiResponse<ReportCreatedResponse>>), AppError> {
    if payload.title.trim().is_empty() {
        return Err(AppError::BadRequest("Judul laporan wajib diisi.".to_string()));
    }
    if payload.description.trim().is_empty() {
        return Err(AppError::BadRequest("Deskripsi laporan wajib diisi.".to_string()));
    }
    if payload.address_text.is_none() && payload.latitude.is_none() {
        return Err(AppError::BadRequest("Lokasi atau alamat laporan wajib disertakan.".to_string()));
    }

    let report = ReportRepository::create_report(&state.db, auth.id, &payload).await?;

    Ok((StatusCode::CREATED, Json(ApiResponse { data: report })))
}

async fn list_reports(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(filter): Query<ReportFilterQuery>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, AppError> {
    let is_warga = auth.has_role("warga") && !auth.has_role("operator") && !auth.has_role("admin") && !auth.has_role("pimpinan");

    // Fetch reports with category name and region name
    let rows = sqlx::query!(
        r#"
        SELECT 
            r.id, r.ticket_number, r.title, r.description, r.status, r.priority,
            r.address_text, r.latitude::float8 as "latitude?", r.longitude::float8 as "longitude?",
            r.submitted_at, r.updated_at,
            wc.name as category_name,
            reg.name as region_name,
            u.full_name as reporter_name
        FROM reports r
        JOIN waste_categories wc ON wc.id = r.category_id
        LEFT JOIN regions reg ON reg.id = r.region_id
        JOIN users u ON u.id = r.reporter_id
        WHERE ($1::uuid IS NULL OR r.reporter_id = $1)
          AND ($2::text IS NULL OR r.status = $2)
          AND ($3::uuid IS NULL OR r.category_id = $3)
          AND ($4::uuid IS NULL OR r.region_id = $4)
        ORDER BY r.submitted_at DESC
        "#,
        if is_warga { Some(auth.id) } else { None },
        filter.status,
        filter.category_id,
        filter.region_id
    )
    .fetch_all(&state.db)
    .await?;

    let reports: Vec<serde_json::Value> = rows.into_iter().map(|r| {
        serde_json::json!({
            "id": r.id,
            "ticket_number": r.ticket_number,
            "title": r.title,
            "description": r.description,
            "status": r.status,
            "priority": r.priority,
            "address_text": r.address_text,
            "latitude": r.latitude,
            "longitude": r.longitude,
            "category_name": r.category_name,
            "region_name": r.region_name,
            "reporter_name": r.reporter_name,
            "submitted_at": r.submitted_at,
            "updated_at": r.updated_at,
        })
    }).collect();

    Ok(Json(ApiResponse { data: reports }))
}

async fn get_report_detail(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let report = sqlx::query!(
        r#"
        SELECT 
            r.id, r.ticket_number, r.title, r.description, r.status, r.priority,
            r.address_text, r.latitude::float8 as "latitude?", r.longitude::float8 as "longitude?",
            r.rejection_reason, r.submitted_at, r.updated_at, r.reporter_id,
            wc.id as category_id, wc.name as category_name,
            reg.id as "region_id?", reg.name as "region_name?",
            u.full_name as reporter_name, u.phone as reporter_phone
        FROM reports r
        JOIN waste_categories wc ON wc.id = r.category_id
        LEFT JOIN regions reg ON reg.id = r.region_id
        JOIN users u ON u.id = r.reporter_id
        WHERE r.id = $1
        "#,
        id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Laporan tidak ditemukan.".to_string()))?;

    // Authorization check: warga only views their own report
    if auth.has_role("warga") && !auth.has_role("operator") && !auth.has_role("admin") && !auth.has_role("petugas") && !auth.has_role("pimpinan") {
        if report.reporter_id != auth.id {
            return Err(AppError::Forbidden("Anda tidak berhak melihat laporan ini.".to_string()));
        }
    }

    // Fetch history
    let history = sqlx::query!(
        r#"
        SELECT h.id, h.from_status, h.to_status, h.note, h.created_at, u.full_name as changed_by_name
        FROM report_status_history h
        LEFT JOIN users u ON u.id = h.changed_by
        WHERE h.report_id = $1
        ORDER BY h.created_at ASC
        "#,
        id
    )
    .fetch_all(&state.db)
    .await?;

    let history_json: Vec<serde_json::Value> = history.into_iter().map(|h| {
        serde_json::json!({
            "id": h.id,
            "from_status": h.from_status,
            "to_status": h.to_status,
            "note": h.note,
            "created_at": h.created_at,
            "changed_by_name": h.changed_by_name
        })
    }).collect();

    // Fetch active assignment if any
    let assignment = sqlx::query!(
        r#"
        SELECT a.id, a.instructions, a.assigned_at, a.accepted_at, a.completed_at,
               petugas.full_name as assignee_name,
               op.full_name as assigned_by_name
        FROM assignments a
        JOIN users petugas ON petugas.id = a.assignee_id
        JOIN users op ON op.id = a.assigned_by
        WHERE a.report_id = $1 AND a.is_active = TRUE
        LIMIT 1
        "#,
        id
    )
    .fetch_optional(&state.db)
    .await?;

    let assignment_json = assignment.map(|a| {
        serde_json::json!({
            "id": a.id,
            "instructions": a.instructions,
            "assigned_at": a.assigned_at,
            "accepted_at": a.accepted_at,
            "completed_at": a.completed_at,
            "assignee_name": a.assignee_name,
            "assigned_by_name": a.assigned_by_name
        })
    });

    Ok(Json(ApiResponse {
        data: serde_json::json!({
            "id": report.id,
            "ticket_number": report.ticket_number,
            "title": report.title,
            "description": report.description,
            "status": report.status,
            "priority": report.priority,
            "rejection_reason": report.rejection_reason,
            "address_text": report.address_text,
            "latitude": report.latitude,
            "longitude": report.longitude,
            "category_id": report.category_id,
            "category_name": report.category_name,
            "region_id": report.region_id,
            "region_name": report.region_name,
            "reporter_name": report.reporter_name,
            "reporter_phone": report.reporter_phone,
            "submitted_at": report.submitted_at,
            "updated_at": report.updated_at,
            "history": history_json,
            "assignment": assignment_json,
        }),
    }))
}

async fn verify_report(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<VerifyReportRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    auth.require_role("operator")?;

    let mut tx = state.db.begin().await?;

    let cur = sqlx::query!("SELECT status FROM reports WHERE id = $1 FOR UPDATE", id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| AppError::NotFound("Laporan tidak ditemukan.".to_string()))?;

    ReportService::validate_status_transition(&cur.status, "verified")?;

    sqlx::query!(
        r#"
        UPDATE reports
        SET status = 'verified',
            priority = COALESCE($1, priority),
            region_id = COALESCE($2, region_id),
            updated_at = now()
        WHERE id = $3
        "#,
        payload.priority,
        payload.region_id,
        id
    )
    .execute(&mut *tx)
    .await?;

    let note_text = payload.note.unwrap_or_else(|| "Laporan telah diverifikasi oleh Operator.".to_string());
    ReportRepository::record_status_history(&mut tx, id, Some(auth.id), Some(&cur.status), "verified", Some(&note_text)).await?;

    tx.commit().await?;

    Ok(Json(ApiResponse {
        data: serde_json::json!({ "message": "Laporan berhasil diverifikasi." }),
    }))
}

async fn reject_report(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<RejectReportRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    auth.require_role("operator")?;

    if payload.reason.trim().is_empty() {
        return Err(AppError::BadRequest("Alasan penolakan laporan wajib diisi.".to_string()));
    }

    let mut tx = state.db.begin().await?;

    let cur = sqlx::query!("SELECT status FROM reports WHERE id = $1 FOR UPDATE", id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| AppError::NotFound("Laporan tidak ditemukan.".to_string()))?;

    ReportService::validate_status_transition(&cur.status, "rejected")?;

    sqlx::query!(
        r#"
        UPDATE reports
        SET status = 'rejected',
            rejection_reason = $1,
            updated_at = now()
        WHERE id = $2
        "#,
        payload.reason,
        id
    )
    .execute(&mut *tx)
    .await?;

    ReportRepository::record_status_history(&mut tx, id, Some(auth.id), Some(&cur.status), "rejected", Some(&payload.reason)).await?;

    tx.commit().await?;

    Ok(Json(ApiResponse {
        data: serde_json::json!({ "message": "Laporan telah ditolak dengan alasan." }),
    }))
}

async fn assign_report(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<AssignReportRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    auth.require_role("operator")?;

    let mut tx = state.db.begin().await?;

    let cur = sqlx::query!("SELECT status FROM reports WHERE id = $1 FOR UPDATE", id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| AppError::NotFound("Laporan tidak ditemukan.".to_string()))?;

    ReportService::validate_status_transition(&cur.status, "assigned")?;

    // Deactivate previous assignments
    sqlx::query!("UPDATE assignments SET is_active = FALSE WHERE report_id = $1", id)
        .execute(&mut *tx)
        .await?;

    // Create new assignment
    sqlx::query!(
        r#"
        INSERT INTO assignments (report_id, assignee_id, assigned_by, instructions)
        VALUES ($1, $2, $3, $4)
        "#,
        id,
        payload.assignee_id,
        auth.id,
        payload.instructions
    )
    .execute(&mut *tx)
    .await?;

    // Update report
    sqlx::query!(
        r#"
        UPDATE reports
        SET status = 'assigned',
            priority = COALESCE($1, priority),
            updated_at = now()
        WHERE id = $2
        "#,
        payload.priority,
        id
    )
    .execute(&mut *tx)
    .await?;

    ReportRepository::record_status_history(&mut tx, id, Some(auth.id), Some(&cur.status), "assigned", Some("Petugas kebersihan telah ditugaskan.")).await?;

    tx.commit().await?;

    Ok(Json(ApiResponse {
        data: serde_json::json!({ "message": "Petugas berhasil ditugaskan." }),
    }))
}

async fn update_status(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateReportStatusRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let mut tx = state.db.begin().await?;

    let cur = sqlx::query!("SELECT status FROM reports WHERE id = $1 FOR UPDATE", id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| AppError::NotFound("Laporan tidak ditemukan.".to_string()))?;

    // Authorization check: only operator/admin or assigned officer
    let is_operator = auth.has_role("operator");
    let is_assigned_officer: bool = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM assignments WHERE report_id = $1 AND assignee_id = $2 AND is_active = TRUE)",
        id,
        auth.id
    )
    .fetch_one(&mut *tx)
    .await?
    .unwrap_or(false);

    if !is_operator && !is_assigned_officer {
        return Err(AppError::Forbidden("Hanya petugas terkait atau operator yang berhak mengubah status laporan.".to_string()));
    }

    ReportService::validate_status_transition(&cur.status, &payload.status)?;

    sqlx::query!(
        "UPDATE reports SET status = $1, updated_at = now() WHERE id = $2",
        payload.status,
        id
    )
    .execute(&mut *tx)
    .await?;

    ReportRepository::record_status_history(&mut tx, id, Some(auth.id), Some(&cur.status), &payload.status, payload.note.as_deref()).await?;

    tx.commit().await?;

    Ok(Json(ApiResponse {
        data: serde_json::json!({ "message": "Status laporan berhasil diperbarui." }),
    }))
}

async fn reopen_report(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let mut tx = state.db.begin().await?;

    let cur = sqlx::query!("SELECT status, reporter_id FROM reports WHERE id = $1 FOR UPDATE", id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| AppError::NotFound("Laporan tidak ditemukan.".to_string()))?;

    // Authorization check: only operator/admin or the original reporter
    if !auth.has_role("operator") && cur.reporter_id != auth.id {
        return Err(AppError::Forbidden("Anda tidak berhak membuka kembali laporan ini.".to_string()));
    }

    ReportService::validate_status_transition(&cur.status, "reopened")?;

    sqlx::query!(
        "UPDATE reports SET status = 'reopened', updated_at = now() WHERE id = $1",
        id
    )
    .execute(&mut *tx)
    .await?;

    ReportRepository::record_status_history(&mut tx, id, Some(auth.id), Some(&cur.status), "reopened", Some("Laporan dibuka kembali.")).await?;

    tx.commit().await?;

    Ok(Json(ApiResponse {
        data: serde_json::json!({ "message": "Laporan dibuka kembali." }),
    }))
}

async fn get_report_history(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, AppError> {
    let history = sqlx::query!(
        r#"
        SELECT h.id, h.from_status, h.to_status, h.note, h.created_at, u.full_name as changed_by_name
        FROM report_status_history h
        LEFT JOIN users u ON u.id = h.changed_by
        WHERE h.report_id = $1
        ORDER BY h.created_at ASC
        "#,
        id
    )
    .fetch_all(&state.db)
    .await?;

    let result = history.into_iter().map(|h| {
        serde_json::json!({
            "id": h.id,
            "from_status": h.from_status,
            "to_status": h.to_status,
            "note": h.note,
            "created_at": h.created_at,
            "changed_by_name": h.changed_by_name
        })
    }).collect();

    Ok(Json(ApiResponse { data: result }))
}

async fn export_reports_csv(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    if !auth.has_role("operator") && !auth.has_role("pimpinan") && !auth.has_role("admin") {
        return Err(AppError::Forbidden("Anda tidak berhak mengekspor data laporan.".to_string()));
    }

    let rows = sqlx::query!(
        r#"
        SELECT 
            r.ticket_number, r.title, r.status, r.priority,
            wc.name as category_name, reg.name as region_name,
            u.full_name as reporter_name, r.submitted_at
        FROM reports r
        JOIN waste_categories wc ON wc.id = r.category_id
        LEFT JOIN regions reg ON reg.id = r.region_id
        JOIN users u ON u.id = r.reporter_id
        ORDER BY r.submitted_at DESC
        "#
    )
    .fetch_all(&state.db)
    .await?;

    let mut csv_data = String::from("Nomor Tiket,Judul Laporan,Kategori,Wilayah,Status,Prioritas,Pelapor,Tanggal Dibuat\n");
    for row in rows {
        csv_data.push_str(&format!(
            "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
            row.ticket_number,
            row.title.replace('"', "\"\""),
            row.category_name.replace('"', "\"\""),
            row.region_name.unwrap_or_default().replace('"', "\"\""),
            row.status,
            row.priority,
            row.reporter_name.replace('"', "\"\""),
            row.submitted_at.to_rfc3339()
        ));
    }

    Ok((
        [
            (header::CONTENT_TYPE, "text/csv; charset=utf-8"),
            (header::CONTENT_DISPOSITION, "attachment; filename=\"laporan_sijaga_sampah.csv\""),
        ],
        csv_data,
    ))
}
