use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use crate::{
    app::{
        errors::external_error::ExternalError, logo::APPLOGO, result::AppResult,
        routers::app_routers, state::AppState,
    },
    domain::user::{Role, User},
    infrastructure::{
        config::AppConfig,
        db::{redis::RedisClient, surreal::SurrealClient},
    },
    middleware::logger::logger,
    utils::{color_logo::color_logo, password::hash_password},
};
use axum::Router;
use tokio::net::TcpListener;
use tracing::info;
use tracing_appender::non_blocking::WorkerGuard;

pub async fn run_startup_tasks() -> AppResult<(WorkerGuard, Router, TcpListener)> {
    let logger_guard = logger();
    dotenvy::dotenv().ok();
    color_logo(APPLOGO)?;
    let config = Arc::new(AppConfig::init()?);
    let port = config.backend_server_config.backend_port;
    let surreal_client = SurrealClient::new(config.surreal_server_config.clone()).await?;
    let redis_client = RedisClient::new(config.redis_server_config.clone()).await?;
    let app_state = Arc::new(AppState::new(config.clone(), surreal_client, redis_client));
    check_if_exists_system_owner(app_state.clone()).await?;
    let app_routers = app_routers(app_state);
    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
    let listener = TcpListener::bind(&address)
        .await
        .map_err(ExternalError::from)?;
    info!("✅ Axum server is running at http://localhost:{}", port);
    Ok((logger_guard, app_routers, listener))
}

pub async fn check_if_exists_system_owner(app_state: Arc<AppState>) -> AppResult<()> {
    let name = app_state
        .config
        .system_owner_config
        .system_owner_name
        .clone();
    let email = app_state
        .config
        .system_owner_config
        .system_owner_email
        .clone();
    let password = app_state
        .config
        .system_owner_config
        .system_owner_password
        .clone();
    let password_hashed = hash_password(password)?;
    let sql = r#"
            BEGIN TRANSACTION;

            LET $existing = (
                SELECT count() AS count FROM user WHERE role = $role
            );

            IF $existing[0].count != 0 {
                RETURN NONE;
            };

            LET $created = CREATE user SET
                name = $name,
                email = $email,
                password_hashed = $password_hashed,
                role = $role,
                is_verified = true,
                is_banned = false;

            COMMIT TRANSACTION;

            RETURN $created;
        "#;
    let mut res = app_state
        .surreal_client
        .client
        .query(sql)
        .bind(("name", name))
        .bind(("email", email))
        .bind(("password_hashed", password_hashed))
        .bind(("role", Role::SystemOwner))
        .await
        .map_err(ExternalError::from)?;
    let system_owner: Option<User> = res.take(0).map_err(ExternalError::from)?;
    if system_owner.is_some() {
        info!("✅ SystemOwner created successfully");
    } else {
        info!("✅ SystemOwner already exists");
    }
    Ok(())
}
