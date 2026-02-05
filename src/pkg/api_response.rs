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

    pub fn error(status: StatusCode, err: sqlx::Error) -> Self {
        let err_msg = err.to_string();
        eprintln!("db error: {}", err_msg);
        ApiResponse {
            status: status.as_u16(),
            message: err_msg,
            data: None,
        }
    }


    pub fn generic_error(status: StatusCode, message:String) -> Self {
        eprintln!("generic error: {}", message);
        ApiResponse {
            status: status.as_u16(),
            message: message,
            data: None,
        }
    }
}
