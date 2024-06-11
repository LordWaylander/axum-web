use axum::{
    http::StatusCode,
    Json,
};

use crate::errors::error;
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

pub async fn show_posts() -> Result<Json<Vec<Post>>, Json<crate::errors::ErrorResponse>> {

    let connection = &mut database::establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    if results.len() == 0 {
        let err: crate::errors::ErrorResponse = error(StatusCode::INTERNAL_SERVER_ERROR.as_str(), "No Posts found");
    
        Err(Json(err))
    } else {
        println!("Displaying {} posts", results.len());
        
        Ok(Json(results))
    }
}

pub async fn create_posts() -> () {

}