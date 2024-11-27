use axum::{
    Router,
    routing::{get, post, patch, delete},
    middleware::{self},
};
use crate::handlers::posts as handler_posts;
use crate::middlewares::{is_authenticate, is_proprietary_post};


pub fn init_api_routes() -> Router {
    let app = Router::new()
        .route("/", get(handler_posts::show_posts))
        .route("/show_post/:id", get(handler_posts::get_one_post))
        ;

        let authenticate = Router::new()
            .route("/create_post", post(handler_posts::create_post))
            .route_layer(middleware::from_fn(is_authenticate::main))
        ;

        let proprietary = Router::new()
        .route("/update_post/:id", patch(handler_posts::update_post))
            .route_layer(middleware::from_fn(is_authenticate::main))
            .route_layer(middleware::from_fn(is_proprietary_post::main))
        .route("/delete_post/:id", delete(handler_posts::delete_post))
            .route_layer(middleware::from_fn(is_authenticate::main))
            .route_layer(middleware::from_fn(is_proprietary_post::main))
        ;

    return app
        .merge(authenticate)
        .merge(proprietary)
    ;
}