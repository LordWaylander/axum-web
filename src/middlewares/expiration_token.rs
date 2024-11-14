use axum:: {
    extract::Request,
    middleware::Next,
    response::{Response, Redirect},
};
use chrono::Utc;
use crate::middlewares::get_token_from_header;

pub async fn main(req: Request, next: Next) -> Result<Response, Redirect>   {
    match get_token_from_header(&req) {
        Ok(info_user_token) => {
            let now = Utc::now().timestamp() as usize;

            if now > info_user_token.exp {
                Err(Redirect::to("/"))
            } else {
                Ok(next.run(req).await)
            }
        }
        Err(e) => {
            Err(Redirect::to("/"))
        }
    }
}