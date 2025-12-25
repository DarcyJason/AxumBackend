use std::sync::Arc;

use axum::{extract::OriginalUri, http::StatusCode, response::IntoResponse};
use tracing::info;

use crate::{
    app::{errors::other_error::OtherErrorKind, response::AppResponse, result::AppResult},
    infrastructure::{
        config::AppConfig,
        db::{
            redis::{RedisClient, health_repo::RedisHealthRepository},
            rustfs::{RustFSClient, health_repo::RustFSHealthRepository},
            surreal::{SurrealClient, health_repo::SurrealHealthRepository},
        },
    },
};

pub struct HealthService {
    pub config: Arc<AppConfig>,
    pub surreal_client: Arc<SurrealClient>,
    pub redis_client: Arc<RedisClient>,
    pub rustfs_client: Arc<RustFSClient>,
}

impl HealthService {
    pub fn new(
        config: Arc<AppConfig>,
        surreal_client: Arc<SurrealClient>,
        redis_client: Arc<RedisClient>,
        rustfs_client: Arc<RustFSClient>,
    ) -> Self {
        HealthService {
            config,
            surreal_client,
            redis_client,
            rustfs_client,
        }
    }
    pub async fn check_health_service(
        &self,
        uri: OriginalUri,
    ) -> AppResult<impl IntoResponse + use<>> {
        info!("Start handling {}", uri.to_string());
        let response = AppResponse::<()>::ok(StatusCode::OK.as_u16(), "Healthy", None);
        info!("Finish handling {}", uri.to_string());
        Ok(response)
    }
    pub async fn check_db_ready_service(
        &self,
        uri: OriginalUri,
    ) -> AppResult<impl IntoResponse + use<>> {
        info!("Start handling {}", uri.to_string());
        let (surreal_ok, redis_ok, rustfs_ok) = tokio::join!(
            async { self.surreal_client.health_check().await.is_ok() },
            async { self.redis_client.health_check().await.is_ok() },
            async { self.rustfs_client.health_check().await.is_ok() }
        );
        let mut failed = Vec::new();
        if !surreal_ok {
            failed.push("SurrealDB");
        }
        if !redis_ok {
            failed.push("Redis");
        }
        if !rustfs_ok {
            failed.push("RustFS");
        }
        if !failed.is_empty() {
            return Err(
                OtherErrorKind::Error(format!("{} server error", failed.join(" & "))).into(),
            );
        }
        let response = AppResponse::<()>::ok(StatusCode::OK.as_u16(), "Ready", None);
        info!("Finish handling {}", uri.to_string());
        Ok(response)
    }
}
