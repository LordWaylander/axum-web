use axum:: {
    extract::{Request, Json},
    http,
    http::StatusCode,
};
use crate::handlers::authenticate::{decode_jwt, Token};
use crate::errors::{error, ErrorResponse};

pub mod redirect;
pub mod is_admin;
pub mod expiration_token;

pub fn get_token_from_header(req: &Request) -> Result<Token, Json<ErrorResponse>> {
    let token_header = req.headers().get(http::header::AUTHORIZATION);

    match token_header {
        Some(token) => {

            let mut header = token.to_str().unwrap().split_whitespace();
            let (_bearer, token) = (header.next(), header.next());

            let decoded_token = decode_jwt(token.unwrap());

            match decoded_token {
                Ok(decoded_token) => {
                    Ok(decoded_token.claims)
                }
                Err(e) => {
                    let err = error(StatusCode::UNAUTHORIZED.to_string(), "Error in the token".to_string());
                    Err(Json(err))
                }
            }
        }
        None => {
            let err = error(StatusCode::UNAUTHORIZED.to_string(), "No token found".to_string());
            Err(Json(err))
        }
    }
}