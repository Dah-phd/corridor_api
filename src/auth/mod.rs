mod auth;
use super::models;
pub use auth::Token;
use diesel;
use diesel::prelude::*;
#[macro_use]
use rocket;

#[derive(rocket::serde::Serialize, rocket::serde::Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum User {
    User(String, String),
    Guest(String),
}

#[post("/auth/login", data = "<user>")]
pub fn login(
    user: rocket::serde::json::Json<User>,
    db: &rocket::State<models::DBLink>,
    users: &rocket::State<models::UserModel>,
) -> rocket::serde::json::Json<Option<String>> {
    match user.into_inner() {
        User::User(username, password) => {
            if users.authenticate(db, &username, &password) {
                return rocket::serde::json::Json(Option::Some(Token::new(username).encode()));
            }
        }
        User::Guest(username) => if users.new_guest(username) {},
    }
    rocket::serde::json::Json(None)
}

#[post("/auth/register", data = "<new_user>")]
pub fn register(
    new_user: rocket::serde::json::Json<models::users::User>,
    db: &rocket::State<models::DBLink>,
    users: &rocket::State<models::UserModel>,
) -> rocket::serde::json::Json<Option<String>> {
    let new_user_data = new_user.into_inner().hash_password();
    let mut db = db.mutex_db.lock().unwrap();
    let conn = &*db;
    let writing_result = diesel::insert_into(models::schema::users::table)
        .values(&new_user_data)
        .execute(conn);

    match writing_result {
        diesel::QueryResult::Ok(_) => {
            let token = Token::new(new_user_data.user).encode();
            rocket::serde::json::Json(Some(token))
        }
        _ => rocket::serde::json::Json(None),
    }
}
