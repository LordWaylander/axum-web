use crate::models::posts::{NewPost, Post, UpdatePost};
use crate::schema::posts;
use crate::schema::posts::dsl::*;
use crate::database;
use crate::schema::users;
use crate::models::users::User;

use diesel::prelude::*;
use diesel::result::Error;

pub fn get_all_posts() -> Result<Vec<(Post, User)>, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<Vec<(Post, User)>, Error> = connection.transaction(|connection| {
        let posts_vector = posts::table
        .order(posts::id.asc())
        .inner_join(users::table.on(posts::user_id.eq(users::id)))
        .select((Post::as_select(), (User::as_select())))
        .get_results::<(Post, User)>(connection)?;

        Ok(posts_vector)
    });

    return result;
}

pub fn get_one_post(other_id: i32) -> Result<(Post, User), diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<(Post, User), Error> = connection.transaction(|connection| {
            let post = posts::table
            .find(other_id)
            .inner_join(users::table.on(posts::user_id.eq(users::id)))
            //.group_by(users::id)
            .select((Post::as_select(), (User::as_select())))
            .get_result::<(Post, User)>(connection)?;

        Ok(post)
    });

    return result;
}

pub fn create_post(payload: NewPost) -> Result<Post, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<Post, Error> = connection.transaction(|connection| {
        diesel::insert_into(posts::table)
        .values(&payload)
        .execute(connection)?;

        let post = posts::table
        .order(posts::id.desc())
        .select(Post::as_select())
        .get_result(connection)?;

        Ok(post)
    }); 

    return result;
}

pub fn update_post(other_id: i32, payload: UpdatePost) -> Result<Post, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<Post, Error> = connection.transaction(|connection| {

        diesel::update(posts::table.find(other_id))
            .set(&payload)
            .execute(connection)?;

            let post = posts::table
            .find(other_id)
            .select(Post::as_select())
            .get_result(connection)?;

        Ok(post)
    });

    return result;

}

pub fn delete_post(other_id: i32) -> Result<Post, diesel::result::Error> {
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