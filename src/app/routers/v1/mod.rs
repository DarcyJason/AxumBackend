use axum::Router;

use crate::app::state::AppState;

pub fn app_routers(app_state: AppState) -> Router {
    let app_routers = Router::new();
    Router::new().nest("/api/v1", app_routers)
}
