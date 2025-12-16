use redis::aio::MultiplexedConnection;

use crate::{
    app::{errors::external_error::ExternalError, result::AppResult},
    infrastructure::config::redis_server_config::RedisServerConfig,
};

pub mod health_repo;

#[derive(Debug, Clone)]
pub struct RedisClient {
    pub conn: MultiplexedConnection,
}

impl RedisClient {
    pub async fn new(redis_server_config: RedisServerConfig) -> AppResult<Self> {
        let client =
            redis::Client::open(redis_server_config.redis_address).map_err(ExternalError::from)?;
        let conn = client
            .get_multiplexed_async_connection()
            .await
            .map_err(ExternalError::from)?;
        Ok(RedisClient { conn })
    }
}
