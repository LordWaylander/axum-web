use axum::{
    http::StatusCode,
    Json,
};

use crate::errors::error;
//use serde::{Deserialize, Serialize};

use diesel::prelude::*;
use diesel::result::Error;
use crate::schema;
use crate::database;
use crate::models::posts::{NewPost, Post};
use schema::posts::dsl::*;
use crate::schema::posts;

use axum::response::Html;

pub async fn hello_fn() -> Html<&'static str> {
    Html("<h1>Hello !</h1>")
}

pub async fn show_posts() -> Result<Json<Vec<Post>>, Json<crate::errors::ErrorResponse>> {

    let connection = &mut database::establish_connection();
    let results = posts
        //.filter(published.eq(true))
        //.limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    if results.len() == 0 {
        let err: crate::errors::ErrorResponse = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), "No Posts found".to_string());
    
        Err(Json(err))
    } else {
        println!("Displaying {} posts", results.len());
        
        Ok(Json(results))
    }
}

pub async fn create_post(Json(payload): Json<NewPost>) -> Result<Json<Post>, Json<crate::errors::ErrorResponse>> {
    let connection = &mut database::establish_connection();
    let new_post = NewPost { title : payload.title, body : payload.body};

        let result: Result<Post, Error> = connection.transaction(|conn| {
            diesel::insert_into(posts::table)
                .values(&new_post)
                .execute(conn)?;

                posts::table
                .order(posts::id.desc())
                .select(Post::as_select())
                .first(conn)

        }); 
        match result {
            Ok(post) => Ok(Json(post)),
            Err(e) => {
                let msg: String = format!("Error : {e}");
                let err: crate::errors::ErrorResponse = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(),msg );
                Err(Json(err))
            },
        }
}