use axum::{
    Router,
    routing::{get, post},
};
use crate::handlers::posts;

pub fn init_api_routes() -> Router {
    let app = Router::new()
        .route("/", get(posts::hello_fn))
        .route("/show_posts", get(posts::show_posts))
        .route("/create_post", post(posts::create_post));

    return app;
}
