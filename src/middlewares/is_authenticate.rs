use axum:: {
    extract::{Request, Json},
    middleware::Next,
    response::IntoResponse,
    http::StatusCode,
};
use crate::middlewares::get_token_from_header;

pub async fn main(req: Request, next: Next) -> impl IntoResponse   {

    match get_token_from_header(&req) {
        Ok(_) => {
            Ok(next.run(req).await) 
        }
        Err(e) => {
            Err(
                (StatusCode::from_u16(e.code_error).unwrap(), Json(e))
            )
        }
    }
}