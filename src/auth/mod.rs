mod auth;
use super::models;
pub use auth::Token;
use diesel;
use diesel::prelude::*;
#[macro_use]
use rocket;

#[derive(rocket::serde::Serialize, rocket::serde::Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    user: String,
    password: String,
}

#[post("/auth/login", data = "<user>")]
pub fn login(user: rocket::serde::json::Json<User>) {}

#[post("/auth/register", data = "<new_user>")]
pub fn register(
    new_user: rocket::serde::json::Json<models::users::NewUser>,
    db: &rocket::State<models::DBLink>,
) -> rocket::serde::json::Json<String> {
    let new_user_data = new_user.into_inner();
    fn insert_into_db(conn: &diesel::SqliteConnection, payload: &models::users::NewUser) -> diesel::QueryResult<usize> {
        diesel::insert_into(models::schema::users::table)
            .values(payload)
            .execute(conn)
    }

    db.run_callback(insert_into_db, &new_user_data);

    let token = Token::new(new_user_data.user).encode();
    rocket::serde::json::Json(token)
}
