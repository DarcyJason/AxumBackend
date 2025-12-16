use axum::http::StatusCode;
use thiserror::Error;

use crate::app::errors::ErrorKind;

#[derive(Debug, Error)]
pub enum BootstrapErrorKind {
    #[error("bootstrap failed: {0}")]
    BootstrapFailed(String),
}

impl ErrorKind for BootstrapErrorKind {
    fn status_code(&self) -> StatusCode {
        match self {
            BootstrapErrorKind::BootstrapFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn message(&self) -> String {
        self.to_string()
    }
}
