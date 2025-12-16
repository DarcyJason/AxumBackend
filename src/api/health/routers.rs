use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{api::health::health_check_handler, app::state::AppState};

pub fn health_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health_check_handler))
        .with_state(app_state)
}
