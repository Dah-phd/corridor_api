use super::schema::users;
use pwhash;
use rocket;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub user: String,
    pub password: String,
    pub email: String,
    pub active: bool,
}

impl User {
    pub fn verify(&self, password: &str) -> bool {
        pwhash::bcrypt::verify(password, &self.password)
    }
    pub fn hash_password(mut self) -> Self {
        let hashed_pass = pwhash::bcrypt::hash(&self.password).unwrap();
        self.password = hashed_pass;
        self
    }
}
