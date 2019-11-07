use chrono::NaiveDateTime;
use super::schema::*;
//use serde::{Serialize, Deserialize};

//#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Queryable)]
#[derive(Queryable, Debug)]
pub struct Category {
    category_id: u32,
    name: String,
    create_time: NaiveDateTime,
    update_time: Option<NaiveDateTime>,
    delete_time: Option<NaiveDateTime>,
}

//#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Queryable)]
#[derive(Queryable)]
pub struct Comment {
    comment_id: u32,
    content: String,
    create_time: NaiveDateTime,
    update_time: Option<NaiveDateTime>,
    delete_time: Option<NaiveDateTime>,
}

//#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Queryable)]
#[derive(Queryable, Debug)]
pub struct Tag {
    tag_id: u32,
    name: String,
    create_time: NaiveDateTime,
    update_time: Option<NaiveDateTime>,
    delete_time: Option<NaiveDateTime>,
}

//#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Queryable)]
#[derive(Queryable)]
pub struct Topic {
    topic_id: u32,
    title: String,
    content: String,
    user_id: i32,
    category_id: i32,
    create_time: NaiveDateTime,
    update_time: Option<NaiveDateTime>,
    delete_time: Option<NaiveDateTime>,
}

//#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Queryable)]
#[derive(Queryable)]
pub struct TopicsTag {
    topic_tag_id: u32,
    topic_id: i32,
    tag_id: i32,
    create_time: NaiveDateTime,
    update_time: Option<NaiveDateTime>,
    delete_time: Option<NaiveDateTime>,
}

//#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Queryable)]
#[derive(Queryable, Debug, Eq, PartialEq)]
pub struct User {
    pub user_id: u32,
    pub user_name: String,
    pub passwd: String,
    create_time: NaiveDateTime,
    update_time: Option<NaiveDateTime>,
    delete_time: Option<NaiveDateTime>,
}