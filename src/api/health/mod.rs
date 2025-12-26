use crate::app::{result::AppResult, state::AppState};
use axum::{
    extract::{ConnectInfo, OriginalUri, State},
    response::IntoResponse,
};
use std::{net::SocketAddr, sync::Arc};

pub mod routers;

pub async fn check_health_handler(
    State(app_state): State<Arc<AppState>>,
    uri: OriginalUri,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> AppResult<impl IntoResponse> {
    app_state
        .services
        .health
        .check_health_service(uri.to_string(), addr.to_string())
        .await
}

pub async fn check_db_ready_handler(
    State(app_state): State<Arc<AppState>>,
    uri: OriginalUri,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> AppResult<impl IntoResponse> {
    app_state
        .services
        .health
        .check_db_ready_service(uri.to_string(), addr.to_string())
        .await
}
