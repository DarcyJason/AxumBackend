use std::sync::Arc;

use axum::Router;

use crate::{api::health::routers::health_routers, app::state::AppState};
use crate::api::auth::routers::auth_routers;

pub fn v1_routers(app_state: Arc<AppState>) -> Router {
    let v1_routers = Router::new()
        .merge(health_routers(app_state.clone()))
        .merge(auth_routers(app_state.clone()));
    Router::new().nest("/api/v1", v1_routers)
}
