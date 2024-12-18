
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL").unwrap();
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}