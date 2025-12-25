use std::sync::Arc;

use resend_rs::Resend;

use crate::{
    infrastructure::{
        config::AppConfig,
        db::{redis::RedisClient, rustfs::RustFSClient, surreal::SurrealClient},
    },
    services::Services,
};

pub struct AppState {
    pub config: Arc<AppConfig>,
    pub surreal_client: Arc<SurrealClient>,
    pub redis_client: Arc<RedisClient>,
    pub rustfs_client: Arc<RustFSClient>,
    pub resend: Arc<Resend>,
    pub services: Services,
}

impl AppState {
    pub fn new(
        config: Arc<AppConfig>,
        surreal_client: SurrealClient,
        redis_client: RedisClient,
        rustfs_client: RustFSClient,
        resend: Resend,
    ) -> Self {
        let surreal_client = Arc::new(surreal_client);
        let redis_client = Arc::new(redis_client);
        let rustfs_client = Arc::new(rustfs_client);
        let resend = Arc::new(resend);
        let services = Services::new(
            config.clone(),
            surreal_client.clone(),
            redis_client.clone(),
            rustfs_client.clone(),
        );
        AppState {
            config,
            surreal_client,
            redis_client,
            rustfs_client,
            resend,
            services,
        }
    }
}
