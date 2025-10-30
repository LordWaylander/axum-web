use axum:: {
    extract::{Request, Json},
    middleware::Next,
    response::IntoResponse,
    http::StatusCode,
};
use crate::middlewares::get_token_from_header;
use crate::format_responses::ErrorResponse;

pub async fn main(req: Request, next: Next) -> impl IntoResponse   {
    match get_token_from_header(&req) {
        Ok(token_data) => {
            let roles = token_data.claims.roles;

            if roles.contains("ROLE_ADMIN") {
                Ok(next.run(req).await)
            } else {
                let err = ErrorResponse::error(StatusCode::UNAUTHORIZED.as_u16(),"Not enough rights to access this ressource".to_string() );
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