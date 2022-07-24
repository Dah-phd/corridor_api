use rocket::serde::{Deserialize, Serialize};
use std::sync::Mutex;
extern crate rand;
use crate::game_lobbies::Lobby;
use chrono;
use rand::{distributions::Alphanumeric, Rng};
// importing Matchs for each game
use crate::quoridor::QuoridorMatch;

const AFK_CONCEDE_TIMER: i64 = 180;

pub trait GameInterface {
    type Position;
    type Spec;
    fn new(player_list: &Vec<String>, owner: String) -> Self;
    fn make_move(&mut self, player_move: PlayerMove) -> PlayerMoveResult;
    fn contains_player(&self, player: &str) -> bool;
    fn get_type(&self) -> MatchType;
    fn get_winner(&self) -> Option<String>;
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

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "rocket::serde")]
pub enum MatchType {
    Quoridor,
    Unknown,
}

impl MatchType {
    pub fn get_expected_players(&self) -> usize {
        match self {
            Self::Quoridor => 2,
            _ => 0,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub enum GenericGame {
    ActiveQuoridor(QuoridorMatch),
    NotFound,
}

impl GenericGame {
    pub fn new(player_list: &Vec<String>, owner: &str, game_type: MatchType) -> Option<Self> {
        match game_type {
            MatchType::Quoridor => Some(Self::ActiveQuoridor(QuoridorMatch::new(player_list, owner.to_owned()))),
            _ => None,
        }
    }

    pub fn unwrap_mut(&mut self) -> &mut QuoridorMatch {
        match self {
            GenericGame::ActiveQuoridor(game) => game,
            _ => panic!("NotFound method called!"),
        }
    }

    pub fn unwrap(&self) -> &QuoridorMatch {
        match self {
            GenericGame::ActiveQuoridor(game) => game,
            _ => panic!("NotFound method called!"),
        }
    }
}

pub struct Match {
    pub game: GenericGame,
    timestamp: i64,
}

impl Match {
    pub fn new(player_list: &Vec<String>, owner: &str, game_type: MatchType) -> Option<Self> {
        if let Some(game) = GenericGame::new(player_list, owner, game_type) {
            return Some(Self {
                game,
                timestamp: chrono::Utc::now().timestamp(),
            });
        }
        None
    }

    fn is_expaired(&self) -> bool {
        self.timestamp + AFK_CONCEDE_TIMER + 30 < chrono::Utc::now().timestamp()
    }

    fn refresh_move_timer(&mut self) {
        self.timestamp = chrono::Utc::now().timestamp();
    }

    pub fn get_owner(&self) -> String {
        self.game.unwrap().owner.to_owned()
    }

    fn make_move(&mut self, player_move: PlayerMove) -> PlayerMoveResult {
        self.refresh_move_timer();
        self.game.unwrap_mut().make_move(player_move)
    }

    pub fn contains_player(&self, player: &String) -> bool {
        self.game.unwrap().contains_player(player)
    }

    fn get_winner(&self) -> Option<String> {
        self.game.unwrap().get_winner()
    }
}

pub struct ActiveMatchs {
    matchs: Mutex<Vec<Match>>,
}

impl ActiveMatchs {
    pub fn new() -> Self {
        Self {
            matchs: Mutex::new(Vec::new()),
        }
    }

    pub fn create_cpu_game(&self, player: &str, game_type: MatchType) -> Option<String> {
        let id_len = 8;
        let mut id = generate_rand_string(id_len);
        let game_list = &mut *self.matchs.lock().unwrap();
        while game_list.iter().any(|x| x.get_owner() == id) {
            id = generate_rand_string(id_len)
        }
        let new_game = Match::new(&vec![player.to_owned()], &id, game_type);
        if let Some(game) = new_game {
            game_list.push(game);
            return Some(id);
        }
        None
    }

    pub fn append(&self, lobby: &Lobby) -> bool {
        self.drop_finished();
        let game_list = &mut *self.matchs.lock().unwrap();
        let new_game = Match::new(&lobby.player_list, &lobby.player_list[0], lobby.match_type);
        if let Some(game) = new_game {
            game_list.push(game);
            return true;
        }
        false
    }

    pub fn get_match_by_player(&self, player: &String) -> Option<GenericGame> {
        let game_list = &mut *self.matchs.lock().unwrap();
        for game in game_list {
            if game.contains_player(player) {
                if game.is_expaired() {}
                return Some(game.game.clone());
            }
        }
        None
    }

    pub fn make_move(&self, owner: &String, player_move: PlayerMove) -> Option<PlayerMoveResult> {
        let game_list = &mut *self.matchs.lock().unwrap();
        for game in game_list {
            if game.get_owner() == *owner {
                return Some(game.make_move(player_move));
            };
        }
        None
    }

    pub fn drop_by_owner(&self, owner: &String) {
        self.drop_finished();
        let game_list = &mut *self.matchs.lock().unwrap();
        game_list.retain(|x| &x.get_owner() != owner)
    }

    fn drop_finished(&self) {
        let game_list = &mut *self.matchs.lock().unwrap();
        game_list.retain(|x| x.get_winner().is_none() || !x.is_expaired())
    }
}

pub fn generate_rand_string(len: usize) -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();
    s
}
