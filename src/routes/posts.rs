use axum::{
    Router,
    routing::get,
};
use crate::handlers::posts;

pub fn init_api_routes() -> Router {
    let app = Router::new()
        .route("/", get(posts::hello_fn))
        .route("/show_posts", get(posts::show_posts));

    return app;
}
