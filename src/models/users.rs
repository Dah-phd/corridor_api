use super::schema::users;

#[derive(Queryable)]
pub struct Users {
    pub user: String,
    pub password: String,
    pub email: String,
    pub active: bool,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub user: String,
    pub password: String,
    pub email: String,
}

impl NewUser {
    pub fn new(user: String, password: String, email: String) -> Self {
        Self { user, password, email }
    }
}
