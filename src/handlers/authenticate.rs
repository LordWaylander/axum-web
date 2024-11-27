use axum::{
    extract::Json,
    http::StatusCode,
};
use bcrypt::verify;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Validation, TokenData};
use serde::{Deserialize, Serialize};

use crate::models::users::SignInData;
use crate::repository::users as RepositoryUsers;
use crate::format_responses::ErrorResponse;
use crate::models::users::UserLogin;
use std::env;

#[derive(Deserialize, Serialize)]
#[derive(Debug)]
pub struct Token {
    pub exp: usize,
    pub iat: usize,
    pub id: i32,
    pub email: String, 
    pub roles: String
}

pub async fn login(
    Json(payload): Json<SignInData>
) -> Result<Json<String>, ErrorResponse> {
    let user: Result<UserLogin, ErrorResponse>  = retrieve_user_by_email(payload.email);

    match user {
        Ok(user) => {
            let is_password_good = verify_password(payload.password.as_str(), user.password.as_str());

            match is_password_good {
                Ok(_) => {
                    match encode_jwt(user) {
                        Ok(t) => {
                            Ok(Json(t.to_string()))
                        }
                        Err(e) => {
                            Err(e)
                        }
                    }
                }
                Err(e) => {
                    Err(e)
                }
            }
        }
        Err(e) => {
            Err(e)
        }
    }
}

fn retrieve_user_by_email(email: String) -> Result<UserLogin, ErrorResponse> {
    let user: Result<UserLogin, ErrorResponse>  = match RepositoryUsers::get_user_by_email(email) {
        Ok(user) => {
            match user {
                Some(u) => {
                    Ok(u)
                }
                None => {
                    let err = ErrorResponse::error(StatusCode::UNAUTHORIZED.as_u16(), "Incorrect user or password".to_string());
                    Err(err)
                }
            }
        }
        Err(e) => {
            let err = ErrorResponse::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e.to_string());
            Err(err)
        }
    };
    user
}

fn verify_password(password_payload: &str, password_hashed: &str) -> Result<bool, ErrorResponse> {
    let is_password_good = match verify(password_payload, password_hashed) {
        Ok(value) => {
            if value {
                Ok(true)
            } else {
                let err = ErrorResponse::error(StatusCode::UNAUTHORIZED.as_u16(), "Incorrect user or password".to_string());
                Err(err)
            }
        }
        Err(e) => {
            let err = ErrorResponse::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e.to_string());
            Err(err)
        }
    };
    is_password_good
}

fn encode_jwt(user: UserLogin) -> Result<String, ErrorResponse> {
    let secret: String = env::var("SECRET_KEY").unwrap().to_string();
    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::minutes(30);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;
    let claim = Token { iat, exp, id: user.id, email: user.email, roles: user.roles };

    let token = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    );

    match token {
        Ok(t) => {
            Ok("Bearer ".to_string() + &t)
        }
        Err(e) => {
            let err = ErrorResponse::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e.to_string());
            Err(err)
        }
    }
}

pub fn decode_jwt(token: &str) -> Result<TokenData<Token>, ErrorResponse> {
    let secret: String = env::var("SECRET_KEY").unwrap().to_string();

    let token = decode::<Token>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default()
    );

    match token {
        Ok(t) => {
            Ok(t)
        }
        Err(e) => {
            let err = ErrorResponse::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e.to_string());
            Err(err)
        }
    }
}