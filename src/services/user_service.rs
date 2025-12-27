use std::sync::Arc;

use crate::{
    app::result::AppResult,
    infrastructure::{
        config::AppConfig,
        db::{
            redis::RedisClient,
            surreal::{SurrealClient, user_repo::SurrealUserRepository},
        },
    },
};

pub struct UserService {
    pub config: Arc<AppConfig>,
    pub surreal: Arc<SurrealClient>,
    pub redis: Arc<RedisClient>,
}

impl UserService {
    pub fn new(
        config: Arc<AppConfig>,
        surreal: Arc<SurrealClient>,
        redis: Arc<RedisClient>,
    ) -> Self {
        UserService {
            config,
            surreal,
            redis,
        }
    }
    pub async fn unverified_user_cleanup(&self) -> AppResult<()> {
        let users = self.surreal.unverified_user_cleanup().await?;
        match users {
            Some(users) => {
                for user in users {
                    tracing::info!("{} is deleted successfully", user.email);
                }
            }
            None => {
                tracing::info!("No user need to delete");
            }
        }
        Ok(())
    }
}
