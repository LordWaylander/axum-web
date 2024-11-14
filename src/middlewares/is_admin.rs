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
        Ok(token_data) => {
            let roles = token_data.claims.roles;

            if roles.contains("ROLE_ADMIN") {
                Ok(next.run(req).await)
            } else {
                let err = error(StatusCode::UNAUTHORIZED.to_string(),"Not enough rights to access this ressource".to_string() );
                Err(err)
            }    
        }
        Err(e) => {
            Err(e)
        }
    }
}