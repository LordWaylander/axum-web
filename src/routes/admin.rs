use axum::{
    Router,
    routing::{get, post, patch, delete},
};
use crate::handlers::admin;

pub fn init_admin_routes() -> Router {
    let app = Router::new()
        .route("/admin", get(admin::show_users))
        .route("/admin/show_user/:id", get(admin::get_one_user))
        .route("/admin/create_user", post(admin::create_user))
        .route("/admin/update_user", patch(admin::update_user))
        .route("/admin/delete_user/:id", delete(admin::delete_user));

    return app;
}
