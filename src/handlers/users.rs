use axum::{
    http::StatusCode,
    Json,
    extract::Path,
};
use crate::errors::error;
use crate::repository::users as RepositoryUsers;
use crate::models::users::{NewUser, UpdateUser, User};
use crate::models::posts::Post;
use crate::errors::ErrorResponse;
use bcrypt::{hash, DEFAULT_COST};

pub async fn show_users() -> Result<Json<Vec<(User, Vec<Post>)>>, Json<ErrorResponse>> {
    let result: Result<_, _> = RepositoryUsers::get_all_users();

    match result {
        Ok(users) => {
            if users.len() == 0 {
                let msg: String = format!("No users found");
                let err: ErrorResponse = error(StatusCode::OK.to_string(),msg );
                Err(Json(err))
            } else {
                Ok(Json(users))
            }
            
        },
        Err(e) => {
            let err: ErrorResponse = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(),e.to_string() );
            Err(Json(err))
        },
    }
}

pub async fn get_one_user(Path(id): Path<i32>) -> Result<Json<(User, Vec<Post>)>, Json<ErrorResponse>> {
    let result = RepositoryUsers::get_one_user(id);

    match result {
        Ok(user) => {          
            Ok(Json(user))
        },
        Err(e) => {
            let err = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), e.to_string());
            Err(Json(err))
        },
    }
}

pub async fn create_user(payload: Json<NewUser>) -> Result<Json<User>, Json<ErrorResponse>> {

    let user = NewUser {
        username: payload.username.clone(),
        email: payload.email.clone(),
        password: hash_password(payload.password.clone()),
        roles: payload.roles.clone()
    };

    let result = RepositoryUsers::create_user(user);

    match result {
        Ok(user) => Ok(Json(user)),
        Err(e) => {
            let err: ErrorResponse = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(),e.to_string() );
            Err(Json(err))
        },
    }
}

pub async fn update_user(Path(id): Path<i32>, payload: Json<UpdateUser>) -> Result<Json<User>, Json<ErrorResponse>> {

    let user: UpdateUser = UpdateUser {
        username: payload.username.clone(),
        email: payload.email.clone(),
        password: payload.password.clone().map(|pwd| hash_password(pwd)),
        roles: payload.roles.clone()
    };


    let result = RepositoryUsers::update_user(id, user);

    match result {
        Ok(user) => Ok(Json(user)),
        Err(e) => {
            let err = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), e.to_string());
            Err(Json(err))
        },
    }
}

pub async fn delete_user(Path(id): Path<i32>) -> Result<Json<String>, Json<ErrorResponse>> {
    let result = RepositoryUsers::delete_user(id);

    match result {
        Ok(user) => {
            Ok(Json(format!("L'utilisateur : {}, ID : {}, est bien supprimé", user.username, user.id)))
        },
        Err(e) => {
            let err = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), e.to_string());
            Err(Json(err))
        },
    }
}

fn hash_password(pwd: String) -> String {
    hash(pwd, DEFAULT_COST).expect("Error hashing password")
}
