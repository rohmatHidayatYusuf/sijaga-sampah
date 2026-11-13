use crate::{
    app_state::AppState,
    dto::*,
    errors::AppError,
    repositories::MasterRepository,
};
use axum::{extract::State, routing::get, Json, Router};

pub fn master_routes() -> Router<AppState> {
    Router::new()
        .route("/master/categories", get(get_categories))
        .route("/master/regions", get(get_regions))
        .route("/master/officers", get(get_officers))
}

async fn get_categories(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, AppError> {
    let categories = MasterRepository::list_categories(&state.db).await?;
    let json_items: Vec<serde_json::Value> = categories
        .into_iter()
        .map(|c| {
            serde_json::json!({
                "id": c.id,
                "code": c.code,
                "name": c.name,
                "description": c.description,
            })
        })
        .collect();

    Ok(Json(ApiResponse { data: json_items }))
}

async fn get_regions(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, AppError> {
    let regions = MasterRepository::list_regions(&state.db).await?;
    let json_items: Vec<serde_json::Value> = regions
        .into_iter()
        .map(|r| {
            serde_json::json!({
                "id": r.id,
                "code": r.code,
                "name": r.name,
                "region_type": r.region_type,
            })
        })
        .collect();

    Ok(Json(ApiResponse { data: json_items }))
}

async fn get_officers(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, AppError> {
    let rows = sqlx::query!(
        r#"
        SELECT u.id, u.full_name, u.phone, u.email
        FROM users u
        JOIN user_roles ur ON ur.user_id = u.id
        JOIN roles r ON r.id = ur.role_id
        WHERE r.code = 'petugas' AND u.is_active = TRUE
        ORDER BY u.full_name ASC
        "#
    )
    .fetch_all(&state.db)
    .await?;

    let officers = rows
        .into_iter()
        .map(|r| {
            serde_json::json!({
                "id": r.id,
                "full_name": r.full_name,
                "phone": r.phone,
                "email": r.email,
            })
        })
        .collect();

    Ok(Json(ApiResponse { data: officers }))
}
