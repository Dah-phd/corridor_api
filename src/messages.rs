use rocket;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub enum ChatID {
    RoomID(String),
    MatchID(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Messages {
    pub id: ChatID,
    pub msg: String,
    pub player: String,
}
