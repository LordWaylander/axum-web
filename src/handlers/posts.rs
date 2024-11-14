use axum::{
    http::StatusCode,
    Json,
    extract::Path
};
use crate::errors::error;
use crate::repository::post as RepositoryPost;
use crate::models::posts::{NewPost, Post, UpdatePost};
use crate::models::users::User;
use crate::errors::ErrorResponse;

pub async fn show_posts() -> Result<Json<Vec<(Post, User)>>, Json<ErrorResponse>> {

    let result = RepositoryPost::get_all_posts();

    match result {
        Ok(post) => {
            if post.len() == 0 {
                let err = error(StatusCode::OK.to_string(),"No posts found".to_string());
                Err(err)
            } else {
                Ok(Json(post))
            }
        },
        Err(e) => {
            let err = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(),e.to_string() );
            Err(err)
        },
    }
}

pub async fn get_one_post(Path(id): Path<i32>) -> Result<Json<(Post, User)>, Json<ErrorResponse>> {

    let result = RepositoryPost::get_one_post(id);

    match result {
        Ok(post) => Ok(Json(post)),
        Err(e) => {
            let err = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), e.to_string());
            Err(err)
        },
    }
}

pub async fn create_post(Json(payload): Json<NewPost>) -> Result<Json<Post>, Json<ErrorResponse>> {

    let result = RepositoryPost::create_post(payload);

    match result {
        Ok(post) => Ok(Json(post)),
        Err(e) => {
            let err = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(),e.to_string() );
            Err(err)
        },
    }
}

pub async fn update_post(Path(id): Path<i32>, Json(payload): Json<UpdatePost>) -> Result<Json<Post>, Json<ErrorResponse>> {

    let result = RepositoryPost::update_post(id, payload);

    match result {
        Ok(post) => Ok(Json(post)),
        Err(e) => {
            let err = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), e.to_string());
            Err(err)
        },
    }
}

pub async fn delete_post(Path(id): Path<i32>) -> Result<Json<String>, Json<ErrorResponse>> {

    let result = RepositoryPost::delete_post(id);

    match result {
        Ok(post) => {
            Ok(Json(format!("Le post : {}, ID : {}, est bien supprimé", post.title, post.id)))
        },
        Err(e) => {
            let err = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), e.to_string());
            Err(err)
        },
    }
}