use std::{
    error::Error,
    fmt::{Debug, Display},
};

use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::app::response::AppResponse;

pub mod bootstrap_error;
pub mod external_error;
pub mod other_error;
pub mod user_error;
pub mod validation_error;

pub trait ErrorKind: Error + Send + Sync {
    fn status_code(&self) -> StatusCode;
    fn message(&self) -> String;
}

pub struct AppError {
    kind: Box<dyn ErrorKind>,
}

impl<E> From<E> for AppError
where
    E: ErrorKind + 'static,
{
    fn from(err: E) -> Self {
        AppError {
            kind: Box::new(err),
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.kind.status_code();
        let message = self.kind.message();
        let body = AppResponse::<()>::err(status_code.as_u16(), &message, status_code.as_str());
        (status_code, Json(body)).into_response()
    }
}
