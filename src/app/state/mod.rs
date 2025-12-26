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
    pub surreal: Arc<SurrealClient>,
    pub redis: Arc<RedisClient>,
    pub rustfs: Arc<RustFSClient>,
    pub resend: Arc<Resend>,
    pub services: Services,
}

impl AppState {
    pub fn new(
        config: Arc<AppConfig>,
        surreal: SurrealClient,
        redis: RedisClient,
        rustfs: RustFSClient,
        resend: Resend,
    ) -> Self {
        let surreal = Arc::new(surreal);
        let redis = Arc::new(redis);
        let rustfs = Arc::new(rustfs);
        let resend = Arc::new(resend);
        let services = Services::new(
            config.clone(),
            surreal.clone(),
            redis.clone(),
            rustfs.clone(),
        );
        AppState {
            config,
            surreal,
            redis,
            rustfs,
            resend,
            services,
        }
    }
}
