use super::*;
use models::*;
use diesel::prelude::*;

use schema::posts;

#[derive(Insertable)]
#[table_name="posts"]
struct InsertablePost {
    title: String,
    content: String,
    user_id: u32,
    category_id: u32,
}

pub fn add(title: String, content: String, user_id: u32, category_id: u32) -> Result<(), error::Error> {
    let connection = establish_connection();
    let new_post = InsertablePost{title, content, user_id, category_id};
    let r = diesel::insert_into(posts::dsl::posts)
        .values(&new_post)
        .execute(&connection);
    match r {
        Ok(c) => {
            if c == 1 {
                Ok(())
            } else {
                Err(error::Error::InsertNumError)
            }
        },
        Err(e) => {
            if let diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::ForeignKeyViolation, _) = e {
                let msg = if e.to_string().contains("user_id") {
                    String::from("user not exists")
                } else if e.to_string().contains("category_id") {
                    String::from("category not exists")
                } else {
                    e.to_string()
                };
                Err(error::Error::ForeignKeyViolation(msg))
            } else {
                Err(error::Error::WapperError(e.to_string()))
            }
        }
    }
}