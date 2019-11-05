#[macro_use]
extern crate diesel;
extern crate dotenv;
//extern crate chrono;

mod db;

use db::*;
use db::models::*;
use diesel::prelude::*;

fn main() {
    use schema::users::dsl::*;
    let connection = establish_connection();
    let u: User = users.filter(user_name.eq("yeyongping"))
        .first(&connection)
        .expect("查找错误");
    println!("{}", u.user_name);
}
