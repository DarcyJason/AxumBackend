use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AppResponse<T> {
    pub code: u16,
    pub message: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> AppResponse<T>
where
    T: Serialize,
{
    pub fn ok(code: u16, message: &str, status: &str, data: Option<T>) -> Self {
        AppResponse {
            code,
            message: message.to_string(),
            status: status.to_string(),
            data,
        }
    }
    pub fn err(code: u16, message: &str, status: &str) -> Self {
        AppResponse {
            code,
            message: message.to_string(),
            status: status.to_string(),
            data: None,
        }
    }
}
