use async_trait::async_trait;

use crate::{
    app::{errors::external_error::ExternalError, result::AppResult},
    domain::user::User,
    infrastructure::db::surreal::SurrealClient,
};

#[async_trait]
pub trait SurrealUserRepository {
    async fn unverified_user_cleanup(&self) -> AppResult<Option<Vec<User>>>;
}

#[async_trait]
impl SurrealUserRepository for SurrealClient {
    async fn unverified_user_cleanup(&self) -> AppResult<Option<Vec<User>>> {
        let sql = r#"
            DELETE user where is_verified = false AND time::now() - created_at > 1h;
        "#;
        let mut result = self.client.query(sql).await.map_err(ExternalError::from)?;
        let users: Option<Vec<User>> = result.take(0).map_err(ExternalError::from)?;
        Ok(users)
    }
}
