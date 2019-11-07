use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;
pub mod user;
pub mod error;
pub mod tag;
pub mod category;
pub mod comment;
pub mod post;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let databser_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&databser_url)
        .expect(&format!("Error conncting to {}", databser_url))
}