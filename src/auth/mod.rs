mod auth;
use super::models;
pub use auth::{AuthTokenServices, Token, TOKEN_ID};
use diesel;
use models::UserModel;
use rocket;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
extern crate time;
use time::{Duration, OffsetDateTime};

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
    cookie_jar: &CookieJar<'_>,
    token_services: &rocket::State<auth::AuthTokenServices>,
) -> Json<String> {
    token.refresh();
    let username = token.user.to_owned();
    set_token(token.encode(&token_services.header), cookie_jar);
    return Json(username);
}

#[post("/auth/login", data = "<user>")]
pub fn login(
    user: Json<User>,
    db: &rocket::State<models::DBLink>,
    cookie_jar: &CookieJar<'_>,
    token_services: &rocket::State<auth::AuthTokenServices>,
) -> Json<UserResult<String>> {
    match user.into_inner() {
        User::User(username, password) => {
            if UserModel::authenticate(db, &username, &password) {
                set_token(Token::new(username.to_owned()).encode(&token_services.header), cookie_jar);
                return Json(UserResult::Ok(username));
            }
        }
        User::Guest(username) => {
            let mut token = Token::new(username.to_owned());
            token.set_time();
            set_token(token.encode(&token_services.header), cookie_jar);
            return Json(UserResult::Ok(username));
        }
    }
    UNAUTHORIZED
}

#[post("/auth/register", data = "<new_user>")]
pub fn register(
    new_user: Json<models::users::UserEntry>,
    db: &rocket::State<models::DBLink>,
    cookie_jar: &CookieJar<'_>,
    token_services: &rocket::State<auth::AuthTokenServices>,
) -> Json<UserResult<String>> {
    let username = new_user.user.to_owned();

    let err: Option<UserResult<String>> = UserResult::derive_naming_error(&username);
    if err.is_some() {
        return Json(err.unwrap());
    }

    match UserModel::new_user(db, new_user.into_inner()) {
        diesel::QueryResult::Ok(_) => {
            let token = Token::new(username.to_owned());
            set_token(token.encode(&token_services.header), cookie_jar);
            Json(UserResult::Ok(username))
        }
        _ => UNAUTHORIZED,
    }
}

#[get("/auth/logout")]
pub fn logout(cookie_jar: &CookieJar<'_>) {
    if let Some(cookie) = cookie_jar.get(TOKEN_ID) {
        cookie_jar.remove(cookie.clone());
    }
}

fn set_token(cookie_val: String, cookie_jar: &CookieJar<'_>) {
    let mut cookie = Cookie::new(TOKEN_ID, cookie_val);
    cookie.set_expires(OffsetDateTime::now_utc() + Duration::weeks(4));
    cookie_jar.add(cookie);
}
