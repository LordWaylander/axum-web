use axum::{
    Router,
    middleware::{self},
};
use crate::middlewares::*;

pub mod posts;
pub mod users;

pub fn init() -> Router {
    let mut app = Router::new()
    .merge(users::init_users_routes())
    .merge(posts::init_api_routes());

    //middleware pour tous
    app = app.layer(middleware::from_fn(redirect::check_if_redirect_to));

    return app;
}

