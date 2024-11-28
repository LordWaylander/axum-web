use axum::Router;

pub mod posts;
pub mod users;
pub mod auth;
pub mod medias;

pub fn init() -> Router {
    let app = Router::new()
    .merge(users::init_users_routes())
    .merge(posts::init_api_routes())
    .merge(auth::init_auth_routes())
    .merge(medias::init_upload_routes());

    return app;
}

