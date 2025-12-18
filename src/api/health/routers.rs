use crate::{
    api::health::{check_db_ready_handler, check_health_handler},
    app::state::AppState,
};
use axum::{Router, routing::get};
use std::sync::Arc;

pub fn health_router(app_state: Arc<AppState>) -> Router {
    let health_routers = Router::new()
        .route("/", get(check_health_handler))
        .route("/ready", get(check_db_ready_handler))
        .with_state(app_state);
    Router::new().nest("/health", health_routers)
}
