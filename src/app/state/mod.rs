use std::sync::Arc;

use crate::{
    infrastructure::{
        config::AppConfig,
        db::{redis::RedisClient, surreal::SurrealClient},
    },
    services::Services,
};

pub struct AppState {
    pub config: Arc<AppConfig>,
    pub surreal_client: Arc<SurrealClient>,
    pub redis_client: Arc<RedisClient>,
    pub services: Services,
}

impl AppState {
    pub fn new(
        config: AppConfig,
        surreal_client: SurrealClient,
        redis_client: RedisClient,
    ) -> Self {
        let config = Arc::new(config);
        let surreal_client = Arc::new(surreal_client);
        let redis_client = Arc::new(redis_client);
        let services = Services::new(config.clone(), surreal_client.clone(), redis_client.clone());
        AppState {
            config,
            surreal_client,
            redis_client,
            services,
        }
    }
}
