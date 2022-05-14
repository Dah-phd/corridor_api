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
        User::Guest(username) => {
            if users.new_guest(&username) {
                return rocket::serde::json::Json(Option::Some(Token::new(username).encode()));
            }
        }
    }
    rocket::serde::json::Json(None)
}

#[post("/auth/register", data = "<new_user>")]
pub fn register(
    new_user: rocket::serde::json::Json<models::users::User>,
    db: &rocket::State<models::DBLink>,
    users: &rocket::State<models::UserModel>,
) -> rocket::serde::json::Json<Result<String, ()>> {
    let user_data = new_user.into_inner();
    let username = user_data.user.to_owned();
    let writing_result = users.new_user(db, user_data);

    match writing_result {
        diesel::QueryResult::Ok(_) => {
            let token = Token::new(username).encode();
            rocket::serde::json::Json(Ok(token))
        }
        diesel::QueryResult::Err(_) => rocket::serde::json::Json(Err(())),
    }
}

#[put("/err", format = "json")]
pub fn unauthorized() -> rocket::serde::json::Json<Result<String, String>> {
    return rocket::serde::json::Json(Err("unauth".to_owned()));
}

#[catch(403)]
pub fn forbidden(req: &rocket::Request) -> rocket::serde::json::Json<Result<(), String>> {
    rocket::serde::json::Json(Err("unauth".to_owned()))
}
