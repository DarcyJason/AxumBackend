use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AppResponse<T> {
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> AppResponse<T>
where
    T: Serialize,
{
    pub fn ok(code: u16, message: &str, data: Option<T>) -> Self {
        AppResponse {
            code,
            message: message.to_string(),
            data,
        }
    }
    pub fn err(code: u16, message: &str) -> Self {
        AppResponse {
            code,
            message: message.to_string(),
            data: None,
        }
    }
}

impl<T> IntoResponse for AppResponse<T>
where
    T: Serialize + Send,
{
    fn into_response(self) -> axum::response::Response {
        let status_code =
            StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status_code, Json(self)).into_response()
    }
}
