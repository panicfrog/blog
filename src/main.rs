#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;

fn main() {
    let r = db::post::add(String::from("自定义标题"), String::from("夜，结束了一天的喧嚣后安静下来，伴随着远处路灯那微弱的光。风，毫无预兆地席卷整片旷野，撩动人的思绪万千。星，遥遥地挂在天空之中，闪烁着它那微微星光，不如阳光般灿烂却如花儿般如痴如醉"), 100001, 100000);
    r.map_err(|e| {
       if let db::error::Error::ForeignKeyViolation(s) = e {
           println!("{}", s);
       }
    });
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
