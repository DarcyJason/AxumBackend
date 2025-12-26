use axum::http::StatusCode;
use thiserror::Error;

use crate::app::errors::ErrorKind;

#[derive(Debug, Error)]
pub enum UserErrorKind {
    #[error("Failed to create user")]
    UserCreatedFailed,
    #[error("User already exists")]
    UserAlreadyExists,
}

impl ErrorKind for UserErrorKind {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::UserCreatedFailed => StatusCode::INTERNAL_SERVER_ERROR,
            Self::UserAlreadyExists => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn message(&self) -> String {
        self.to_string()
    }
}
