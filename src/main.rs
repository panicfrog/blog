#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate serde;

mod db;
mod api;

fn main() {
    api::run();
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
