use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
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

#[derive(Serialize)]
pub struct UserContext {
    pub user: JsonMessage,
    pub active_match: JsonMessage,
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

#[derive(Serialize, Clone)]
pub enum JsonMessage {
    User {
        email: String,
        username: String,
        auth_token: String,
    },
    QuoridorID(String),
    Unauthorized,
    NotFound,
    NotAnEmail,
    ShouldNotBeEmail,
    AlreadyTaken,
    ServerError,
}

impl IntoResponse for JsonMessage {
    fn into_response(self) -> axum::response::Response {
        let mut status_code = None;
        let mut body: Option<Json<Self>> = None;
        match self {
            Self::User { .. } => {
                body.replace(self.into());
            }
            Self::QuoridorID(..) => {
                body.replace(self.into());
            }
            Self::Unauthorized => {
                status_code.replace(StatusCode::FORBIDDEN);
            }
            Self::NotFound => {
                status_code.replace(StatusCode::NOT_FOUND);
            }
            Self::NotAnEmail => {
                body.replace(self.into());
            }
            Self::ShouldNotBeEmail => {
                body.replace(self.into());
            }
            Self::AlreadyTaken => {
                body.replace(self.into());
            }
            Self::ServerError => {
                status_code.replace(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
        match body {
            Some(data) => (status_code.unwrap_or(StatusCode::OK), data).into_response(),
            None => status_code.unwrap_or(StatusCode::OK).into_response(),
        }
    }
}
