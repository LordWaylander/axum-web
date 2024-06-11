use axum;
use std::env;
use dotenvy::dotenv;
mod routes;
mod handlers;
mod middlewares;
mod database;
mod models;
mod schema;
mod errors;

#[tokio::main]
async fn main() {
    check_env();
    
    let address = env::var("ADDRESS").expect("ADDRESS must be set");

    //tracing_subscriber::fmt::init();
    let app = routes::init();

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn check_env() {
    dotenv().ok();
    env::var("ADDRESS").expect("ADDRESS must be set");
    env::var("DATABASE_URL").expect("DATABASE_URL must be set");
}