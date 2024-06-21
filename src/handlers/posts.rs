use axum::{
    http::StatusCode,
    Json,
    extract::Path
};
use crate::errors::error;
use crate::repository::post;
use crate::models::posts::{NewPost, Post, UpdatePost};
use crate::models::users::User;

pub async fn show_posts() -> Result<Json<Vec<(Post, User)>>, Json<crate::errors::ErrorResponse>> {

    let result = post::get_all_posts();

    match result {
        Ok(post) => {
            if post.len() == 0 {
                let msg: String = format!("No posts found");
                let err: crate::errors::ErrorResponse = error(StatusCode::OK.to_string(),msg );
                Err(Json(err))
            } else {
                Ok(Json(post))
            }
            
        },
        Err(e) => {
            let err: crate::errors::ErrorResponse = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(),e.to_string() );
            Err(Json(err))
        },
    }
}

pub async fn get_one_post(Path(id): Path<i32>) -> Result<Json<(Post, User)>, Json<crate::errors::ErrorResponse>> {

    let result = post::get_one_post(Path(id));

    match result {
        Ok(post) => Ok(Json(post)),
        Err(e) => {
            let err = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), e.to_string());
            Err(Json(err))
        },
    }
}

pub async fn create_post(Json(payload): Json<NewPost>) -> Result<Json<Post>, Json<crate::errors::ErrorResponse>> {

    let result = post::create_post(Json(payload));

    match result {
        Ok(post) => Ok(Json(post)),
        Err(e) => {
            let err: crate::errors::ErrorResponse = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(),e.to_string() );
            Err(Json(err))
        },
    }
}

pub async fn update_post(Json(payload): Json<UpdatePost>) -> Result<Json<Post>, Json<crate::errors::ErrorResponse>> {

    let result = post::update_post(Json(payload));

    match result {
        Ok(post) => Ok(Json(post)),
        Err(e) => {
            let err = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), e.to_string());
            Err(Json(err))
        },
    }
}

pub async fn delete_post(Path(id): Path<i32>) -> Result<Json<String>, Json<crate::errors::ErrorResponse>> {

    let result = post::delete_post(Path(id));

    match result {
        Ok(post) => {
            Ok(Json(format!("Le post : {}, ID : {}, est bien supprimé", post.title, post.id)))
        },
        Err(e) => {
            let err = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), e.to_string());
            Err(Json(err))
        },
    }
}