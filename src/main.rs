use axum;
use tokio::net::TcpListener;
use std::env;
use dotenvy::dotenv;
mod routes;
mod handlers;
mod middlewares;
mod database;
mod models;
mod schema;
mod repository;
mod format_responses;

#[tokio::main]
async fn main() {
    check_env();
    
    let address = env::var("ADDRESS").unwrap();

    let app = routes::init();

    let listener = TcpListener::bind(address)
        .await
        .expect("Unable to connect to the server");

    println!("Listening on {}", listener.local_addr().unwrap() );

    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}

fn check_env() {
    dotenv().ok();
    env::var("ADDRESS").expect("ADDRESS must be set");
    env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    env::var("SECRET_KEY").expect("SECRET_KEY must be set");
}