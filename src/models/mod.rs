pub mod schema;
pub mod users;
use diesel;
use diesel::prelude::*;
use std::sync::Mutex;

pub struct DBLink {
    mutex_db: Mutex<diesel::sqlite::SqliteConnection>,
}

impl DBLink {
    pub fn new(path: &str) -> Self {
        Self {
            mutex_db: Mutex::new(diesel::sqlite::SqliteConnection::establish(path).expect("Unable to find DB")),
        }
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
