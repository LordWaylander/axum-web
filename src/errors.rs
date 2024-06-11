use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    error: &'static str,
    code_error: &'static str,
}

pub fn error(code: &'static str, message: &'static str) -> ErrorResponse {

    let error_response = ErrorResponse {
        error: message,
        code_error: code,
    };

    error_response
}