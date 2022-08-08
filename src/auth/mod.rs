mod auth;
mod secret_key;
use super::models;
pub use auth::{AuthTokenServices, Token, TOKEN_ID};
use models::users::{User, UserResult};
use rocket;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
extern crate time;
use time::{Duration, OffsetDateTime};

const UNAUTHORIZED: Json<UserResult<String>> = Json(UserResult::UserNotFound);

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub enum UserType {
    User(String, String),
    Guest(String),
}

#[get("/auth/get_username")]
pub fn get_user_name_from_token(
    mut token: Token,
    db: &rocket::State<models::DBLink>,
    cookie_jar: &CookieJar<'_>,
    token_services: &rocket::State<AuthTokenServices>,
) -> Json<UserType> {
    token.refresh();
    let username = token.user.to_owned();
    set_token(token.encode(&token_services.header), cookie_jar);
    if let Ok(user) = User::get_user_by_username(db, &username) {
        return Json(UserType::User(user.user, user.email));
    }
    return Json(UserType::Guest(username));
}

#[post("/auth/login", data = "<user>")]
pub fn login(
    user: Json<UserType>,
    db: &rocket::State<models::DBLink>,
    cookie_jar: &CookieJar<'_>,
    token_services: &rocket::State<AuthTokenServices>,
) -> Json<UserResult<String>> {
    match user.into_inner() {
        UserType::User(username, password) => {
            if let Some(user) = User::authenticate(db, &username, &password) {
                set_token(Token::new(user.user.to_owned()).encode(&token_services.header), cookie_jar);
                return Json(UserResult::Ok(user.user.to_owned()));
            }
        }
        UserType::Guest(username) => {
            if let Some(err) = User::maybe_naming_err(db, &username) {
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
    new_user: Json<User>,
    db: &rocket::State<models::DBLink>,
    cookie_jar: &CookieJar<'_>,
    token_services: &rocket::State<AuthTokenServices>,
) -> Json<UserResult<String>> {
    let username = new_user.user.to_owned();
    if let Some(err) = User::maybe_naming_err(db, &username) {
        return Json(err);
    }
    if let Some(err) = User::maybe_pass_err(&new_user.password) {
        return Json(err);
    }
    if let Some(err) = User::maybe_email_err(db, &new_user.email) {
        return Json(err);
    }
    if new_user.into_inner().save(db).is_ok() {
        let token = Token::new(username.to_owned());
        set_token(token.encode(&token_services.header), cookie_jar);
        return Json(UserResult::Ok(username));
    }
    UNAUTHORIZED
}

#[put("/auth/update_email", data = "<user>")]
pub fn update_email(user: Json<User>, _auth: Token, db: &rocket::State<models::DBLink>) -> Json<UserResult<String>> {
    let email = user.email.to_owned();
    if let Some(err) = User::maybe_email_err(db, &email) {
        return Json(err);
    }
    if let Ok(_) = user.into_inner().change_email(db) {
        return Json(UserResult::Ok(email));
    }
    UNAUTHORIZED
}

#[put("/auth/update_pass", data = "<user>")]
pub fn update_password(user: Json<User>, _auth: Token, db: &rocket::State<models::DBLink>) -> Json<UserResult<String>> {
    if let Some(err) = User::maybe_pass_err(&user.password) {
        return Json(err);
    }
    if let Ok(_) = user.into_inner().change_password(db) {
        return Json(UserResult::Ok("".to_owned()));
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
