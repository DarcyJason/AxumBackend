use crate::app::{routers::v1::v1_routers, state::AppState};
use axum::Router;
use std::sync::Arc;

pub mod v1;

pub fn app_routers(app_state: Arc<AppState>) -> Router {
    Router::new().merge(v1_routers(app_state.clone()))
}
