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
use crate::errors::{ErrorResponse,error};
use crate::repository::post as RepositoryPost;

pub async fn main(mut req: Request, next: Next) -> Result<Response, Json<ErrorResponse>>   {
    match get_token_from_header(&req) {
        Ok(token) => {
            if let Ok(param_uri) =  req.extract_parts().await.map(|Path::<i32>(path_params)| path_params) {

                let result = RepositoryPost::get_one_post(param_uri);

                match result {
                    Ok(post) => {
                        if token.claims.id  == post.1.id {
                            Ok(next.run(req).await) 
                        } else {
                            let err = error(StatusCode::UNAUTHORIZED.to_string(),"You are not the post's proprietary".to_string());
                            Err(err)
                        }
                    },
                    Err(e) => {
                        let err = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), e.to_string());
                        Err(err)
                    },
                }
            } else {
                let err = error(StatusCode::NOT_ACCEPTABLE.to_string(),"No params URI found".to_string());
                    Err(err)
            }
        }
        Err(e) => {
            Err(e)
        }
    }
}