use figment::{Figment, providers::Env};
use serde::{Deserialize, Serialize};

use crate::{
    app::{errors::external_error::ExternalError, result::AppResult},
    infrastructure::config::{
        backend_server_config::BackendServerConfig, frontend_server_config::FrontendServerConfig,
        jwt_config::JwtConfig, mail_server_config::MailServerConfig,
        redis_server_config::RedisServerConfig, surreal_server_config::SurrealServerConfig,
    },
};

pub mod backend_server_config;
pub mod frontend_server_config;
pub mod jwt_config;
pub mod mail_server_config;
pub mod redis_server_config;
pub mod surreal_server_config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(flatten)]
    pub backend_server_config: BackendServerConfig,
    #[serde(flatten)]
    pub frontend_server_config: FrontendServerConfig,
    #[serde(flatten)]
    pub mail_server_config: MailServerConfig,
    #[serde(flatten)]
    pub surreal_server_config: SurrealServerConfig,
    #[serde(flatten)]
    pub redis_server_config: RedisServerConfig,
    #[serde(flatten)]
    pub jwt_config: JwtConfig,
}

impl AppConfig {
    pub fn init() -> AppResult<Self> {
        Ok(Figment::new()
            .merge(Env::prefixed(""))
            .extract()
            .map_err(ExternalError::from)?)
    }
}
