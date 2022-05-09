mod auth;
use super::models;
use diesel;

pub fn register(user: String, password: String, email: String) {
    diesel::insert_into(models::schema::users::table).values(models::users::NewUser::new(user, password, email));
}
