pub mod auth;
pub mod dashboard;
pub mod health;
pub mod master;
pub mod notifications;
pub mod reports;
pub mod schedules;
pub mod tasks;

use crate::app_state::AppState;
use axum::Router;

pub fn api_router(state: AppState) -> Router {
    let api_v1 = Router::new()
        .merge(auth::auth_routes())
        .merge(reports::report_routes())
        .merge(tasks::task_routes())
        .merge(schedules::schedule_routes())
        .merge(dashboard::dashboard_routes())
        .merge(master::master_routes())
        .merge(notifications::notification_routes())
        .with_state(state.clone());

    Router::new()
        .nest("/api/v1", api_v1)
        .merge(health::health_routes().with_state(state))
}
