use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse};

use crate::{
    app::{errors::other_error::OtherErrorKind, response::AppResponse, result::AppResult},
    infrastructure::{
        config::AppConfig,
        db::{
            redis::{RedisClient, health_repo::RedisHealthRepository},
            surreal::{SurrealClient, health_repo::SurrealHealthRepository},
        },
    },
};

pub struct HealthService {
    pub config: Arc<AppConfig>,
    pub surreal_client: Arc<SurrealClient>,
    pub redis_client: Arc<RedisClient>,
}

impl HealthService {
    pub fn new(
        config: Arc<AppConfig>,
        surreal_client: Arc<SurrealClient>,
        redis_client: Arc<RedisClient>,
    ) -> Self {
        HealthService {
            config,
            surreal_client,
            redis_client,
        }
    }
    pub async fn check_health_service(&self) -> AppResult<impl IntoResponse + use<>> {
        Ok(AppResponse::<()>::ok(
            StatusCode::OK.as_u16(),
            "Healthy",
            None,
        ))
    }
    pub async fn check_db_ready_service(&self) -> AppResult<impl IntoResponse + use<>> {
        let checks: (bool, bool) = tokio::join!(
            async { self.surreal_client.health_check().await.is_ok() },
            async { self.redis_client.health_check().await.is_ok() }
        );
        if !checks.0 && !checks.1 {
            Err(
                OtherErrorKind::Error("Redis server error and SurrealDB server error".to_string())
                    .into(),
            )
        } else if !checks.0 && checks.1 {
            Err(OtherErrorKind::Error("Redis server error".to_string()).into())
        } else if checks.0 && !checks.1 {
            Err(OtherErrorKind::Error("SurrealDB server error".to_string()).into())
        } else {
            Ok(AppResponse::<()>::ok(
                StatusCode::OK.as_u16(),
                "Ready",
                None,
            ))
        }
    }
}
