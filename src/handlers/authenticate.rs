// https://blog.logrocket.com/using-rust-axum-build-jwt-authentication-api/
// https://www.shuttle.dev/blog/2024/02/21/using-jwt-auth-rust

use axum::{
    extract::Json,
    http::StatusCode,
};
use bcrypt::verify;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::models::users::SignInData;
use crate::repository::users as RepositoryUsers;
use crate::errors::{error, ErrorResponse};
use crate::models::users::UserLogin;
use std::env;

#[derive(Serialize, Deserialize)]
// Define a structure for holding claims data used in JWT tokens
pub struct Claims {
    pub exp: usize,  // Expiry time of the token
    pub iat: usize,  // Issued at time of the token
    pub email: String,  // Email associated with the token
}

pub async fn login(
    Json(payload): Json<SignInData>
) -> Result<Json<String>, Json<ErrorResponse>> { //-> Result<Json<String>, Json<ErrorResponse>> {
    let user: Result<UserLogin, Json<ErrorResponse>>  = retrieve_user_by_email(payload.email);

    // verify password
    match user {
        Ok(user) => {
            let is_password_good = verify_password(payload.password.as_str(), user.password.as_str());

            match is_password_good {
                Ok(_) => {
                    // generate JWT Token
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

fn retrieve_user_by_email(email: String) -> Result<UserLogin, Json<ErrorResponse>> {
    let user: Result<UserLogin, Json<ErrorResponse>>  = match RepositoryUsers::get_user_by_email(email) {
        Ok(user) => {
            match user {
                Some(u) => {
                    Ok(u)
                }
                None => {
                    let err = error(StatusCode::UNAUTHORIZED.to_string(), "user not found".to_string());
                    Err(Json(err))
                }
            }
        }
        Err(e) => {
            let err = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), e.to_string());
            Err(Json(err))
        }
    };

    user
}

fn verify_password(password_payload: &str, password_hashed: &str) -> Result<bool, Json<ErrorResponse>> {
    let is_password_good = match verify(password_payload, password_hashed) {
        Ok(value) => {
            if value {
                Ok(true)
            } else {
                let err = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), "user not found".to_string());
                Err(Json(err))
            }
        }
        Err(e) => {
            let err = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), e.to_string());
            Err(Json(err))
        }
    };

    is_password_good
}

fn encode_jwt(user: UserLogin) -> Result<String, Json<ErrorResponse>> {

    #[derive(Serialize)]
    struct Claims {
        exp: usize,
        iat: usize,
        id: i32,
        email: String, 
        roles: String
    }

    let secret: String = env::var("SECRET_KEY").unwrap().to_string();
    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::hours(1);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;
    let claim = Claims { iat, exp, id: user.id, email: user.email, roles: user.roles };

    let token = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    );

    match token {
        Ok(t) => {
            Ok(t)
        }
        Err(e) => {
            let err = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), e.to_string());
            Err(Json(err))
        }
    }

}