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

pub struct UserModel {
    mutex_guests: Mutex<HashMap<String, i64>>,
}

impl UserModel {
    pub fn new() -> Self {
        Self {
            mutex_guests: Mutex::new(HashMap::new()),
        }
    }

    pub fn new_user(&self, db: &DBLink, new_user: UserEntry) -> QueryResult<usize> {
        let db = db.mutex_db.lock().unwrap();
        let conn = &*db;
        diesel::insert_into(schema::users::table)
            .values(new_user.hash_password())
            .execute(conn)
    }

    pub fn new_guest(&self, user_name: &String) -> bool {
        let mut guests = self.mutex_guests.lock().unwrap();
        let exposed_hash = &mut *guests;
        let now = chrono::Utc::now().timestamp();
        exposed_hash.retain(|_, v| *v > now);
        if !exposed_hash.contains_key(user_name) {
            exposed_hash.insert(user_name.to_owned(), chrono::Utc::now().timestamp() + 1800);
            return true;
        }
        false
    }

    pub fn authenticate(&self, db: &DBLink, username: &String, pass: &String) -> bool {
        let db = db.mutex_db.lock().unwrap();
        let conn = &*db;
        use schema::users::dsl::*;
        let user_profile: QueryResult<UserEntry> = users.filter(user.eq(username.to_owned())).get_result(conn);
        if user_profile.is_err() {
            return false;
        }
        return user_profile.unwrap().verify(pass);
    }

    pub fn user_is_active(&self, db: Option<&DBLink>, user: String) -> bool {
        if let Some(db) = db {
            use schema::users::dsl::*;
            let conn = &*db.mutex_db.lock().unwrap();
            let results: QueryResult<UserEntry> = users.filter(user.eq(user.to_owned())).get_result(conn);
            if results.is_ok() {
                return true;
            }
        }
        let mut guests = self.mutex_guests.lock().unwrap();
        let exposed_hash = &mut *guests;
        let now = chrono::Utc::now().timestamp();
        exposed_hash.retain(|_, v| *v > now);
        if exposed_hash.contains_key(&user) {
            return exposed_hash.insert(user, now + 1800).is_some();
        }
        false
    }
}
