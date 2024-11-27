use crate::schema::users::dsl::*;
use crate::schema::{posts, users};
use crate::models::users::{NewUser, UpdateUser, User, UserLogin};
use crate::models::posts::Post;

use crate::database;

use diesel::prelude::*;
use diesel::result::Error;

pub fn get_user_by_email(other_email: String) -> Result<Option<UserLogin>, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result = connection.transaction(|connection| {
        let user = users.filter(users::email.eq(other_email)).select(UserLogin::as_select()).first::<UserLogin>(connection).optional();
        user
    });

    return result;
}

pub fn get_all_users() -> Result<Vec<(User, Vec<Post>)>, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<Vec<(User, Vec<Post>)>, Error> = connection.transaction(|connection| {

        let u = users.order(users::id.desc()).select(User::as_select()).load::<User>(connection)?;
        let p = Post::belonging_to(&u)
        .load::<Post>(connection)?
        .grouped_by(&u);

        let data: Vec<(User, Vec<Post>)> = u.into_iter().zip(p).collect::<Vec<_>>();

        Ok(data)
    });

    return result;
}

pub fn get_one_user(other_id: i32) -> Result<(User, Vec<Post>), diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<(User, Vec<Post>), Error> = connection.transaction(|connection| {

        let u: User = users.find(other_id).select(User::as_select()).first::<User>(connection).unwrap();
        let p = posts::table.filter(posts::user_id.eq(other_id)).load::<Post>(connection)?;

        let data: (User, Vec<Post>) = (
            u,
            p
        );

        Ok(data)
    });

    return result;
}

pub fn create_user(payload: NewUser) -> Result<User, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<User, Error> = connection.transaction(|connection| {

        diesel::insert_into(users)
            .values(&payload)
            .execute(connection)?;

            let user = users
            .order(users::id.desc())
            .select(User::as_select())
            .get_result(connection)?;

            Ok(user)
    }); 

    return result;
}

pub fn update_user(other_id: i32, payload: UpdateUser) -> Result<User, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<User, Error> = connection.transaction(|connection| {

        diesel::update(users.find(other_id))
            .set(&payload)
            .execute(connection)?;

            let user = users
            .find(other_id)
            .select(User::as_select())
            .get_result(connection)?;

        Ok(user)
    });

    return result;

}

pub fn delete_user(other_id: i32) -> Result<User, diesel::result::Error> {
    let connection = &mut database::establish_connection();

    let result: Result<User, Error> = connection.transaction(|connection| {

        /*
        * Delete la relation avec le post
        */
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
