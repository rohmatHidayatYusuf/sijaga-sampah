use crate::{
    app_state::AppState,
    dto::*,
    errors::AppError,
    middleware::AuthUser,
    repositories::AuthRepository,
    services::AuthService,
};
use axum::{extract::State, http::StatusCode, routing::{get, patch, post}, Json, Router};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))
        .route("/auth/me", get(get_me))
        .route("/users/me", patch(update_me))
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<ApiResponse<AuthResponse>>), AppError> {
    if payload.full_name.trim().is_empty() {
        return Err(AppError::BadRequest("Nama lengkap wajib diisi.".to_string()));
    }
    if payload.email.trim().is_empty() || !payload.email.contains('@') {
        return Err(AppError::BadRequest("Format email tidak valid.".to_string()));
    }
    // SEC-09: Password complexity validation
    if payload.password.len() < 8 {
        return Err(AppError::BadRequest("Password minimal 8 karakter.".to_string()));
    }
    let has_letter = payload.password.chars().any(|c| c.is_alphabetic());
    let has_digit = payload.password.chars().any(|c| c.is_numeric());
    if !has_letter || !has_digit {
        return Err(AppError::BadRequest("Password harus mengandung kombinasi huruf dan angka.".to_string()));
    }

    if let Some(_) = AuthRepository::find_user_by_email(&state.db, &payload.email).await? {
        return Err(AppError::Conflict("Email sudah terdaftar dalam sistem.".to_string()));
    }

    let password_hash = AuthService::hash_password(&payload.password)?;
    let user = AuthRepository::create_user_with_role(
        &state.db,
        &payload.full_name,
        &payload.email,
        payload.phone.as_deref(),
        &password_hash,
        "warga",
    )
    .await?;

    // SEC-05: Audit log registration
    let _ = sqlx::query!(
        "INSERT INTO audit_logs (actor_id, action, entity_type, entity_id, metadata) VALUES ($1, 'USER_REGISTER', 'users', $2, $3)",
        user.id,
        user.id,
        serde_json::json!({ "email": user.email, "role": "warga" })
    )
    .execute(&state.db)
    .await;

    let roles = vec!["warga".to_string()];
    let token = state
        .generate_jwt(user.id, &user.email, roles.clone())
        .map_err(|e| AppError::Internal(format!("Gagal membuat token: {}", e)))?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse {
            data: AuthResponse {
                token,
                user: UserProfileResponse {
                    id: user.id,
                    full_name: user.full_name,
                    email: user.email,
                    phone: user.phone,
                    roles,
                },
            },
        }),
    ))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    let user = AuthRepository::find_user_by_email(&state.db, &payload.email)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Kredensial email atau password salah.".to_string()))?;

    if !user.is_active {
        return Err(AppError::Forbidden("Akun Anda telah dinonaktifkan.".to_string()));
    }

    let is_valid = AuthService::verify_password(&payload.password, &user.password_hash)?;
    if !is_valid {
        return Err(AppError::Unauthorized("Kredensial email atau password salah.".to_string()));
    }

    let roles = AuthRepository::find_user_roles(&state.db, user.id).await?;
    let token = state
        .generate_jwt(user.id, &user.email, roles.clone())
        .map_err(|e| AppError::Internal(format!("Gagal membuat token JWT: {}", e)))?;

    // SEC-05: Audit log successful login
    let _ = sqlx::query!(
        "INSERT INTO audit_logs (actor_id, action, entity_type, entity_id, metadata) VALUES ($1, 'USER_LOGIN_SUCCESS', 'users', $2, $3)",
        user.id,
        user.id,
        serde_json::json!({ "email": user.email, "roles": roles })
    )
    .execute(&state.db)
    .await;

    Ok(Json(ApiResponse {
        data: AuthResponse {
            token,
            user: UserProfileResponse {
                id: user.id,
                full_name: user.full_name,
                email: user.email,
                phone: user.phone,
                roles,
            },
        },
    }))
}

async fn logout() -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    Ok(Json(ApiResponse {
        data: serde_json::json!({
            "message": "Sesi berhasil diakhiri."
        }),
    }))
}

async fn get_me(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<ApiResponse<UserProfileResponse>>, AppError> {
    let user = sqlx::query!(
        "SELECT id, full_name, email, phone FROM users WHERE id = $1",
        auth.id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Pengguna tidak ditemukan.".to_string()))?;

    let roles = AuthRepository::find_user_roles(&state.db, auth.id).await?;

    Ok(Json(ApiResponse {
        data: UserProfileResponse {
            id: user.id,
            full_name: user.full_name,
            email: user.email,
            phone: user.phone,
            roles,
        },
    }))
}

async fn update_me(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    sqlx::query!(
        r#"
        UPDATE users
        SET full_name = COALESCE($1, full_name),
            phone = COALESCE($2, phone),
            updated_at = now()
        WHERE id = $3
        "#,
        payload.full_name,
        payload.phone,
        auth.id
    )
    .execute(&state.db)
    .await?;

    Ok(Json(ApiResponse {
        data: serde_json::json!({ "message": "Profil berhasil diperbarui." }),
    }))
}
