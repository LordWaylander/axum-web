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
        /*let users_vector = users
        .order(users::id.asc())
        .inner_join(posts::table.on(posts::id.eq(posts::user_id)))
        .limit(5)
        .select((PublicUser::as_select(), Post::as_select()))
        .get_results::<(PublicUser, Post)>(connection)?;*/

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

        // juste pour enlever le password de la réponse
        //let data = u.into_iter().map(|User {id: other_id, username : other_username, email : other_email}| PublicUser { id : other_id, username : other_username, email  : other_email }).zip(p).collect::<Vec<_>>();
        let data = u.into_iter().zip(p).collect::<Vec<_>>();

        Ok(data)
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
            .order(users::id.desc())
            .select(User::as_select())
            .get_result(connection)?;

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
            .get_result(connection)?;

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
        .filter(users::id.eq(&other_id))
        .execute(connection)?;
        
        Ok(post)
    });

    return result;
}

fn hash_password(pwd: String) -> String {
    hash(pwd, DEFAULT_COST).expect("Error hashing password")
}