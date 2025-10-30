use axum:: {
    extract::{Request, Json},
    middleware::Next,
    response::IntoResponse,
    extract::Path,
    http::StatusCode
};
use axum::RequestExt;
use crate::middlewares::get_token_from_header;
use crate::format_responses::ErrorResponse;
use crate::repository::post::get_one_post;

pub async fn main(mut req: Request, next: Next) -> impl IntoResponse  {
    match get_token_from_header(&req) {
        Ok(token) => {
            if let Ok(param_uri) =  req.extract_parts().await.map(|Path::<i32>(path_params)| path_params) {

                let post = get_one_post(param_uri);

                match post {
                    Ok(post) => {
                        if token.claims.id  == post.1.id {
                            Ok(next.run(req).await) 
                        } else {
                            let err = ErrorResponse::error(StatusCode::UNAUTHORIZED.as_u16(),"You are not the post's proprietary".to_string());
                            Err(
                                (StatusCode::from_u16(err.code_error).unwrap(), Json(err))
                            )
                        }
                    },
                    Err(e) => {
                        let err = ErrorResponse::error(StatusCode::UNAUTHORIZED.as_u16(),e.to_string());
                        Err(
                            (StatusCode::from_u16(err.code_error).unwrap(), Json(err))
                        )
                    }
                }
            } else {
                let err = ErrorResponse::error(StatusCode::NOT_ACCEPTABLE.as_u16(),"No params URI found".to_string());
                Err(
                    (StatusCode::from_u16(err.code_error).unwrap(), Json(err))
                )
            }
        }
        Err(e) => {
            Err(
                (StatusCode::from_u16(e.code_error).unwrap(), Json(e))
            )
        }
    }
}