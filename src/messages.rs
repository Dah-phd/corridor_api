use serde::{Deserialize, Serialize};

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
    Concede(String),
    QuoridorWallV((usize, usize), String),
    QuoridorWallH((usize, usize), String),
    QuoridorMove((usize, usize), String),
}

impl PlayerMove {
    pub fn confirm_player(&self, player: &String) -> bool {
        match self {
            Self::QuoridorWallH(_, move_player) => player == move_player,
            Self::QuoridorWallV(_, move_player) => player == move_player,
            Self::QuoridorMove(_, move_player) => player == move_player,
            Self::Concede(move_player) => player == move_player,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PlayerMoveResult {
    Ok,
    WrongPlayerTurn,
    Disallowed,
    Unauthorized,
    Unknown,
    GameFinished,
}
impl PlayerMoveResult {
    pub fn is_ok(&self) -> bool {
        if let Self::Ok = self {
            return true;
        }
        false
    }
}
