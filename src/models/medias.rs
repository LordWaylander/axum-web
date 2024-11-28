use crate::schema::medias;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, PartialEq, Debug, Selectable)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
#[derive(Serialize, Deserialize)]
pub struct Media {
    pub file_name: String,
    pub url: String
}