use crate::{
    app::{result::AppResult, state::AppState},
    domain::request::auth::{LoginRequest, RegisterRequest},
};
use axum::{Json, extract::State, response::IntoResponse};
use std::sync::Arc;

pub mod routers;
pub mod validation;

pub async fn register_handler(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<impl IntoResponse> {
    app_state.services.auth.register_service(payload).await
}

pub async fn login_handler(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<impl IntoResponse> {
    app_state.services.auth.login_service(payload).await
}

pub async fn logout_handler() {
    todo!()
}

pub async fn forget_password_handler() {
    todo!()
}

pub async fn refresh_handler() {
    todo!()
}
