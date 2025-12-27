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
        db::{redis::RedisClient, rustfs::RustFSClient, surreal::SurrealClient},
        tasks::cleanup::unverified_user_cleanup,
    },
    middleware::logger::logger,
    utils::{color_logo::color_logo, password::hash_password},
};
use axum::Router;
use resend_rs::Resend;
use tokio::net::TcpListener;
use tracing_appender::non_blocking::WorkerGuard;

pub async fn run_startup_tasks() -> AppResult<(WorkerGuard, Router, TcpListener)> {
    let logger_guard = logger();
    dotenvy::dotenv().ok();
    color_logo(APPLOGO)?;
    let config = Arc::new(AppConfig::init()?);
    let port = config.backend_server_config.backend_port;
    let surreal = SurrealClient::new(config.surreal_server_config.clone()).await?;
    let redis = RedisClient::new(config.redis_server_config.clone()).await?;
    let rustfs = RustFSClient::new(config.rustfs_server_config.clone()).await;
    let resend = Resend::new(&config.mail_server_config.resend_api_key.clone());
    let app_state = Arc::new(AppState::new(
        config.clone(),
        surreal,
        redis,
        rustfs,
        resend,
    ));
    check_if_exists_system_owner(app_state.clone()).await?;
    unverified_user_cleanup::start(app_state.clone()).await;
    let app_routers = app_routers(app_state);
    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
    let listener = TcpListener::bind(&address)
        .await
        .map_err(ExternalError::from)?;
    tracing::info!("Axum server is running at http://localhost:{}", port);
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
            LET $system_owner = SELECT * FROM user WHERE role = $role;
            LET $result = IF array::len($system_owner) > 0 THEN
                $system_owner
            ELSE (CREATE user SET
                    name = $name,
                    email = $email,
                    password_hashed = $password_hashed,
                    role = $role,
                    is_verified = true,
                    is_banned = false);
            END;
        COMMIT TRANSACTION;
        RETURN $result;
        "#;
    let mut res = app_state
        .surreal
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
        tracing::info!("SystemOwner created successfully");
    } else {
        tracing::info!("SystemOwner already exists");
    }
    Ok(())
}
