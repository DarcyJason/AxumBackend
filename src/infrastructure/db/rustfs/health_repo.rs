use async_trait::async_trait;

use crate::{app::result::AppResult, infrastructure::db::rustfs::RustFSClient};

#[async_trait]
pub trait RustFSHealthRepository {
    async fn health_check(&self) -> AppResult<bool>;
}

#[async_trait]
impl RustFSHealthRepository for RustFSClient {
    async fn health_check(&self) -> AppResult<bool> {
        match self.rustfs.list_buckets().send().await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
