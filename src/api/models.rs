use super::super::db::models;

#[derive(Serialize)]
pub struct User {
    pub user_name: String,
    pub user_id: u32
}

impl User {
    pub fn new(m: models::User) -> User {
        User{user_name: m.user_name, user_id: m.user_id}
    }
}