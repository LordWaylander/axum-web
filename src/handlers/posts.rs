use axum::{
    http::StatusCode,
    Json,
};

use crate::errors::error;

use diesel::prelude::*;
use diesel::result::Error;
use crate::schema;
use crate::database;
use crate::models::posts::{NewPost, Post, UpdatePost};
use crate::schema::posts;

use axum::response::Html;

pub async fn hello_fn() -> Html<&'static str> {
    Html("<h1>Hello !</h1>")
}

pub async fn show_posts() -> Result<Json<Vec<Post>>, Json<crate::errors::ErrorResponse>> {

    let connection = &mut database::establish_connection();

    let result: Result<Vec<Post>, Error> = connection.transaction(|conn| {
        let posts = posts::table
        .order(posts::id.asc())
        //.filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .get_results(conn)?;

        Ok(posts)
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

pub async fn create_post(Json(payload): Json<NewPost>) -> Result<Json<Post>, Json<crate::errors::ErrorResponse>> {
    let connection = &mut database::establish_connection();

    let new_post = NewPost { 
        title : payload.title, 
        body : payload.body, 
        published: Some(payload.published.unwrap_or(false))
    };

    let result: Result<Post, Error> = connection.transaction(|conn| {
        diesel::insert_into(posts::table)
            .values(&new_post)
            .execute(conn)?;

            let post = posts::table
            .order(posts::id.desc())
            .select(Post::as_select())
            .first(conn)?;

            Ok(post)
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

pub async fn update_post(Json(payload): Json<UpdatePost>) -> Result<Json<Post>, Json<crate::errors::ErrorResponse>> {

    let connection = &mut database::establish_connection();

    let result: Result<Post, Error> = connection.transaction(|connection| {

        let update_post = UpdatePost {
            id: payload.id,
            title: payload.title,
            body: payload.body,
            published: payload.published,
        };

        diesel::update(posts::table.find(payload.id))
            .set(&update_post)
            .execute(connection)?;

            let post = posts::table
            .find(payload.id)
            .select(Post::as_select())
            .first(connection)?;

        Ok(post)
    });

    match result {
        Ok(post) => Ok(Json(post)),
        Err(e) => {
            let msg = format!("Error: {}", e);
            let err = error(StatusCode::INTERNAL_SERVER_ERROR.to_string(), msg);
            Err(Json(err))
        },
    }

}