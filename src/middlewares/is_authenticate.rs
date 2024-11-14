use axum:: {
    extract::Request,
    middleware::Next,
    response::Response,
    Json,
    http::StatusCode,
};
use crate::middlewares::get_token_from_header;
use crate::errors::{ErrorResponse, error};

pub async fn main(req: Request, next: Next) -> Result<Response, Json<ErrorResponse>>   {

    match get_token_from_header(&req) {
        Ok(_) => {
            Ok(next.run(req).await) 
        }
        Err(e) => {
            Err(e)
        }
    }
}