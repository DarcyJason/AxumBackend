use std::net::{SocketAddr};

use crate::{
    app::{
        errors::external_error::ExternalError, result::AppResult,
    },
    infrastructure::{
        bootstrap::{shutdown::run_shutdown_tasks, startup::run_startup_tasks},
    },
};

pub mod errors;
pub mod logo;
pub mod response;
pub mod result;
pub mod routers;
pub mod state;

pub async fn run() -> AppResult<()> {
    let (_logger_guard, app_routers, listener) = run_startup_tasks().await.unwrap();
    axum::serve(
        listener,
        app_routers.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(run_shutdown_tasks())
    .await
    .map_err(ExternalError::from)?;
    Ok(())
}
