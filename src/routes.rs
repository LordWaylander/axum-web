use axum::{
    Router,
    routing::{get, post},
    middleware::{self},
};
use crate::middlewares::*;

pub mod api;
pub mod admin;

pub fn init() -> Router {
    let mut app = Router::new()
    .merge(admin::init_admin_routes())
    .merge(api::init_api_routes());

    app = app.layer(middleware::from_fn(redirect::check_if_redirect_to));

    return app;
}

