#[derive(Queryable)]
pub struct Users {
    pub user: String,
    pub password: String,
    pub email: String,
    pub active: bool,
}
