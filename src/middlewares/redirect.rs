use tower::ServiceBuilder;
use axum:: {
    extract::Request,
    middleware::Next,
    response::{Response, Redirect},
    http::StatusCode,
};

pub async fn check_if_redirect_to(req: Request, next: Next,) -> Result<Response, Redirect> {
    let original_url = req.uri().path().to_owned();
    let uri_splitted = original_url.split('/').collect::<Vec<&str>>();

    if uri_splitted[1] == "truc" {
        //Err(StatusCode::UNAUTHORIZED)
        Err(Redirect::to("/"))
    } else {
        Ok(next.run(req).await)
    }
}