mod auth;
use super::models;
pub use auth::{AuthTokenServices, Token};
use diesel;
use models::UserModel;
use rocket;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

const UNAUTHORIZED: Json<UserResult<String>> = Json(UserResult::UserNotFound);
const UNSUPPORTED_CHARS: &str = "|*#;+/\\$%@! ~=<>";

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub enum User {
    User(String, String),
    Guest(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub enum UserResult<T> {
    Ok(T),
    PlayerExists,
    NameTooShort,
    UnsupportedSymbol,
    UserNotFound,
}

impl<T> UserResult<T> {
    pub fn derive_naming_error(username: &str) -> Option<Self> {
        if username.len() < 4 {
            return Some(Self::NameTooShort);
        }
        for char in UNSUPPORTED_CHARS.chars() {
            if username.contains(char) {
                return Some(Self::UnsupportedSymbol);
            }
        }
        None
    }
}

#[get("/auth/get_username")]
pub fn get_user_name_from_token(
    mut token: auth::Token,
    token_services: &rocket::State<auth::AuthTokenServices>,
) -> Json<(String, String)> {
    token.refresh();
    return Json((token.user.to_owned(), token.encode(&token_services.header)));
}

#[post("/auth/login", data = "<user>")]
pub fn login(
    user: Json<User>,
    db: &rocket::State<models::DBLink>,
    token_services: &rocket::State<auth::AuthTokenServices>,
) -> Json<UserResult<String>> {
    match user.into_inner() {
        User::User(username, password) => {
            if UserModel::authenticate(db, &username, &password) {
                return Json(UserResult::Ok(Token::new(username).encode(&token_services.header)));
            }
        }
        User::Guest(username) => {
            let mut token = Token::new(username);
            token.set_time();
            return Json(UserResult::Ok(token.encode(&token_services.header)));
        }
    }
    UNAUTHORIZED
}

#[post("/auth/register", data = "<new_user>")]
pub fn register(
    new_user: Json<models::users::UserEntry>,
    db: &rocket::State<models::DBLink>,
    token_services: &rocket::State<auth::AuthTokenServices>,
) -> Json<UserResult<String>> {
    let username = new_user.user.to_owned();

    let err: Option<UserResult<String>> = UserResult::derive_naming_error(&username);
    if err.is_some() {
        return Json(err.unwrap());
    }

    match UserModel::new_user(db, new_user.into_inner()) {
        diesel::QueryResult::Ok(_) => {
            let token = Token::new(username).encode(&token_services.header);
            Json(UserResult::Ok(token))
        }
        _ => UNAUTHORIZED,
    }
}

#[catch(403)]
pub fn forbidden(_: &rocket::Request) -> Json<UserResult<String>> {
    UNAUTHORIZED
}
