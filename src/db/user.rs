use super::*;
use models::*;
use diesel::prelude::*;

use schema::users::dsl::*;
use schema::users;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use crate::db::models::User;

#[derive(Insertable)]
#[table_name="users"]
struct InsertableUser {
    pub user_name: String,
    pub passwd: String,
}

pub fn add(un: String, pd: String) -> Result<(), error::Error> {
    let connection = establish_connection();
    let new_u = InsertableUser{user_name: un , passwd: pd};
    let r = diesel::insert_into(users)
        .values(&new_u)
        .execute(&connection);
    match r {
        Ok(s) => {
            if s == 1 {
                Ok(())
            } else {
                Err(error::Error::InsertNumError)
            }
        },
        Err(e) => {
            if let diesel::result::Error::DatabaseError(UniqueViolation, _) = e {
                Err(error::Error::DuplicateData(e.to_string()))
            } else {
                Err(error::Error::WapperError(e.to_string()))
            }
        }
    }
}

pub fn find(un: String) -> Result<User, error::Error> {
    let connection = establish_connection();
    let r: QueryResult<User> = users::dsl::users.filter(users::dsl::user_name.eq(un))
        .first(&connection);
    match r {
        Ok(u) => Ok(u),
        Err(e) => {
            if let diesel::NotFound = e {
                Err(error::Error::NotFound)
            } else {
                Err(error::Error::WapperError(e.to_string()))
            }
        }
    }
}

pub fn change_passwd(un: String, pw: String) -> Result<(), error::Error> {
    let u = find(un);
    match u {
        Ok(_u) => {
            if pw == _u.passwd {
                Ok(())
            } else {
                let connection = establish_connection();
                let r = diesel::update(users::dsl::users.find(_u.user_id))
                    .set(passwd.eq(pw))
                    .execute(&connection);
                match r {
                    Ok(s) => {
                        if s == 1 {
                            Ok(())
                        } else {
                            Err(error::Error::NotFound)
                        }
                    },
                    Err(e) => {
                        if let diesel::NotFound = e {
                            Err(error::Error::NotFound)
                        } else {
                            Err(error::Error::WapperError(e.to_string()))
                        }
                    }
                }
            }
        },
        Err(e) => Err(e)
    }

}