use crate::schema::medias;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, PartialEq, Debug, Selectable)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[derive(Serialize, Deserialize)]
pub struct Media {
    pub id: i32,
    pub file_name: String,
    pub url: String,
    pub path: String
}

#[derive(Insertable)]
#[diesel(table_name = medias)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[derive(Deserialize)]
pub struct NewMedia {
    pub file_name: String,
    pub url: String,
    pub path: String
}

#[derive(AsChangeset)]
#[diesel(table_name = medias)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[derive(Serialize, Deserialize)]
pub struct UpdateMedia {
    pub id: i32,
    pub file_name: String,
    pub url: String,
    pub path: String
}