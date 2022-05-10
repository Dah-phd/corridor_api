mod auth;
use super::models;
pub use auth::Token;
use diesel;

pub fn register(user: String, password: String, email: String) {
    diesel::insert_into(models::schema::users::table).values(models::users::NewUser::new(user, password, email));
}
