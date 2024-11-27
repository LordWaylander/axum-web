use serde::Serialize;
use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
};
use crate::models::{
    users::User,
    posts::Post
};

#[derive(Serialize)]
pub struct UserResponse {
    pub user: User,
    pub post: Vec<Post>
}

#[derive(Serialize)]
pub struct PostResponse {
    pub post: Post,
    pub user: User
}

pub struct ErrorResponse {
    pub error: String,
    pub code_error: u16,
}

impl ErrorResponse {
    pub fn error(code: u16, message: String) -> ErrorResponse {

        let error_response = ErrorResponse {
            error: message,
            code_error: code,
        };

        error_response
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {

        (StatusCode::from_u16(self.code_error).unwrap(), self.error).into_response()
    }
    
}

/*impl IntoResponse for ErrorResponse {
    
}*/

/*impl IntoResponse for FormatResponseUser {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}*/