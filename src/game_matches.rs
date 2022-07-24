use rocket::serde::{Deserialize, Serialize};
use std::sync::Mutex;
// importing Matchs for each game
use crate::quoridor::QuoridorMatch;
extern crate rand;
use crate::game_lobbies::Lobby;
use rand::{distributions::Alphanumeric, Rng};

pub trait GameMatch {
    type Position;
    type Spec;
    fn new(player_list: &Vec<String>, owner: String) -> Self;
    fn make_move(&mut self, player_move: PlayerMove) -> PlayerMoveResult;
    fn contains_player(&self, player: &str) -> bool;
    fn get_type(&self) -> MatchType;
    fn get_winner(&self) -> Option<String>;
    fn is_expaired(&self) -> bool;
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
pub enum Match {
    ActiveQuoridor(QuoridorMatch),
    NotFound,
}

impl Match {
    pub fn new(player_list: &Vec<String>, owner: &str, match_type: MatchType) -> Option<Self> {
        match match_type {
            MatchType::Quoridor => Some(Self::ActiveQuoridor(QuoridorMatch::new(player_list, owner.to_owned()))),
            _ => None,
        }
    }
    pub fn get_owner(&self) -> String {
        match self {
            Match::ActiveQuoridor(game) => return game.owner.to_owned(),
            _ => panic!("NotFound method called!"),
        }
    }

    pub fn unwrap(&mut self) -> &mut QuoridorMatch {
        match self {
            Match::ActiveQuoridor(game) => game,
            _ => panic!("NotFound method called!"),
        }
    }

    pub fn make_move(&mut self, player_move: PlayerMove) -> PlayerMoveResult {
        match self {
            Match::ActiveQuoridor(game) => game.make_move(player_move),
            _ => panic!("NotFound method called!"),
        }
    }

    pub fn contains_player(&self, player: &String) -> bool {
        match self {
            Match::ActiveQuoridor(game) => game.contains_player(player),
            _ => panic!("NotFound method called!"),
        }
    }

    fn is_expaired(&self) -> bool {
        match self {
            Match::ActiveQuoridor(game) => game.is_expaired(),
            _ => panic!("NotFound method called!"),
        }
    }

    fn get_winner(&self) -> Option<String> {
        match self {
            Match::ActiveQuoridor(game) => game.get_winner(),
            _ => panic!("NotFound method called!"),
        }
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
        if new_game.is_none() {
            return false;
        }
        game_list.push(new_game.unwrap());
        true
    }

    pub fn get_match_by_player(&self, player: &String) -> Option<Match> {
        let game_list = &mut *self.matchs.lock().unwrap();
        for game in game_list {
            if game.unwrap().contains_player(player) {
                return Some(game.clone());
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
