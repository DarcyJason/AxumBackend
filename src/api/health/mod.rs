use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};

use crate::app::{result::AppResult, state::AppState};

pub mod routers;

pub async fn health_check_handler(
    State(app_state): State<Arc<AppState>>,
) -> AppResult<impl IntoResponse> {
    app_state.services.health.health_check().await
}
