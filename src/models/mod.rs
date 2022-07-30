pub mod schema;
pub mod users;
use diesel;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::sync::Mutex;
use users::UserEntry;

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
}

pub struct DBLink {
    pub mutex_db: Mutex<diesel::sqlite::SqliteConnection>,
}

impl DBLink {
    pub fn new(path: &str) -> Self {
        Self {
            mutex_db: Mutex::new(diesel::sqlite::SqliteConnection::establish(path).expect("No DB")),
        }
    }
}

pub struct UserModel {}

impl UserModel {
    pub fn new_user(db: &DBLink, new_user: UserEntry) -> QueryResult<usize> {
        let db = db.mutex_db.lock().unwrap();
        let conn = &*db;
        diesel::insert_into(schema::users::table)
            .values(new_user.hash_password())
            .execute(conn)
    }

    pub fn authenticate(db: &DBLink, username: &String, pass: &String) -> bool {
        let db = db.mutex_db.lock().unwrap();
        let conn = &*db;
        use schema::users::dsl::*;
        let user_profile: QueryResult<UserEntry> = users.filter(user.eq(username.to_owned())).get_result(conn);
        if user_profile.is_err() {
            return false;
        }
        user_profile.unwrap().verify(pass)
    }

    pub fn is_active(db: &DBLink, username: &String) -> bool {
        let result = Self::get_user_object(db, username);
        result.is_ok() && result.unwrap().active
    }

    pub fn is_username_free(db: &DBLink, username: &String) -> Option<UserResult<String>> {
        let nameing_error = Self::get_nameing_errors(username);
        if nameing_error.is_some() {
            nameing_error
        } else if Self::get_user_object(db, username).is_ok() {
            Some(UserResult::PlayerExists)
        } else {
            None
        }
    }

    pub fn is_password_effective(password: &String) -> Option<UserResult<String>> {
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

    fn get_user_object(db: &DBLink, username: &String) -> QueryResult<UserEntry> {
        use schema::users::dsl::*;
        let conn = &*db.mutex_db.lock().unwrap();
        users.filter(user.eq(username.to_owned())).get_result(conn)
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
