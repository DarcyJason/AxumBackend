use std::net::{Ipv4Addr, SocketAddr};

use tokio::net::TcpListener;
use tracing::info;

use crate::{
    app::{
        errors::external_error::ExternalError, logo::APPLOGO, result::AppResult,
        routers::v1::app_routers, state::AppState,
    },
    infrastructure::{
        bootstrap::shutdown::run_shutdown_tasks,
        config::AppConfig,
        db::{redis::RedisClient, surreal::SurrealClient},
    },
    middleware::logger::logger,
    utils::color_logo::gradient_text,
};

pub mod errors;
pub mod logo;
pub mod response;
pub mod result;
pub mod routers;
pub mod state;

pub async fn run() -> AppResult<()> {
    let _logger_guard = logger();
    dotenvy::dotenv().ok();
    let _ = gradient_text(APPLOGO);
    let config = AppConfig::init()?;
    let port = config.backend_server_config.backend_port;
    info!("✅ Axum server is running at http://localhost:{}", port);
    let surreal_client = SurrealClient::new(config.clone().surreal_server_config).await?;
    let redis_client = RedisClient::new(config.clone().redis_server_config).await?;
    let app_state = AppState::new(config.clone(), surreal_client, redis_client);
    let app_routers = app_routers(app_state);
    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
    let listener = TcpListener::bind(&address)
        .await
        .map_err(ExternalError::from)?;
    axum::serve(
        listener,
        app_routers.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(run_shutdown_tasks())
    .await
    .map_err(ExternalError::from)?;
    Ok(())
}
