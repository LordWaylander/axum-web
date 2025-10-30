use serde::Serialize;

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

#[derive(Serialize)]
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