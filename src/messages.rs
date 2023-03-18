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

#[derive(Serialize, Clone)]
pub enum JsonMessage {
    User {
        email: String,
        username: String,
        auth_token: String,
    },
    ChatMessage {
        user: String,
        msg: String,
    },
    LobbyID(String),
    QuoridorID(String),
    Unauthorized,
    NotAnEmail,
    ShouldNotBeEmail,
    EmailAlreadyInUse,
    ServerErrror,
    AlreadyStarted
}