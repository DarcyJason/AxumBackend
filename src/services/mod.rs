use std::sync::Arc;

use crate::{
    infrastructure::{
        config::AppConfig,
        db::{redis::RedisClient, rustfs::RustFSClient, surreal::SurrealClient},
    },
    services::{
        auth_service::AuthService, health_service::HealthService, user_service::UserService,
    },
};

pub mod admin_service;
pub mod audit_service;
pub mod auth_service;
pub mod health_service;
pub mod user_service;
pub mod vault_service;

pub struct Services {
    pub health: HealthService,
    pub auth: AuthService,
    pub user: UserService,
}

impl Services {
    pub fn new(
        config: Arc<AppConfig>,
        surreal: Arc<SurrealClient>,
        redis: Arc<RedisClient>,
        rustfs: Arc<RustFSClient>,
    ) -> Self {
        let health = HealthService::new(
            config.clone(),
            surreal.clone(),
            redis.clone(),
            rustfs.clone(),
        );
        let auth = AuthService::new(config.clone(), surreal.clone(), redis.clone());
        let user = UserService::new(config.clone(), surreal.clone(), redis.clone());
        Services { health, auth, user }
    }
}
