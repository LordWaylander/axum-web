use crate::models::users::{NewUser, User, UpdateUser};
use bcrypt::{hash, DEFAULT_COST};
use crate::schema::users::dsl::{users, id, username, email };
use crate::database;
use diesel::prelude::*;
use diesel::result::Error;
use axum::{
    Json,
    extract::Path
};

pub fn get_all_users() -> Result<Vec<User>, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<Vec<User>, Error> = connection.transaction(|connection| {
        let users_vector = users
        .order(id.asc())
        //.filter(published.eq(true))
        .limit(5)
        .select(User::as_select())
        .get_results(connection)?;

        Ok(users_vector)
    });

    return result;
}

pub fn get_one_user(Path(other_id): Path<i32>) -> Result<User, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<User, Error> = connection.transaction(|connection| {
            let user = users
            .find(other_id)
            .select(User::as_select())
            .first(connection)?;

        Ok(user)
    });

    return result;
}

pub fn create_user(Json(payload): Json<NewUser>) -> Result<User, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<User, Error> = connection.transaction(|connection| {

        let hashed_password = hash_password(payload.password);

        let new_post = NewUser { 
            username : payload.username, 
            email : payload.email, 
            password: hashed_password,
        };

        diesel::insert_into(users)
            .values(&new_post)
            .execute(connection)?;

            let post = users
            .order(id.desc())
            .select(User::as_select())
            .first(connection)?;

            Ok(post)
    }); 

    return result;
}

pub fn update_user(Json(payload): Json<UpdateUser>) -> Result<User, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<User, Error> = connection.transaction(|connection| {

        let hashed_password = payload.password.map(|pwd| hash_password(pwd));

        let update_post = UpdateUser {
            id: payload.id,
            username: payload.username,
            email: payload.email,
            password: hashed_password,
        };

        diesel::update(users.find(payload.id))
            .set(&update_post)
            .execute(connection)?;

            let post = users
            .find(payload.id)
            .select(User::as_select())
            .first(connection)?;

        Ok(post)
    });

    return result;

}

pub fn delete_user(Path(other_id): Path<i32>) -> Result<User, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<User, Error> = connection.transaction(|connection| {

        let post = users
        .find(&other_id)
        .select(User::as_select())
        .get_result(connection)?;

        diesel::delete(users)
        .filter(id.eq(&other_id))
        .execute(connection)?;
        
        Ok(post)
    });

    return result;
}

fn hash_password(pwd: String) -> String {
    hash(pwd, DEFAULT_COST).expect("Error hashing password")
}