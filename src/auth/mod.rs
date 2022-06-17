mod auth;
use super::models;
pub use auth::Token;
use diesel;
use models::UserModel;
use rocket;

const UNAUTHORIZED: rocket::serde::json::Json<UserResult<String>> = rocket::serde::json::Json(UserResult::UserNotFound);

#[derive(rocket::serde::Serialize, rocket::serde::Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub enum User {
    User(String, String),
    Guest(String),
}

#[derive(rocket::serde::Serialize, rocket::serde::Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub enum UserResult<T> {
    Ok(T),
    PlayerExists,
    NameTooShort,
    UnsupportedSymbol,
    UserNotFound,
}

impl<T> UserResult<T> {
    pub fn ok_or_default(self, default: T) -> T {
        match self {
            Self::Ok(v) => v,
            _ => default,
        }
    }
    pub fn unwrap(self) -> T {
        match self {
            Self::Ok(v) => return v,
            _ => panic!("No value to unwrap"),
        }
    }
}

fn get_naming_errors<T>(name: &String) -> Option<UserResult<T>> {
    if name_is_using_unsupported_symbols(name) {
        return Some(UserResult::UnsupportedSymbol);
    }
    if name_is_too_short(name) {
        return Some(UserResult::NameTooShort);
    }
    None
}

fn name_is_too_short(name: &String) -> bool {
    name.len() < 4
}

fn name_is_using_unsupported_symbols(name: &String) -> bool {
    let arr = ["|"];
    for symbol in arr {
        if name.contains(symbol) {
            return true;
        }
    }
    false
}

impl User {
    pub fn update_guest_name(self) -> UserResult<Self> {
        match self {
            Self::User(name, pass) => {
                let name_err = get_naming_errors(&name);
                if name_err.is_some() {
                    return name_err.unwrap();
                }
                return UserResult::Ok(Self::User(name, pass));
            }
            Self::Guest(name) => {
                let name_err = get_naming_errors(&name);
                if name_err.is_some() {
                    return name_err.unwrap();
                }
                return UserResult::Ok(Self::Guest(name + "|"));
            }
        }
    }
    pub fn unwrap_username(self) -> String {
        match self {
            Self::User(username, _) => return username,
            Self::Guest(username) => return username,
        }
    }
}

#[get("/auth/get_username")]
pub fn get_user_name_from_token(token: auth::Token) -> rocket::serde::json::Json<String> {
    return rocket::serde::json::Json(token.user);
}

#[get("/auth/new_token")]
pub fn new_token(mut token: auth::Token) -> rocket::serde::json::Json<Option<String>> {
    token.refresh();
    return rocket::serde::json::Json(Some(token.encode()));
}

#[post("/auth/login", data = "<user>")]
pub fn login(
    user: rocket::serde::json::Json<User>,
    db: &rocket::State<models::DBLink>,
) -> rocket::serde::json::Json<Option<String>> {
    match user.into_inner() {
        User::User(username, password) => {
            if UserModel::authenticate(db, &username, &password) {
                return rocket::serde::json::Json(Some(Token::new(username).encode()));
            }
        }
        User::Guest(username) => {
            let mut token = Token::new(username);
            token.set_time();
            return rocket::serde::json::Json(Some(token.encode()));
        }
    }
    rocket::serde::json::Json(None)
}

#[post("/auth/register", data = "<new_user>")]
pub fn register(
    new_user: rocket::serde::json::Json<models::users::UserEntry>,
    db: &rocket::State<models::DBLink>,
) -> rocket::serde::json::Json<UserResult<String>> {
    let user_data = new_user.into_inner();
    let username = user_data.user.to_owned();
    let writing_result = UserModel::new_user(db, user_data);

    match writing_result {
        diesel::QueryResult::Ok(_) => {
            let token = Token::new(username).encode();
            rocket::serde::json::Json(UserResult::Ok(token))
        }
        diesel::QueryResult::Err(_) => UNAUTHORIZED,
    }
}

#[catch(403)]
pub fn forbidden(_: &rocket::Request) -> rocket::serde::json::Json<UserResult<String>> {
    UNAUTHORIZED
}
