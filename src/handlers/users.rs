use axum::{
    http::StatusCode,
    Json,
    extract::Path,
    response::IntoResponse
};
use crate::repository::users as RepositoryUsers;
use crate::models::users::{NewUser, UpdateUser, User};
use crate::models::posts::Post;
use bcrypt::{hash, DEFAULT_COST};
use crate::format_responses::{UserResponse, ErrorResponse};

pub async fn show_users() -> impl IntoResponse {
    let result: Result<_, _> = RepositoryUsers::get_all_users();

    match result {
        Ok(response) => {
            if response.len() == 0 {
                let err = ErrorResponse::error(StatusCode::OK.as_u16(), "No users found".to_string());
                Err(
                    (StatusCode::from_u16(err.code_error).unwrap(), Json(err))
                )
            } else {
                let mut resp_json : Vec<UserResponse> = Vec::new();

                for res in response {
                    resp_json.push(UserResponse { user: res.0, post: res.1 })
                }
                Ok((StatusCode::OK, Json(resp_json)))
            }
            
        },
        Err(e) => {
            let err = ErrorResponse::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e.to_string() );
            Err(
                (StatusCode::from_u16(err.code_error).unwrap(), Json(err))
            )
        },
    }
}

pub async fn get_one_user(Path(id): Path<i32>) -> impl IntoResponse {
    let result: Result<(User, Vec<Post>), diesel::result::Error> = RepositoryUsers::get_one_user(id);

    match result {
        Ok(response) => {   
            Ok((StatusCode::OK, Json(UserResponse { user: response.0, post: response.1 })))       
        },
        Err(e) => {
            let err = ErrorResponse::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e.to_string());
            Err(
                (StatusCode::from_u16(err.code_error).unwrap(), Json(err))
            )
        },
    }
}

pub async fn create_user(payload: Json<NewUser>) -> impl IntoResponse {

    let user = NewUser {
        username: payload.username.clone(),
        email: payload.email.clone(),
        password: hash_password(payload.password.clone()),
        roles: payload.roles.clone()
    };

    let result = RepositoryUsers::create_user(user);

    match result {
        Ok(user) => Ok((StatusCode::OK, Json(user))), //pas mieux de retourner un status 200, avec message " user bien créé " ?
        Err(e) => {
            let err = ErrorResponse::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(),e.to_string() );
            Err(
                (StatusCode::from_u16(err.code_error).unwrap(), Json(err))
            )
        },
    }
}

pub async fn update_user(Path(id): Path<i32>, payload: Json<UpdateUser>) -> impl IntoResponse {

    let user: UpdateUser = UpdateUser {
        username: payload.username.clone(),
        email: payload.email.clone(),
        password: payload.password.clone().map(|pwd| hash_password(pwd)),
        roles: payload.roles.clone()
    };


    let result = RepositoryUsers::update_user(id, user);

    match result {
        Ok(user) => Ok((StatusCode::OK, Json(user))),//pas mieux de retourner un status 200, avec message " user bien modifié " ?
        Err(e) => {
            let err = ErrorResponse::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e.to_string());
            Err(
                (StatusCode::from_u16(err.code_error).unwrap(), Json(err))
            )
        },
    }
}

pub async fn delete_user(Path(id): Path<i32>) -> impl IntoResponse {
    let result = RepositoryUsers::delete_user(id);

    match result {
        Ok(user) => {
            Ok((StatusCode::OK, Json(format!("user : {}, ID : {}, has been deleted", user.username, user.id))))
        },
        Err(e) => {
            let err = ErrorResponse::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e.to_string());
            Err(
                (StatusCode::from_u16(err.code_error).unwrap(), Json(err))
            )
        },
    }
}

fn hash_password(pwd: String) -> String {
    hash(pwd, DEFAULT_COST).expect("Error hashing password")
}
