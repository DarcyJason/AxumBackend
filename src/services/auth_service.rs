use std::sync::Arc;

use axum::response::IntoResponse;

use crate::{
    api::auth::validation::{validate_login_payload, validate_register_payload},
    app::result::AppResult,
    domain::request::auth::{LoginRequest, RegisterRequest},
    infrastructure::{
        config::AppConfig,
        db::{redis::RedisClient, surreal::SurrealClient},
    },
};

pub struct AuthService {
    pub config: Arc<AppConfig>,
    pub surreal_client: Arc<SurrealClient>,
    pub redis_client: Arc<RedisClient>,
}

impl AuthService {
    pub fn new(
        config: Arc<AppConfig>,
        surreal_client: Arc<SurrealClient>,
        redis_client: Arc<RedisClient>,
    ) -> Self {
        AuthService {
            config,
            surreal_client,
            redis_client,
        }
    }
    pub async fn register_service(
        &self,
        payload: RegisterRequest,
    ) -> AppResult<impl IntoResponse + use<>> {
        validate_register_payload(&payload)?;
        Ok("ok")
    }
    pub async fn login_service(
        &self,
        payload: LoginRequest,
    ) -> AppResult<impl IntoResponse + use<>> {
        validate_login_payload(&payload)?;
        Ok("ok")
    }
}
