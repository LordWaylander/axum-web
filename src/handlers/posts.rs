use axum::{
    http::StatusCode,
    Json,
    extract::Path,
    response::IntoResponse
};
use crate::repository::post as RepositoryPost;
use crate::models::posts::{NewPost, UpdatePost};
use crate::format_responses::{PostResponse, ErrorResponse};

pub async fn show_posts() -> impl IntoResponse {

    let result = RepositoryPost::get_all_posts();

    match result {
        Ok(response) => {
            if response.len() == 0 {
                let err = ErrorResponse::error(StatusCode::NOT_FOUND.as_u16(),"No posts found".to_string());
                Err(
                    (StatusCode::from_u16(err.code_error).unwrap(), Json(err))
                )
            } else {
                
                let mut resp_json = Vec::new();

                for r in response {
                    resp_json.push(PostResponse { post: r.0, user: r.1 })
                }


                Ok(
                    (StatusCode::OK, Json(resp_json))
                )
            }
        },
        Err(e) => {
            let err = ErrorResponse::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(),e.to_string() );
            Err(
                (StatusCode::from_u16(err.code_error).unwrap(), Json(err))
            )
        },
    }
}

pub async fn get_one_post(Path(id): Path<i32>) -> impl IntoResponse {

    let result = RepositoryPost::get_one_post(id);

    match result {
        Ok(response) => {
            Ok(
                (StatusCode::OK, Json(response))
            )
        },
        Err(e) => {
            let err = ErrorResponse::error(StatusCode::NOT_FOUND.as_u16(), e.to_string());
            Err(
                (StatusCode::from_u16(err.code_error).unwrap(), Json(err))
            )
        },
    }
}

pub async fn create_post(Json(payload): Json<NewPost>) -> impl IntoResponse {

    let result = RepositoryPost::create_post(payload);

    match result {
        Ok(post) => {
            Ok(
                (StatusCode::OK, Json(post))
            )
        },
        Err(e) => {
            let err = ErrorResponse::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(),e.to_string() );
            Err(
                (StatusCode::from_u16(err.code_error).unwrap(), Json(err))
            )
        },
    }
}

pub async fn update_post(Path(id): Path<i32>, Json(payload): Json<UpdatePost>) -> impl IntoResponse {

    let result = RepositoryPost::update_post(id, payload);

    match result {
        Ok(post) => Ok((StatusCode::OK, Json(post))),
        Err(e) => {
            let err = ErrorResponse::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e.to_string());
            Err(
                (StatusCode::from_u16(err.code_error).unwrap(), Json(err))
            )
        },
    }
}

pub async fn delete_post(Path(id): Path<i32>) -> impl IntoResponse {

    let result = RepositoryPost::delete_post(id);

    match result {
        Ok(post) => {
            Ok((StatusCode::OK, Json(format!("Le post : {}, ID : {}, est bien supprimé", post.title, post.id))))
        },
        Err(e) => {
            let err = ErrorResponse::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), e.to_string());
            Err(
                (StatusCode::from_u16(err.code_error).unwrap(), Json(err))
            )
        },
    }
}