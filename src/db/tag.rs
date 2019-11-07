use super::*;
use models::*;
use diesel::prelude::*;

use schema::tags;
use crate::db::models::Tag;

#[derive(Insertable)]
#[table_name="tags"]
struct InsertableTag {
    name: String,
}

pub fn add(ts: Vec<String>) -> Result<(), error::Error> {
    let connection = establish_connection();
    let new_tags: Vec<InsertableTag> = ts.into_iter()
        .map(|t| InsertableTag{ name: t} ).collect();
    let r = diesel::insert_into(tags::dsl::tags)
        .values(&new_tags)
        .execute(&connection);
    match r {
        Ok(c) => {
            if c == new_tags.len() {
                Ok(())
            } else {
                Err(error::Error::InsertNumError)
            }
        } ,
        Err(e) => {
            if let diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) =  e {
                Ok(())
            } else {
                Err(error::Error::WapperError(e.to_string()))
            }
        }
    }
}

pub fn getAll(){
    let connection = establish_connection();
    let r = tags::dsl::tags.filter(tags::dsl::name.ne(String::from("")))
        .load::<Tag>(&connection);
    match r {
        Ok(c) => {
          println!("{:?}", c);
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
}