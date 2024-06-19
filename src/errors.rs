use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
    code_error: String,
}

pub fn error(code: String, message: String) -> ErrorResponse {

    let error_response = ErrorResponse {
        error: message,
        code_error: code,
    };

    error_response
}