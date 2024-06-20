use axum::{
    Router,
    routing::{get, post, patch, delete},
};
use crate::handlers::posts;

pub fn init_api_routes() -> Router {
    let app = Router::new()
        .route("/", get(posts::show_posts))
        .route("/show_post/:id", get(posts::get_one_post))
        .route("/create_post", post(posts::create_post))
        .route("/update_post", patch(posts::update_post))
        .route("/delete_post/:id", delete(posts::delete_post));

    return app;
}
