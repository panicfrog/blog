use super::*;
use models::*;
use diesel::prelude::*;

use schema::comments;

#[derive(Insertable)]
#[table_name="comments"]
struct InsertableComment {
    content: String,
    post_id: u32,
    sid: Option<u32>,
}

pub fn add(content: String, post_id: u32, sid: Option<u32>) -> Result<(), error::Error> {
    let new_comment = InsertableComment{content, post_id, sid};
    let connection = establish_connection();
    let r = diesel::insert_into(comments::dsl::comments)
        .values(&new_comment)
        .execute(&connection);

    match r {
        Ok(c) => {
            if c == 1 { Ok(()) }
            else { Err(error::Error::InsertNumError) }
        },
        Err(e) => {
            if let diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::ForeignKeyViolation, _) = e {
                let msg = if e.to_string().contains("post_id") {
                    String::from("post not exists")
                } else if e.to_string().contains("sid") {
                    String::from("super comment not exists")
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
