use axum::{
    Router,
    routing::{get, post, patch, delete},
};
use crate::handlers::users as handler_users;

pub fn init_users_routes() -> Router {
    let app = Router::new()
        .route("/admin", get(handler_users::show_users))
        .route("/admin/show_user/:id", get(handler_users::get_one_user))
        .route("/admin/create_user", post(handler_users::create_user))
        .route("/admin/update_user/:id", patch(handler_users::update_user))
        .route("/admin/delete_user/:id", delete(handler_users::delete_user));

    return app;
}
