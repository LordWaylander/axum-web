//use tower::ServiceBuilder;
use axum:: {
    extract::Request,
    middleware::Next,
    response::{Response, Redirect},
    //http::StatusCode,
};

pub async fn main(req: Request, next: Next) -> Response {
    // is authenticate && is admin -> OK
    println!("{:?}", req);

    next.run(req).await
}