use axum::{
    http::StatusCode,
    Json,
    extract::Path
};
use crate::errors::error;
use crate::repository::users;
use crate::models::users::{NewUser, UpdateUser, User};
use crate::models::posts::Post;

pub async fn show_users() -> Result<Json<Vec<(User, Vec<Post>)>>, Json<crate::errors::ErrorResponse>> {
    let result: Result<_, _> = users::get_all_users();

    match result {
        Ok(users) => {
            if users.len() == 0 {
                let msg: String = format!("No users found");
                let err: crate::errors::ErrorResponse = error(StatusCode::OK.to_string(),msg );
                Err(Json(err))
            } else {
                Ok(Json(users))
            }
            
        },
        Err(e) => {
            let err: crate::errors::ErrorResponse = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(),e.to_string() );
            Err(Json(err))
        },
    }
}

pub async fn get_one_user(Path(id): Path<i32>) -> Result<Json<Vec<(User, Vec<Post>)>>, Json<crate::errors::ErrorResponse>> {
    let result = users::get_one_user(Path(id));

    match result {
        Ok(user) => Ok(Json(user)),
        Err(e) => {
            let err = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), e.to_string());
            Err(Json(err))
        },
    }
}

pub async fn create_user(Json(payload): Json<NewUser>) -> Result<Json<User>, Json<crate::errors::ErrorResponse>> {
    let result = users::create_user(Json(payload));

    match result {
        Ok(user) => Ok(Json(user)),
        Err(e) => {
            let err: crate::errors::ErrorResponse = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(),e.to_string() );
            Err(Json(err))
        },
    }
}

pub async fn update_user(Json(payload): Json<UpdateUser>) -> Result<Json<User>, Json<crate::errors::ErrorResponse>> {

    let result = users::update_user(Json(payload));

    match result {
        Ok(user) => Ok(Json(user)),
        Err(e) => {
            let err = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), e.to_string());
            Err(Json(err))
        },
    }
}

pub async fn delete_user(Path(id): Path<i32>) -> Result<Json<String>, Json<crate::errors::ErrorResponse>> {
    let result = users::delete_user(Path(id));

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
