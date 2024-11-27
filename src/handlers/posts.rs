use axum::{
    http::StatusCode,
    Json,
    extract::Path,
    response::{IntoResponse, Response},
};
use crate::repository::post as RepositoryPost;
use crate::models::posts::{NewPost, Post, UpdatePost};
use crate::format_responses::{PostResponse, ErrorResponse};

pub async fn show_posts() -> Result<Json<Vec<PostResponse>>, ErrorResponse> {

    let result = RepositoryPost::get_all_posts();

    match result {
        Ok(response) => {
            if response.len() == 0 {
                let err = ErrorResponse::error(StatusCode::OK.as_u16(),"No posts found".to_string());
                Err(err)
            } else {
                
                let mut resp_json = Vec::new();

                for r in response {
                    resp_json.push(PostResponse { post: r.0, user: r.1 })
                }


                Ok(Json(resp_json))
            }
        },
        Err(e) => {
            let err = ErrorResponse::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(),e.to_string() );
            Err(err)
        },
    }
}

use axum::debug_handler;

#[debug_handler]
pub async fn get_one_post(Path(id): Path<i32>) -> Result<Json<PostResponse>, ErrorResponse> {

    let result = RepositoryPost::get_one_post(id);

    match result {
        Ok(response) => {
            Ok(Json(PostResponse{post: response.0, user: response.1}))
        },
        Err(e) => {
            let err = ErrorResponse::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e.to_string());
            Err(err)
        },
    }
}

pub async fn create_post(Json(payload): Json<NewPost>) -> Result<Json<Post>, ErrorResponse> {

    let result = RepositoryPost::create_post(payload);

    match result {
        Ok(post) => Ok(Json(post)),
        Err(e) => {
            let err = ErrorResponse::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(),e.to_string() );
            Err(err)
        },
    }
}

pub async fn update_post(Path(id): Path<i32>, Json(payload): Json<UpdatePost>) -> Result<Json<Post>, ErrorResponse> {

    let result = RepositoryPost::update_post(id, payload);

    match result {
        Ok(post) => Ok(Json(post)),
        Err(e) => {
            let err = ErrorResponse::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e.to_string());
            Err(err)
        },
    }
}

pub async fn delete_post(Path(id): Path<i32>) -> Result<Json<String>, ErrorResponse> {

    let result = RepositoryPost::delete_post(id);

    match result {
        Ok(post) => {
            Ok(Json(format!("Le post : {}, ID : {}, est bien supprimé", post.title, post.id)))
        },
        Err(e) => {
            let err = ErrorResponse::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e.to_string());
            Err(err)
        },
    }
}