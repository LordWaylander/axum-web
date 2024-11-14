use serde::Serialize;
use axum::Json;

#[derive(Serialize)]
#[derive(Debug)]
pub struct ErrorResponse {
    error: String,
    code_error: String,
}

pub fn error(code: String, message: String) -> Json<ErrorResponse> {

    let error_response = ErrorResponse {
        error: message,
        code_error: code,
    };

    Json(error_response)
}