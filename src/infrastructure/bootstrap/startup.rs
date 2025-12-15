use std::net::{Ipv4Addr, SocketAddr};

use crate::{app::{errors::external_error::ExternalError, logo::APPLOGO, result::AppResult, routers::v1::app_routers, state::AppState}, infrastructure::{config::AppConfig, db::{redis::RedisClient, surreal::SurrealClient}}, middleware::logger::logger, utils::color_logo::color_logo};
use axum::Router;
use tokio::net::TcpListener;
use tracing::info;
use tracing_appender::non_blocking::WorkerGuard;


pub async fn run_startup_tasks() -> AppResult<(WorkerGuard, Router, TcpListener)>{
    let logger_guard = logger();
    dotenvy::dotenv().ok();
    color_logo(APPLOGO).unwrap();
    let config = AppConfig::init()?;
    let port = config.backend_server_config.backend_port;
    let surreal_client = SurrealClient::new(config.clone().surreal_server_config).await?;
    let redis_client = RedisClient::new(config.clone().redis_server_config).await?;
    let app_state = AppState::new(config.clone(), surreal_client, redis_client);
    let app_routers = app_routers(app_state);
    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
    let listener = TcpListener::bind(&address)
        .await
        .map_err(ExternalError::from)?;
    info!("✅ Axum server is running at http://localhost:{}", port);
    Ok((logger_guard, app_routers, listener))
}