use rocket;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub enum ChatID {
    MatchID(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    pub id: ChatID,
    pub msg: String,
    pub player: String,
}
