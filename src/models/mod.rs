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

    pub fn run_callback<T, E>(&self, callback: fn(&diesel::sqlite::SqliteConnection) -> Result<T, E>) -> Result<T, E> {
        let mut db = self.mutex_db.lock().unwrap();
        callback(&mut *db)
    }
}
