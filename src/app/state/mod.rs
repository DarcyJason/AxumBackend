use std::sync::Arc;

use crate::infrastructure::{
    config::AppConfig, db::redis::RedisClient, db::surreal::SurrealClient,
};

pub struct AppState {
    pub config: Arc<AppConfig>,
    pub surreal_client: Arc<SurrealClient>,
    pub redis_client: Arc<RedisClient>,
    // pub service: Services,
}

impl AppState {
    pub fn new(
        config: AppConfig,
        surreal_client: SurrealClient,
        redis_client: RedisClient,
    ) -> Self {
        AppState {
            config: Arc::new(config),
            surreal_client: Arc::new(surreal_client),
            redis_client: Arc::new(redis_client),
        }
    }
}
