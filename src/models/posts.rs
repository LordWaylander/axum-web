use crate::schema::posts;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[derive(Deserialize)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub published : Option<bool>,
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