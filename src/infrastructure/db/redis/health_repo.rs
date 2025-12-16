use async_trait::async_trait;
use redis::AsyncTypedCommands;

use crate::{
    app::{errors::external_error::ExternalError, result::AppResult},
    infrastructure::db::redis::RedisClient,
};

#[async_trait]
pub trait RedisHealthRepository {
    async fn health_check(&self) -> AppResult<bool>;
}

#[async_trait]
impl RedisHealthRepository for RedisClient {
    async fn health_check(&self) -> AppResult<bool> {
        let mut conn = self.conn.clone();
        let response = conn.ping().await.map_err(ExternalError::from)?;
        Ok(response == "PONG")
    }
}
