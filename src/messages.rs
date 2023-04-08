use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use crate::quoridor::QuoridorMatch;

#[derive(Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct GuestLogin {
    pub username: String,
}

#[derive(Deserialize)]
pub struct UserCreate {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct UserContext {
    pub email: String,
    pub username: String,
    pub auth_token: String,
    pub active_match: Option<String>,
}

impl IntoResponse for UserContext {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, self).into_response()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ChatMessage {
    Message {
        user: String,
        message: String,
        timestamp: i64,
    },
    MessageError(String),
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
    Disallowed,
    GameFinished,
}

#[derive(Serialize, Clone)]
pub struct QuoridorMatchMeta {
    id: String,
    up_player: String,
    down_player: String,
}

impl From<(String, QuoridorMatch)> for QuoridorMatchMeta {
    fn from(value: (String, QuoridorMatch)) -> Self {
        Self {
            id: value.0,
            up_player: value.1.up_player,
            down_player: value.1.down_player,
        }
    }
}