use axum:: {
    extract::{Request, Json},
    http,
    http::StatusCode,
};
use jsonwebtoken::TokenData;
use crate::handlers::authenticate::{decode_jwt, Token};
use crate::errors::{error, ErrorResponse};

pub mod is_admin;
pub mod is_authenticate;
pub mod is_proprietary_post;

pub fn get_token_from_header(req: &Request) -> Result<TokenData<Token>, Json<ErrorResponse>> {
    let token_header = req.headers().get(http::header::AUTHORIZATION);

    match token_header {
        Some(token) => {

            let mut header = token.to_str().unwrap().split_whitespace();
            let (_bearer, token) = (header.next().unwrap(), header.next().unwrap());

            let decoded_token = decode_jwt(token);

            decoded_token
        }
        None => {
            let err = error(StatusCode::UNAUTHORIZED.to_string(), "No token found".to_string());
            Err(err)
        }
    }
}