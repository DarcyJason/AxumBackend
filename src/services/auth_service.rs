use std::sync::Arc;

use axum::response::IntoResponse;

use crate::{
    api::auth::validation::{validate_login_payload, validate_register_payload},
    app::{errors::user_error::UserErrorKind, result::AppResult},
    domain::request::auth::{LoginRequest, RegisterRequest},
    infrastructure::{
        config::AppConfig,
        db::{
            redis::RedisClient,
            surreal::{SurrealClient, auth_repo::SurrealAuthRepository},
        },
    },
};

pub struct AuthService {
    pub config: Arc<AppConfig>,
    pub surreal: Arc<SurrealClient>,
    pub redis: Arc<RedisClient>,
}

impl AuthService {
    pub fn new(
        config: Arc<AppConfig>,
        surreal: Arc<SurrealClient>,
        redis: Arc<RedisClient>,
    ) -> Self {
        AuthService {
            config,
            surreal,
            redis,
        }
    }
    pub async fn register_service(
        &self,
        uri: String,
        addr: String,
        payload: RegisterRequest,
    ) -> AppResult<impl IntoResponse + use<>> {
        tracing::info!("Start handling {} for {}", uri, addr);
        validate_register_payload(&payload)?;
        if self
            .surreal
            .find_user_by_email(&payload.email)
            .await?
            .is_some()
        {
            return Err(UserErrorKind::UserAlreadyExists.into());
        }
        Ok("ok")
    }
    pub async fn login_service(
        &self,
        uri: String,
        addr: String,
        payload: LoginRequest,
    ) -> AppResult<impl IntoResponse + use<>> {
        tracing::info!("Start handling {} for {}", uri, addr);
        validate_login_payload(&payload)?;
        Ok("ok")
    }
}
