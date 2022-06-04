use super::schema::users;
use pwhash;
use rocket;
use rocket::serde::{Deserialize, Serialize};

fn default_player_status() -> bool {
    true
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Insertable)]
#[table_name = "users"]
pub struct UserEntry {
    pub user: String,
    pub password: String,
    pub email: String,
    #[serde(default = "default_player_status")]
    pub active: bool,
}

impl UserEntry {
    pub fn verify(&self, password: &str) -> bool {
        pwhash::bcrypt::verify(password, &self.password)
    }

    pub fn hash_password(mut self) -> Self {
        let hashed_pass = pwhash::bcrypt::hash(&self.password).unwrap();
        self.password = hashed_pass;
        self
    }
}
