use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse};

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
    pub surreal: Arc<SurrealClient>,
    pub redis: Arc<RedisClient>,
    pub rustfs: Arc<RustFSClient>,
}

impl HealthService {
    pub fn new(
        config: Arc<AppConfig>,
        surreal: Arc<SurrealClient>,
        redis: Arc<RedisClient>,
        rustfs: Arc<RustFSClient>,
    ) -> Self {
        HealthService {
            config,
            surreal,
            redis,
            rustfs,
        }
    }
    pub async fn check_health_service(
        &self,
        uri: String,
        addr: String,
    ) -> AppResult<impl IntoResponse + use<>> {
        tracing::info!("Start handling {} for {}", uri, addr);
        let response = AppResponse::<()>::ok(StatusCode::OK.as_u16(), "Healthy", None);
        tracing::info!("Finish handling {} for {}", uri, addr);
        Ok(response)
    }
    pub async fn check_db_ready_service(
        &self,
        uri: String,
        addr: String,
    ) -> AppResult<impl IntoResponse + use<>> {
        tracing::info!("Start handling {} for {}", uri, addr);
        let (surreal_ok, redis_ok, rustfs_ok) = tokio::join!(
            async { self.surreal.health_check().await.is_ok() },
            async { self.redis.health_check().await.is_ok() },
            async { self.rustfs.health_check().await.is_ok() }
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
        tracing::info!("Finish handling {} for {}", uri, addr);
        Ok(response)
    }
}
