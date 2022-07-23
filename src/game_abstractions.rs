use rocket::serde::{Deserialize, Serialize};
use std::sync::Mutex;
// importing Matchs for each game
use crate::quoridor::QuoridorMatch;
extern crate rand;
use rand::{distributions::Alphanumeric, Rng};

pub trait GameMatch {
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

    pub fn contains_player(&self, player: &String) -> bool {
        match self {
            Match::ActiveQuoridor(game) => game.contains_player(player),
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

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LobbyBase {
    pub owner: String,
    pub game: MatchType,
}

#[derive(Debug, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Lobby {
    pub owner: String,
    match_type: MatchType,
    player_list: Vec<String>,
    game_started: Option<String>,
    time_stamp: i64,
}

impl Lobby {
    pub fn new(lobby_base: &LobbyBase) -> Self {
        Self {
            owner: lobby_base.owner.to_owned(),
            match_type: lobby_base.game,
            player_list: vec![lobby_base.owner.to_owned()],
            game_started: None,
            time_stamp: chrono::Utc::now().timestamp() + 600,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.game_started.is_some()
    }
    fn prepare(&mut self) {
        if self.match_type.get_expected_players() == self.player_list.len() {
            self.game_started = Some(self.owner.to_owned())
        }
    }

    pub fn expaired(&self) -> bool {
        self.time_stamp < chrono::Utc::now().timestamp()
    }
}

pub struct MatchLobbies {
    pub lobbies: Mutex<Vec<Lobby>>,
}

impl MatchLobbies {
    pub fn new() -> Self {
        Self {
            lobbies: Mutex::new(Vec::new()),
        }
    }

    fn drop_expaired_and_started(&self) {
        let lobbies = &mut *self.lobbies.lock().unwrap();
        lobbies.retain(|x| !x.expaired() && x.game_started.is_none());
    }

    pub fn get_all(&self) -> Vec<Lobby> {
        self.drop_expaired_and_started();
        return self.lobbies.lock().unwrap().clone().to_vec();
    }

    pub fn new_lobby(&self, lobby_base: LobbyBase) -> Option<String> {
        self.drop_expaired_and_started();
        let lobbies = &mut *self.lobbies.lock().unwrap();
        for lobby in lobbies.iter() {
            if lobby.owner == lobby_base.owner {
                return None;
            }
        }
        lobbies.push(Lobby::new(&lobby_base));
        Some(lobby_base.owner.to_owned())
    }

    pub fn add_player_to_lobby(&self, lobby_owner: &String, player: &String) -> Option<Lobby> {
        self.drop_expaired_and_started();
        let lobbies = &mut *self.lobbies.lock().unwrap();
        for lobby in lobbies {
            if &lobby.owner == lobby_owner && !lobby.player_list.contains(player) {
                lobby.player_list.push(player.to_owned());
                lobby.prepare();
                return Some(lobby.clone());
            }
        }
        None
    }

    pub fn drop(&self, lobby_owner: &String) {
        let lobbies = &mut *self.lobbies.lock().unwrap();
        lobbies.retain(|x| &x.owner != lobby_owner);
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

    pub fn get_match(&self, player: &String) -> Option<Match> {
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
            let exposed_match = game.unwrap();
            if exposed_match.owner == *owner {
                return Some(exposed_match.make_move(player_move));
            }
        }
        None
    }

    pub fn drop(&self, owner: &String) {
        self.drop_finished();
        let game_list = &mut *self.matchs.lock().unwrap();
        game_list.retain(|x| &x.get_owner() != owner)
    }

    fn drop_finished(&self) {
        let game_list = &mut *self.matchs.lock().unwrap();
        game_list.retain(|x| x.get_winner().is_none())
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
