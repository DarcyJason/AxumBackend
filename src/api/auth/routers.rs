use std::sync::Arc;
use axum::Router;
use axum::routing::post;
use crate::api::auth::{login_handler, register_handler};
use crate::app::state::AppState;

pub fn auth_routers(app_state: Arc<AppState>) -> Router {
    let auth_routers = Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .with_state(app_state);
    Router::new().nest("/auth", auth_routers)
}