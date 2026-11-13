use crate::{
    app_state::AppState,
    dto::*,
    errors::AppError,
    middleware::AuthUser,
};
use axum::{extract::State, routing::get, Json, Router};

pub fn dashboard_routes() -> Router<AppState> {
    Router::new()
        .route("/dashboard/summary", get(get_summary))
        .route("/dashboard/trends", get(get_trends))
        .route("/dashboard/map-points", get(get_map_points))
}

async fn get_summary(
    State(state): State<AppState>,
    _auth: AuthUser,
) -> Result<Json<ApiResponse<DashboardSummary>>, AppError> {
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM reports")
        .fetch_one(&state.db)
        .await?;

    let submitted: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM reports WHERE status IN ('submitted', 'under_review', 'verified')")
        .fetch_one(&state.db)
        .await?;

    let in_progress: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM reports WHERE status IN ('assigned', 'in_progress')")
        .fetch_one(&state.db)
        .await?;

    let resolved: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM reports WHERE status IN ('resolved', 'closed')")
        .fetch_one(&state.db)
        .await?;

    let rejected: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM reports WHERE status = 'rejected'")
        .fetch_one(&state.db)
        .await?;

    let active_officers: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(DISTINCT ur.user_id)
        FROM user_roles ur
        JOIN roles r ON r.id = ur.role_id
        JOIN users u ON u.id = ur.user_id
        WHERE r.code = 'petugas' AND u.is_active = TRUE
        "#
    )
    .fetch_one(&state.db)
    .await?;

    let resolution_rate = if total > 0 {
        ((resolved as f64) / (total as f64)) * 100.0
    } else {
        0.0
    };

    Ok(Json(ApiResponse {
        data: DashboardSummary {
            total_reports: total,
            submitted_reports: submitted,
            in_progress_reports: in_progress,
            resolved_reports: resolved,
            rejected_reports: rejected,
            resolution_rate_percentage: (resolution_rate * 10.0).round() / 10.0,
            active_officers,
        },
    }))
}

async fn get_trends(
    State(state): State<AppState>,
    _auth: AuthUser,
) -> Result<Json<ApiResponse<Vec<TrendPoint>>>, AppError> {
    let rows = sqlx::query!(
        r#"
        SELECT 
            TO_CHAR(DATE_TRUNC('day', submitted_at), 'YYYY-MM-DD') as day,
            COUNT(*) as count
        FROM reports
        WHERE submitted_at >= now() - INTERVAL '7 days'
        GROUP BY DATE_TRUNC('day', submitted_at)
        ORDER BY DATE_TRUNC('day', submitted_at) ASC
        "#
    )
    .fetch_all(&state.db)
    .await?;

    let trends = rows
        .into_iter()
        .map(|r| TrendPoint {
            date: r.day.unwrap_or_default(),
            count: r.count.unwrap_or(0),
        })
        .collect();

    Ok(Json(ApiResponse { data: trends }))
}

async fn get_map_points(
    State(state): State<AppState>,
    _auth: AuthUser,
) -> Result<Json<ApiResponse<Vec<MapPoint>>>, AppError> {
    let rows = sqlx::query!(
        r#"
        SELECT 
            r.id, r.ticket_number, r.title, r.status, r.priority,
            r.latitude::float8 as "latitude!", r.longitude::float8 as "longitude!",
            wc.name as category_name
        FROM reports r
        JOIN waste_categories wc ON wc.id = r.category_id
        WHERE r.latitude IS NOT NULL AND r.longitude IS NOT NULL
        ORDER BY r.submitted_at DESC
        LIMIT 100
        "#
    )
    .fetch_all(&state.db)
    .await?;

    let points = rows
        .into_iter()
        .map(|r| MapPoint {
            id: r.id,
            ticket_number: r.ticket_number,
            title: r.title,
            category_name: r.category_name,
            status: r.status,
            priority: r.priority,
            latitude: r.latitude,
            longitude: r.longitude,
        })
        .collect();

    Ok(Json(ApiResponse { data: points }))
}
