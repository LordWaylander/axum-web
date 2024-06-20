use crate::models::posts::{NewPost, Post, UpdatePost};
use crate::schema::posts;
use crate::schema::posts::dsl::*;
use crate::database;
use diesel::prelude::*;
use diesel::result::Error;

use axum::{
    Json,
    extract::Path
};

pub fn get_all_posts() -> Result<Vec<Post>, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<Vec<Post>, Error> = connection.transaction(|connection| {
        let posts_vector = posts::table
        .order(posts::id.asc())
        //.filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .get_results(connection)?;

        Ok(posts_vector)
    });

    return result;
}

pub fn get_one_post(Path(other_id): Path<i32>) -> Result<Post, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<Post, Error> = connection.transaction(|connection| {
            let post = posts::table
            .find(other_id)
            .select(Post::as_select())
            .first(connection)?;

        Ok(post)
    });

    return result;
}

pub fn create_post(Json(payload): Json<NewPost>) -> Result<Post, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let new_post = NewPost { 
        title : payload.title, 
        body : payload.body, 
        published: Some(payload.published.unwrap_or(false)),
        user_id: payload.user_id
    };

    let result: Result<Post, Error> = connection.transaction(|connection| {
        diesel::insert_into(posts::table)
            .values(&new_post)
            .execute(connection)?;

            let post = posts::table
            .order(posts::id.desc())
            .select(Post::as_select())
            .first(connection)?;

            Ok(post)
    }); 

    return result;
}

pub fn update_post(Json(payload): Json<UpdatePost>) -> Result<Post, diesel::result::Error> {
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

    return result;

}

pub fn delete_post(Path(other_id): Path<i32>) -> Result<Post, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<Post, Error> = connection.transaction(|connection| {

        let post = posts::table
        .find(&other_id)
        .select(Post::as_select())
        .get_result(connection)?;

        diesel::delete(posts::table)
        .filter(id.eq(&other_id))
        .execute(connection)?;
        
        Ok(post)
    });

    return result;
}