pub mod schema;
pub mod users;
use chrono;
use diesel;
use diesel::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;
use users::UserEntry;

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
        use schema::users::dsl::*;
        let conn = &*db.mutex_db.lock().unwrap();
        let result: QueryResult<UserEntry> = users.filter(user.eq(username.to_owned())).get_result(conn);
        result.is_ok() && result.unwrap().active
    }
}
