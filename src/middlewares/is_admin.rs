use axum:: {
    extract::Request,
    middleware::Next,
    response::{Response, Redirect}
};
use crate::middlewares::get_token_from_header;

pub async fn main(req: Request, next: Next) -> Result<Response, Redirect>   {
    match get_token_from_header(&req) {
        Ok(info_user_token) => {
            let roles = info_user_token.roles;

            if roles.contains("ROLE_ADMIN") {
                Ok(next.run(req).await)
            } else {
                Err(Redirect::to("/"))
            }    
        }
        Err(e) => {
            Err(Redirect::to("/"))
        }
    }
}