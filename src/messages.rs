use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserCreate {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ChatID {
    MatchID(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: ChatID,
    pub msg: String,
    pub player: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PlayerMove {
    Concede,
    QuoridorWallV { row: usize, col: usize },
    QuoridorWallH { row: usize, col: usize },
    QuoridorMove { row: usize, col: usize },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PlayerMoveResult {
    Ok,
    WrongPlayerTurn,
    Disallowed,
    Unknown,
    GameFinished,
}
impl PlayerMoveResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, Self::Ok)
    }
}

#[derive(Serialize, Clone)]
pub enum JsonMessage {
    User {
        email: String,
        username: String,
        auth_token: String,
    },
    LobbyID(String),
    QuoridorID(String),
    Unauthorized,
    EmailAlreadyInUse,
    ServerErrror,
}
