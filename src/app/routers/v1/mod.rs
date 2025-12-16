use std::sync::Arc;

use axum::Router;

use crate::{api::health::routers::health_router, app::state::AppState};

pub fn v1_routers(app_state: Arc<AppState>) -> Router {
    let v1_routers = Router::new().merge(health_router(app_state.clone()));
    Router::new().nest("/api/v1", v1_routers)
}
