use axum::{
    Router,
    routing::{get, post, patch, delete},
};
use crate::handlers::posts as handler_posts;

pub fn init_api_routes() -> Router {
    let app = Router::new()
        .route("/", get(handler_posts::show_posts))
        .route("/show_post/:id", get(handler_posts::get_one_post))
        .route("/create_post", post(handler_posts::create_post))
        .route("/update_post/:id", patch(handler_posts::update_post))
        .route("/delete_post/:id", delete(handler_posts::delete_post));

    return app;
}
