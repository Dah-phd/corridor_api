use super::schema;
use super::schema::users;
use super::DBLink;
use diesel;
use diesel::prelude::*;
use pwhash;
use rocket;
use rocket::serde::{Deserialize, Serialize};
extern crate regex;
use regex::Regex;

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
    EmailNotValid,
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
        let conn = &*db.mutex_db.lock().unwrap();
        use schema::users::dsl::*;
        if let Ok(user_object) = users.filter(user.eq(username.to_owned())).get_result::<User>(conn) {
            if user_object.verify_pass(pass) {
                return Some(user_object);
            }
        };
        None
    }

    pub fn save(self, db: &DBLink) -> QueryResult<usize> {
        diesel::insert_into(schema::users::table)
            .values(self.setup_for_save())
            .execute(&*db.mutex_db.lock().unwrap())
    }

    pub fn change_email(&self, db: &DBLink) -> QueryResult<usize> {
        use schema::users::dsl::*;
        let target = users.filter(user.eq(&self.user));
        diesel::update(target)
            .set(email.eq(&self.email))
            .execute(&*db.mutex_db.lock().unwrap())
    }

    pub fn change_password(self, db: &DBLink) -> QueryResult<usize> {
        let new_user = self.setup_for_save();
        use schema::users::dsl::*;
        let target = users.filter(user.eq(&new_user.user));
        diesel::update(target)
            .set(password.eq(&new_user.password))
            .execute(&*db.mutex_db.lock().unwrap())
    }

    fn verify_pass(&self, pass: &str) -> bool {
        pwhash::bcrypt::verify(pass, &self.password)
    }

    fn setup_for_save(mut self) -> Self {
        let hashed_pass = pwhash::bcrypt::hash(&self.password).unwrap();
        self.password = hashed_pass;
        self
    }

    pub fn is_active(db: &DBLink, username: &String) -> bool {
        let result = Self::get_user_by_username(db, username);
        result.is_ok() && result.unwrap().active
    }

    pub fn maybe_naming_err(db: &DBLink, username: &String) -> Option<UserResult<String>> {
        if let Some(naming_err) = Self::get_naming_errors(username) {
            Some(naming_err)
        } else if Self::get_user_by_username(db, username).is_ok() {
            Some(UserResult::PlayerExists)
        } else {
            None
        }
    }

    pub fn maybe_pass_err(password: &String) -> Option<UserResult<String>> {
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

    pub fn maybe_email_err(db: &DBLink, email: &String) -> Option<UserResult<String>> {
        if let Ok(regex) = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})") {
            if !regex.is_match(email) {
                return Some(UserResult::EmailNotValid);
            }
        }
        if Self::get_user_by_email(db, email).is_ok() {
            return Some(UserResult::EmailTaken);
        }
        None
    }

    pub fn get_user_by_username(db: &DBLink, username: &String) -> QueryResult<User> {
        use schema::users::dsl::*;
        let conn = &*db.mutex_db.lock().unwrap();
        users.filter(user.eq(username.to_owned())).get_result(conn)
    }

    fn get_user_by_email(db: &DBLink, e_mail: &String) -> QueryResult<User> {
        use schema::users::dsl::*;
        let conn = &*db.mutex_db.lock().unwrap();
        users.filter(email.eq(e_mail.to_owned())).get_result(conn)
    }

    fn get_naming_errors(username: &String) -> Option<UserResult<String>> {
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
