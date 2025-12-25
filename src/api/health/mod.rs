use crate::app::{result::AppResult, state::AppState};
use axum::{
    extract::{OriginalUri, State},
    response::IntoResponse,
};
use std::sync::Arc;

pub mod routers;

pub async fn check_health_handler(
    State(app_state): State<Arc<AppState>>,
    uri: OriginalUri,
) -> AppResult<impl IntoResponse> {
    app_state.services.health.check_health_service(uri).await
}

pub async fn check_db_ready_handler(
    State(app_state): State<Arc<AppState>>,
    uri: OriginalUri,
) -> AppResult<impl IntoResponse> {
    app_state.services.health.check_db_ready_service(uri).await
}
