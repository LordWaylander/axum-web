use crate::schema::users::dsl::*;
use crate::schema::users;
use crate::models::users::{NewUser, UpdateUser, User};
use crate::models::posts::Post;

use crate::database;

use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use diesel::result::Error;
use axum::{
    Json,
    extract::Path
};

pub fn get_all_users() -> Result<Vec<(User, Vec<Post>)>, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<Vec<(User, Vec<Post>)>, Error> = connection.transaction(|connection| {

        let u = users.order(users::id.desc()).select(User::as_select()).load::<User>(connection)?;
        let p = Post::belonging_to(&u)
        .load::<Post>(connection)?
        .grouped_by(&u);

        let data = u.into_iter().zip(p).collect::<Vec<_>>();

        Ok(data)
    });

    return result;
}

pub fn get_one_user(Path(other_id): Path<i32>) -> Result<Vec<(User, Vec<Post>)>, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<Vec<(User, Vec<Post>)>, Error> = connection.transaction(|connection| {

        let u = users.find(other_id).select(User::as_select()).load::<User>(connection)?;
        let p = Post::belonging_to(&u)
            .load::<Post>(connection)?
            .grouped_by(&u);

        let data = u.into_iter().zip(p).collect::<Vec<_>>();

        Ok(data)
    });

    return result;
}

pub fn create_user(Json(payload): Json<NewUser>) -> Result<User, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<User, Error> = connection.transaction(|connection| {

        let hashed_password = hash_password(payload.password);

        let new_user = NewUser { 
            username : payload.username, 
            email : payload.email, 
            password: hashed_password,
        };

        diesel::insert_into(users)
            .values(&new_user)
            .execute(connection)?;

            //get_one_user
            let user = users
            .order(users::id.desc())
            .select(User::as_select())
            .get_result(connection)?;

            Ok(user)
    }); 

    return result;
}

pub fn update_user(Path(other_id): Path<i32>, Json(payload): Json<UpdateUser>) -> Result<User, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<User, Error> = connection.transaction(|connection| {

        let hashed_password = payload.password.map(|pwd| hash_password(pwd));

        let update_user = UpdateUser {
            username: payload.username,
            email: payload.email,
            password: hashed_password,
        };

        diesel::update(users.find(other_id))
            .set(&update_user)
            .execute(connection)?;

            let user = users
            .find(other_id)
            .select(User::as_select())
            .get_result(connection)?;

        Ok(user)
    });

    return result;

}

pub fn delete_user(Path(other_id): Path<i32>) -> Result<User, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<User, Error> = connection.transaction(|connection| {

        //get_one_user
        let user = users
        .find(&other_id)
        .select(User::as_select())
        .get_result(connection)?;

        diesel::delete(users)
        .filter(users::id.eq(&other_id))
        .execute(connection)?;
        
        Ok(user)
    });

    return result;
}

//rien a faire ici -> handlers
fn hash_password(pwd: String) -> String {
    hash(pwd, DEFAULT_COST).expect("Error hashing password")
}