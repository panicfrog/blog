use super::*;
use models::*;
use diesel::prelude::*;

use schema::categorys;
use crate::db::models::Category;

#[derive(Insertable)]
#[table_name="categorys"]
struct InsertableCategory {
    name: String,
}

pub fn add(cs: Vec<String>) -> Result<(), error::Error> {
    let connection = establish_connection();
    let new_categorys: Vec<InsertableCategory> = cs.into_iter()
        .map(|c| InsertableCategory{name: c})
        .collect();
    let r = diesel::insert_into(categorys::dsl::categorys)
        .values(&new_categorys)
        .execute(&connection);
    match r {
        Ok(c ) => {
            if c == new_categorys.len() {
                Ok(())
            } else {
                Err(error::Error::InsertNumError)
            }
        },
        Err(e) => {
            if let diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) = e {
                Ok(())
            } else {
                Err(error::Error::WapperError(e.to_string()))
            }
        }
    }
}

pub fn get_all() -> Result<Vec<Category>, error::Error> {
    let connection = establish_connection();
    let r = categorys::dsl::categorys.filter(categorys::dsl::name.ne(String::from("")))
        .load::<Category>(&connection);
    match r {
        Ok(c) => Ok(c),
        Err(e) => Err(error::Error::WapperError(e.to_string()))
    }
}
