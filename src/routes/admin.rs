use axum::{
    Router,
    routing::{get, post, patch, delete},
};
use crate::handlers::admin;

pub fn init_admin_routes() -> Router {
    let app = Router::new()
        .route("/admin", get(admin::show_users))
        .route("/show_user/:id", get(admin::get_one_user))
        .route("/create_user", post(admin::create_user))
        .route("/update_user", patch(admin::update_user))
        .route("/delete_user/:id", delete(admin::delete_user));

    return app;
}
