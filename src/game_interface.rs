// importing Matchs for each game
use crate::quoridor::QuoridorMatch;
use rocket::serde::{Deserialize, Serialize};
extern crate rand;
use crate::game_matches::GameType;

const AFK_CC_TIMER: i64 = 180;

pub trait GenericGameInterface {
    type Position;
    type Spec;
    fn new(player_list: &Vec<String>, owner: String, timestamp: i64) -> Self;
    fn make_move(&mut self, player_move: PlayerMove) -> PlayerMoveResult;
    fn contains_player(&self, player: &str) -> bool;
    fn get_winner(&self) -> Option<String>;
    fn get_timestamp(&self) -> i64;
    fn set_timestamp(&mut self, timestamp: i64);
    fn get_current_player(&self) -> &String;

    fn is_expaired(&self) -> bool {
        self.get_timestamp() + AFK_CC_TIMER + 30 < chrono::Utc::now().timestamp()
    }

    fn refresh_timestamp(&mut self) {
        self.set_timestamp(chrono::Utc::now().timestamp())
    }

    fn timeout_guard(&mut self, player: &String) {
        if self.get_timestamp() + AFK_CC_TIMER < chrono::Utc::now().timestamp() && self.get_current_player() != player {
            self.make_move(PlayerMove::Concede(player.to_owned()));
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub enum PlayerMove {
    Concede(String),
    QuoridorWallV((usize, usize), String),
    QuoridorWallH((usize, usize), String),
    QuoridorMove((usize, usize), String),
    ChessMove((usize, usize), (usize, usize), String), // ((from)=>(to))
}

impl PlayerMove {
    pub fn confirm_player(&self, player: &String) -> bool {
        match self {
            Self::QuoridorWallH(_, move_player) => player == move_player,
            Self::QuoridorWallV(_, move_player) => player == move_player,
            Self::QuoridorMove(_, move_player) => player == move_player,
            Self::ChessMove(_, _, move_player) => player == move_player,
            Self::Concede(move_player) => player == move_player,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
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

#[derive(Debug, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub enum GenericGame {
    ActiveQuoridor(QuoridorMatch),
    NotFound,
}

impl GenericGame {
    pub fn new(player_list: &Vec<String>, owner: &str, match_type: GameType) -> Option<Self> {
        match match_type {
            GameType::Quoridor => Some(Self::ActiveQuoridor(QuoridorMatch::new(
                player_list,
                owner.to_owned(),
                chrono::Utc::now().timestamp(),
            ))),
            _ => None,
        }
    }
    pub fn get_owner(&self) -> String {
        match self {
            GenericGame::ActiveQuoridor(game) => return game.owner.to_owned(),
            _ => panic!("NotFound method called!"),
        }
    }

    pub fn make_move(&mut self, player_move: PlayerMove) -> PlayerMoveResult {
        match self {
            GenericGame::ActiveQuoridor(game) => {
                game.refresh_timestamp();
                game.make_move(player_move)
            }
            _ => panic!("NotFound method called!"),
        }
    }

    pub fn contains_player(&self, player: &String) -> bool {
        match self {
            GenericGame::ActiveQuoridor(game) => game.contains_player(player),
            _ => panic!("NotFound method called!"),
        }
    }

    pub fn is_expaired(&self) -> bool {
        match self {
            GenericGame::ActiveQuoridor(game) => game.is_expaired(),
            _ => panic!("NotFound method called!"),
        }
    }

    pub fn get_winner(&self) -> Option<String> {
        match self {
            GenericGame::ActiveQuoridor(game) => game.get_winner(),
            _ => panic!("NotFound method called!"),
        }
    }

    pub fn timeout_guard(&mut self, player: &String) {
        match self {
            GenericGame::ActiveQuoridor(game) => game.timeout_guard(player),
            _ => panic!("NotFound method called!"),
        }
    }
}
