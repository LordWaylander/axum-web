use axum::{
    Router,
    routing::{get, post},
    middleware::{self},
};
use crate::middlewares::*;
use crate::handlers::*;

pub fn init() -> Router {
    let mut app = Router::new()
        .route("/", get(hello::hello_fn))
        .route("/truc", get(hello::truc_fn))
        .route("/api", post(api::create_user))
        .layer(middleware::from_fn(redirect::check_if_redirect_to))
        ;

    app = Router::route(app, "/machin/:key", get(api::key_fn));

    return app;
}

