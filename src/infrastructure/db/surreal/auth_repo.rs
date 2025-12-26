use async_trait::async_trait;

use crate::{
    app::{errors::external_error::ExternalError, result::AppResult},
    domain::user::{Role, User},
    infrastructure::db::surreal::SurrealClient,
};

#[async_trait]
pub trait SurrealAuthRepository {
    async fn find_user_by_email(&self, email: &str) -> AppResult<Option<User>>;
    async fn create_user(
        &self,
        name: &str,
        email: &str,
        password_hashed: &str,
        role: Role,
    ) -> AppResult<Option<User>>;
}

#[async_trait]
impl SurrealAuthRepository for SurrealClient {
    async fn find_user_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let sql = r#"
            SELECT * FROM user WHERE email = $email;
        "#;
        let mut result = self
            .client
            .query(sql)
            .bind(("email", email.to_string()))
            .await
            .map_err(ExternalError::from)?;
        let user: Option<User> = result.take(0).map_err(ExternalError::from)?;
        Ok(user)
    }
    async fn create_user(
        &self,
        name: &str,
        email: &str,
        password_hashed: &str,
        role: Role,
    ) -> AppResult<Option<User>> {
        let sql = r#"
            CREATE user CONTENT {
                name: $name,
                email: $email,
                password_hashed: $password_hashed,
                role: $role,
                is_verified: false,
                is_banned: false,
            };
        "#;
        let mut result = self
            .client
            .query(sql)
            .bind(("name", name.to_string()))
            .bind(("email", email.to_string()))
            .bind(("password_hashed", password_hashed.to_string()))
            .bind(("role", role))
            .await
            .map_err(ExternalError::from)?;
        let user: Option<User> = result.take(0).map_err(ExternalError::from)?;
        Ok(user)
    }
}
