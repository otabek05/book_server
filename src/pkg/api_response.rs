use axum::http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: &str) -> Self {
        ApiResponse {
            status: 200,
            message: message.to_string(),
            data: Some(data),
        }
    }

    pub fn error(status: StatusCode, message: String) -> Self {
        ApiResponse {
            status: status.as_u16(),
            message: message,
            data: None,
        }
    }
}
