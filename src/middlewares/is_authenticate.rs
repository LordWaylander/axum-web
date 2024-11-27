use axum:: {
    extract::Request,
    middleware::Next,
    response::Response,
};
use crate::middlewares::get_token_from_header;
use crate::format_responses::ErrorResponse;

pub async fn main(req: Request, next: Next) -> Result<Response, ErrorResponse>   {

    match get_token_from_header(&req) {
        Ok(_) => {
            Ok(next.run(req).await) 
        }
        Err(e) => {
            Err(e)
        }
    }
}