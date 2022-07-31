use super::schema;
use super::schema::users;
use super::DBLink;
use diesel;
use diesel::prelude::*;
use pwhash;
use rocket;
use rocket::serde::{Deserialize, Serialize};

const UNSUPPORTED_CHARS: &str = "|*#;+/\\$%@! ~=<>";

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub enum UserResult<T> {
    Ok(T),
    PlayerExists,
    NameTooShort,
    UnsupportedSymbol,
    UnsupportedSymbolInPass,
    UserNotFound,
    PassTooShort,
    EmailTaken,
}

fn default_player_status() -> bool {
    true
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub user: String,
    pub password: String,
    pub email: String,
    #[serde(default = "default_player_status")]
    pub active: bool,
}

impl User {
    pub fn authenticate(db: &DBLink, username: &String, pass: &String) -> Option<Self> {
        let db = db.mutex_db.lock().unwrap();
        let conn = &*db;
        use schema::users::dsl::*;
        let user_profile: QueryResult<User> = users.filter(user.eq(username.to_owned())).get_result(conn);
        if let Ok(user_object) = user_profile {
            if user_object.verify(pass) {
                return Some(user_object);
            }
        };
        None
    }

    pub fn save(self, db: &DBLink) -> QueryResult<usize> {
        let db = db.mutex_db.lock().unwrap();
        let conn = &*db;
        diesel::insert_into(schema::users::table)
            .values(self.hash_password())
            .execute(conn)
    }

    fn verify(&self, password: &str) -> bool {
        pwhash::bcrypt::verify(password, &self.password)
    }

    fn hash_password(mut self) -> Self {
        let hashed_pass = pwhash::bcrypt::hash(&self.password).unwrap();
        self.password = hashed_pass;
        self
    }

    pub fn is_active(db: &DBLink, username: &String) -> bool {
        let result = Self::get_user_by_username(db, username);
        result.is_ok() && result.unwrap().active
    }

    pub fn is_username_valid(db: &DBLink, username: &String) -> Option<UserResult<String>> {
        if let Some(naming_err) = Self::get_nameing_errors(username) {
            Some(naming_err)
        } else if Self::get_user_by_username(db, username).is_ok() {
            Some(UserResult::PlayerExists)
        } else {
            None
        }
    }

    pub fn is_password_valid(password: &String) -> Option<UserResult<String>> {
        if password.len() < 8 {
            return Some(UserResult::PassTooShort);
        }
        for char in UNSUPPORTED_CHARS.chars() {
            if password.contains(char) {
                return Some(UserResult::UnsupportedSymbolInPass);
            }
        }
        None
    }

    pub fn is_email_valid(db: &DBLink, email: &String) -> Option<UserResult<String>> {
        if Self::get_user_by_email(db, email).is_ok() {
            return Some(UserResult::EmailTaken);
        }
        None
    }

    fn get_user_by_username(db: &DBLink, username: &String) -> QueryResult<User> {
        use schema::users::dsl::*;
        let conn = &*db.mutex_db.lock().unwrap();
        users.filter(user.eq(username.to_owned())).get_result(conn)
    }

    fn get_user_by_email(db: &DBLink, e_mail: &String) -> QueryResult<User> {
        use schema::users::dsl::*;
        let conn = &*db.mutex_db.lock().unwrap();
        users.filter(email.eq(e_mail.to_owned())).get_result(conn)
    }

    fn get_nameing_errors(username: &String) -> Option<UserResult<String>> {
        if username.len() < 4 {
            return Some(UserResult::NameTooShort);
        }
        for char in UNSUPPORTED_CHARS.chars() {
            if username.contains(char) {
                return Some(UserResult::UnsupportedSymbol);
            }
        }
        None
    }
}
