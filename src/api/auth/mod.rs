use crate::{
    app::{result::AppResult, state::AppState},
    domain::request::auth::{LoginRequest, RegisterRequest},
};
use axum::{
    Json,
    extract::{OriginalUri, State},
    response::IntoResponse,
};
use axum_client_ip::ClientIp;
use std::sync::Arc;

pub mod routers;
pub mod validation;

pub async fn register_handler(
    State(app_state): State<Arc<AppState>>,
    uri: OriginalUri,
    ClientIp(ip): ClientIp,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<impl IntoResponse> {
    app_state
        .services
        .auth
        .register_service(uri.to_string(), ip.to_string(), payload)
        .await
}

pub async fn login_handler(
    State(app_state): State<Arc<AppState>>,
    uri: OriginalUri,
    ClientIp(ip): ClientIp,
    Json(payload): Json<LoginRequest>,
) -> AppResult<impl IntoResponse> {
    app_state
        .services
        .auth
        .login_service(uri.to_string(), ip.to_string(), payload)
        .await
}

pub async fn logout_handler() {
    todo!()
}

pub async fn forget_password_handler() {
    todo!()
}

pub async fn refresh_token_handler() {
    todo!()
}
