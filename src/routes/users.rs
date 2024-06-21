use axum::{
    Router,
    routing::{get, post, patch, delete},
};
use crate::handlers::users;

pub fn init_users_routes() -> Router {
    let app = Router::new()
        .route("/admin", get(users::show_users))
        .route("/admin/show_user/:id", get(users::get_one_user))
        .route("/admin/create_user", post(users::create_user))
        .route("/admin/update_user/:id", patch(users::update_user))
        .route("/admin/delete_user/:id", delete(users::delete_user));

    return app;
}
