use axum::{
    Router,
    routing::get,
};
use crate::handlers::admin;

pub fn init_admin_routes() -> Router {
    let app = Router::new()
        .route("/admin", get(admin::hello_fn))
        .route("/admin/truc", get(admin::truc_fn));

    return app;
}
