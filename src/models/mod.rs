pub mod schema;
pub mod users;
use chrono;
use diesel;
use diesel::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct DBLink {
    mutex_db: Mutex<diesel::sqlite::SqliteConnection>,
    mutex_guests: Mutex<HashMap<String, i64>>,
}

impl DBLink {
    pub fn new(path: &str) -> Self {
        Self {
            mutex_db: Mutex::new(diesel::sqlite::SqliteConnection::establish(path).expect("No DB")),
            mutex_guests: Mutex::new(HashMap::new()),
        }
    }

    pub fn confirm_user(&self, new_user: &String, pass: String) -> bool {
        use schema::users::dsl::*;
        false
    }

    pub fn new_guest(&self, new_user: String) -> bool {
        let mut guests_db = self.mutex_guests.lock().unwrap();
        let exposed_guest_hash = &mut *guests_db;
        let now = chrono::Utc::now().timestamp();
        exposed_guest_hash.retain(|_, v| *v < now);
        if !exposed_guest_hash.contains_key(&new_user) {
            exposed_guest_hash.insert(new_user, chrono::Utc::now().timestamp() + 1800);
            return true;
        }
        false
    }

    pub fn run_callback<X, E>(
        &self,
        callback: fn(&diesel::sqlite::SqliteConnection, X) -> diesel::QueryResult<E>,
        state: X,
    ) -> diesel::QueryResult<E> {
        let mut db = self.mutex_db.lock().unwrap();
        let conn = &mut *db;
        callback(conn, state)
    }
}
