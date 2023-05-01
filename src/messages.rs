use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::errors::StateError;
use crate::leader_board::UserLeaderBoard;
use crate::quoridor::QuoridorMatch;

impl IntoResponse for UserLeaderBoard {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self).into_response()).into_response()
    }
}

impl IntoResponse for StateError {
    fn into_response(self) -> axum::response::Response {
        let mut status_code = None;
        match self {
            Self::Unauthorized => {
                status_code.replace(StatusCode::FORBIDDEN);
            }
            Self::NotFound => {
                status_code.replace(StatusCode::NOT_FOUND);
            }
            Self::AlreadyTaken => {}
            Self::UnsupportedDataType(_) => {}
            Self::ServerError => {
                status_code.replace(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
        (status_code.unwrap_or(StatusCode::OK), Json(self)).into_response()
    }
}

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
#[serde(rename_all = "camelCase")]
pub struct UserContext {
    pub email: String,
    pub username: String,
    pub auth_token: String,
    pub active_match: Option<String>,
}

impl IntoResponse for UserContext {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub user: String,
    pub message: String,
    pub timestamp: i64,
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
#[serde(rename_all = "camelCase")]
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
