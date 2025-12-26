use std::sync::Arc;

use axum::response::IntoResponse;

use crate::{
    api::auth::validation::{validate_login_payload, validate_register_payload},
    app::{errors::user_error::UserErrorKind, response::AppResponse, result::AppResult},
    domain::{
        request::auth::{LoginRequest, RegisterRequest},
        user::Role,
    },
    infrastructure::{
        config::AppConfig,
        db::{
            redis::RedisClient,
            surreal::{SurrealClient, auth_repo::SurrealAuthRepository},
        },
    },
    utils::password::hash_password,
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
        if let Some(user) = self.surreal.find_user_by_email(&payload.email).await? {
            tracing::error!("User {} already exists", user.name);
            tracing::info!("Finish handling {} for {}", uri, addr);
            return Err(UserErrorKind::UserAlreadyExists.into());
        }
        let password_hashed = hash_password(payload.password)?;
        if let Err(e) = self
            .surreal
            .create_user(&payload.name, &payload.email, &password_hashed, Role::User)
            .await
        {
            tracing::error!("Create user by {} failed: {}", payload.email, e);
            tracing::info!("Finish handling {} for {}", uri, addr);
            return Err(UserErrorKind::UserCreatedFailed.into());
        }
        let response = AppResponse::<()>::ok(200, "Create user successfully", None);
        tracing::info!("Finish handling {} for {}", uri, addr);
        Ok(response)
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
