#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;

use db::*;
use db::models::*;
use diesel::prelude::*;

fn main() {
//    let r = db::user::add(String::from("xiaoming"), String::from("123456"));
//    r.map_err(|e| {
//       println!("{:?}", e);
//    });
//    let r = db::user::find(String::from("nihaohahaha"));
//    let r = db::user::changePasswd(String::from("yeyongping2"), String::from("123456"));

//    let r = db::tag::add(vec![String::from("rust"), String::from("swift")]);
//    r.map_err(|e| {
//       println!("{:?}", e);
//    });
//
//    tag::getAll();


}

/*
    user
        POST /v1/user/signup  { username: <String>, passwd: <String> }
        POST /v1/user/login   { username <String>, passwd: <String> }
    topic
        GET  /v1/topic/getTopic/:id
        POST /v1/topic/publishTopic { title: <String>, content: <String>, user_id: <String>, category_id: <i32>, tags: [<i32>] }
    tag
        GET  /v1/tag/getAll
        POST /v1/tag/add      { tars: [<String>] }
    comment
        GET  /v1/comment/getComment/:id
        POST /v1/comment/publishComment  { topic_id: <i32>, content: <String> }
*/
