use axum::{
    Router,
    routing::{get, post, patch, delete},
    middleware::{self},
};

use tower_http::services::{ServeDir, ServeFile};

use crate::handlers::medias as handler_upload;

use crate::middlewares::is_authenticate;

pub fn init_upload_routes() -> Router {
    let app = Router::new()
        .route("/upload", get(handler_upload::get_all_upload))
        .route("/upload/:id", get(handler_upload::get_one_upload))
        .route("/upload", post(handler_upload::post_upload))
        //.route("/upload:id", patch(handler_upload::update_upload))
        //.route("/upload:id", delete(handler_upload::delete_upload))
        .layer(middleware::from_fn(is_authenticate::main))
        .nest_service("/uploads", ServeDir::new("uploads"));

    return app;
}
