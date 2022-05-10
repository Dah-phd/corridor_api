use super::schema::users;
use rocket;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable)]
pub struct Users {
    pub user: String,
    pub password: String,
    pub email: String,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub user: String,
    pub password: String,
    pub email: String,
}

impl NewUser {
    pub fn new(user: String, password: String, email: String) -> Self {
        Self { user, password, email }
    }
}
