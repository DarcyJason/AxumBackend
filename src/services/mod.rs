use std::sync::Arc;

use crate::{
    infrastructure::{
        config::AppConfig,
        db::{redis::RedisClient, surreal::SurrealClient},
    },
    services::{auth_service::AuthService, health_service::HealthService},
};

pub mod admin_service;
pub mod audit_service;
pub mod auth_service;
pub mod health_service;
pub mod user_service;

pub struct Services {
    pub health: HealthService,
    pub auth: AuthService,
}

impl Services {
    pub fn new(
        config: Arc<AppConfig>,
        surreal_client: Arc<SurrealClient>,
        redis_client: Arc<RedisClient>,
    ) -> Self {
        let health =
            HealthService::new(config.clone(), surreal_client.clone(), redis_client.clone());
        let auth = AuthService::new(config.clone(), surreal_client.clone(), redis_client.clone());
        Services { health, auth }
    }
}
