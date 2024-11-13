use axum::{
    Router,
    routing::post,
};

use crate::handlers::authenticate as handler_auth;

pub fn init_auth_routes() -> Router {
    let app = Router::new()
    .route("/login", post(handler_auth::login));

    return app;
}