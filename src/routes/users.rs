use axum::{
    Router,
    routing::{get, post, patch, delete},
    middleware::{self},
};

use crate::handlers::users as handler_users;

use crate::middlewares::{is_admin, expiration_token};

pub fn init_users_routes() -> Router {
    let app = Router::new()
        .route("/admin", get(handler_users::show_users))
        .route("/admin/show_user/:id", get(handler_users::get_one_user))
        .route("/admin/create_user", post(handler_users::create_user))
        .route("/admin/update_user/:id", patch(handler_users::update_user))
        .route("/admin/delete_user/:id", delete(handler_users::delete_user))
        .layer(middleware::from_fn(is_admin::main))
        .layer(middleware::from_fn(expiration_token::main));

    return app;
}
