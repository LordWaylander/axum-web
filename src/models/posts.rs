use crate::schema::posts;
use crate::models::users::User;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, PartialEq, Debug, Selectable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[derive(Deserialize)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub published : Option<bool>,
    pub user_id: i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[derive(Serialize, Deserialize)]
pub struct UpdatePost {
    pub id: i32,
    pub title: Option<String>,
    pub body: Option<String>,
    pub published : Option<bool>,
}