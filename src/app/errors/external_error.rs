use axum::http::StatusCode;
use thiserror::Error;

use crate::app::errors::ErrorKind;

#[derive(Debug, Error)]
pub enum ExternalError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    ParseColor(#[from] colorgrad::GradientBuilderError),
    #[error(transparent)]
    Redis(#[from] redis::RedisError),
    #[error(transparent)]
    SurrealDB(#[from] surrealdb::Error),
    #[error(transparent)]
    Figment(#[from] figment2::Error),
    #[error(transparent)]
    JsonWebToken(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    Resend(#[from] resend_rs::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

impl ErrorKind for ExternalError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
    fn message(&self) -> String {
        self.to_string()
    }
}
