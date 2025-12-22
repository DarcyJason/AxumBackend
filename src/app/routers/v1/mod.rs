use std::sync::Arc;

use axum::Router;

use crate::api::auth::routers::auth_routers;
use crate::middleware::cors::cors;
use crate::{api::health::routers::health_routers, app::state::AppState};

pub fn v1_routers(app_state: Arc<AppState>) -> Router {
    let v1_routers = Router::new()
        .merge(health_routers(app_state.clone()))
        .merge(auth_routers(app_state.clone()))
        .layer(cors(
            app_state
                .config
                .frontend_server_config
                .frontend_address
                .clone(),
        ));
    Router::new().nest("/api/v1", v1_routers)
}
