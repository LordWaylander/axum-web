use axum:: {
    extract::Request,
    middleware::Next,
    response::Response,
    Json,
    extract::Path,
    http::StatusCode
};
use axum::RequestExt;
use crate::middlewares::get_token_from_header;
use crate::format_responses::ErrorResponse;
use crate::repository::post::get_one_post;

pub async fn main(mut req: Request, next: Next) -> Result<Response, ErrorResponse>   {
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
                            Err(err)
                        }
                    },
                    Err(e) => {
                        let err = ErrorResponse::error(StatusCode::UNAUTHORIZED.as_u16(),e.to_string());
                        Err(err)
                    }
                }
            } else {
                let err = ErrorResponse::error(StatusCode::NOT_ACCEPTABLE.as_u16(),"No params URI found".to_string());
                    Err(err)
            }
        }
        Err(e) => {
            Err(e)
        }
    }
}