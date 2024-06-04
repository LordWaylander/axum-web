use axum::{
    Router,
    routing::{get, post},
};
use crate::handlers::*;

pub fn init_api_routes() -> Router {
    let mut app = Router::new()
        .route("/", get(hello::hello_fn))
        .route("/truc", get(hello::truc_fn))
        .route("/api", post(api::create_user));
        
        app = Router::route(app, "/machin/:key/:key2", get(api::key_fn));

    return app;
}
