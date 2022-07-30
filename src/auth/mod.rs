mod auth;
use super::models;
pub use auth::{AuthTokenServices, Token, TOKEN_ID};
use models::{UserModel, UserResult};
use rocket;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
extern crate time;
use time::{Duration, OffsetDateTime};

const UNAUTHORIZED: Json<UserResult<String>> = Json(UserResult::UserNotFound);

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub enum User {
    User(String, String),
    Guest(String),
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
            if let Some(err) = UserModel::is_username_valid(db, &username) {
                return Json(err);
            }
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
    if let Some(err) = UserModel::is_username_valid(db, &username) {
        return Json(err);
    }
    if let Some(err) = UserModel::is_password_valid(&new_user.password) {
        return Json(err);
    }
    if UserModel::new_user(db, new_user.into_inner()).is_ok() {
        let token = Token::new(username.to_owned());
        set_token(token.encode(&token_services.header), cookie_jar);
        return Json(UserResult::Ok(username));
    }
    UNAUTHORIZED
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
