use std::net::SocketAddr;
use axum::{
    http::{header, HeaderValue, Request, StatusCode},
    middleware::{self, Next},
    response::Response,
};
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app_state;
mod config;
mod dto;
mod errors;
mod middleware as app_middleware;
mod models;
mod repositories;
mod routes;
mod services;

use app_state::AppState;
use config::AppConfig;
use routes::api_router;

/// SEC-07: Security Headers Middleware (OWASP A05)
async fn security_headers_middleware<B>(req: Request<B>, next: Next<B>) -> Response {
    let mut response = next.run(req).await;
    let headers = response.headers_mut();

    headers.insert(
        header::X_FRAME_OPTIONS,
        HeaderValue::from_static("DENY"),
    );
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static("default-src 'self'; frame-ancestors 'none'; object-src 'none';"),
    );
    headers.insert(
        header::STRICT_TRANSPORT_SECURITY,
        HeaderValue::from_static("max-age=31536000; includeSubDomains"),
    );

    response
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Inisialisasi Tracing / Observability
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sijaga_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Memulai SIJAGA SAMPAH - Backend Service v1.0.0 (Hardened)");

    // 2. Load Konfigurasi
    let config = AppConfig::from_env();
    tracing::info!("Konfigurasi server dimuat: {}:{}", config.host, config.port);

    // 3. Inisialisasi Database Connection Pool
    tracing::info!("Menghubungkan ke PostgreSQL: {}", config.database_url);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .connect(&config.database_url)
        .await
        .map_err(|e| {
            tracing::error!("Gagal menyambung ke database: {}", e);
            e
        })?;

    // Jalankan migrasi jika tabel belum ada
    tracing::info!("Menjalankan migrasi database...");
    let _ = sqlx::query("CREATE EXTENSION IF NOT EXISTS pgcrypto").execute(&pool).await;

    // 4. Inisialisasi State & Middleware
    let state = AppState::new(pool, config.clone());

    // SEC-06: Restrict CORS origins (OWASP A05)
    let allowed_origins = [
        "http://localhost:3000".parse().unwrap(),
        "http://127.0.0.1:3000".parse().unwrap(),
    ];
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(allowed_origins))
        .allow_methods(AllowMethods::any())
        .allow_headers(AllowHeaders::any());

    // 5. Susun Router dengan Security Headers Middleware
    let app = api_router(state)
        .layer(middleware::from_fn(security_headers_middleware))
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    // 6. Jalankan Server HTTP
    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
    tracing::info!("SIJAGA SAMPAH API siap melayani di http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("Server dimatikan dengan aman (graceful shutdown).");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Gagal menginstal handler sinyal Ctrl+C");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Gagal menginstal handler sinyal terminate")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
