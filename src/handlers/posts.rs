/*use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
    extract::{Path, Query},
};*/
//use serde::{Deserialize, Serialize};

use diesel::prelude::*;
use crate::schema;
use crate::database;
use crate::models::posts::Post;
use schema::posts::dsl::*;

use axum::response::Html;

pub async fn hello_fn() -> Html<&'static str> {
    Html("<h1>Hello !</h1>")
}

pub async fn show_posts() -> () {
    

    let connection = &mut database::establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}