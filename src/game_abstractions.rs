use rocket::serde::{Deserialize, Serialize};
use std::sync::Mutex;
// importing Matchs for each game
use crate::quoridor::QuoridorMatch;

pub trait GameMatch {
    type Position;
    type Spec;
    fn new(player_list: &Vec<String>, owner: String) -> Self;
    fn move_player(&mut self, player: &str, new_position: Self::Position, options: Self::Spec) -> PlayerMoveResult;
    fn make_move(&mut self, player_move: PlayerMove) -> PlayerMoveResult;
    fn contains_player(&self, player: &str) -> bool;
    fn get_type(&self) -> MatchType;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub enum PlayerMove {
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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub enum Match {
    ActiveQuoridor(QuoridorMatch),
    NotFound,
}

impl Match {
    pub fn new(player_list: &Vec<String>, owner: String, match_type: MatchType) -> Option<Self> {
        match match_type {
            MatchType::Quoridor => Some(Self::ActiveQuoridor(QuoridorMatch::new(player_list, owner))),
        }
    }
    pub fn get_owner(&self) -> String {
        match self {
            Match::ActiveQuoridor(v) => return v.owner.to_owned(),
            _ => panic!("NotFound method called!"),
        }
    }

    pub fn unwrap(&mut self) -> &mut QuoridorMatch {
        match self {
            Match::ActiveQuoridor(v) => v,
            _ => panic!("NotFound method called!"),
        }
    }

    pub fn contains_player(&self, player: &String) -> bool {
        match self {
            Match::ActiveQuoridor(game) => game.contains_player(player),
            _ => panic!("NotFound method called!"),
        }
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RoomBase {
    pub owner: String,
    pub game: MatchType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Lobby {
    pub owner: String,
    pub match_type: MatchType,
    pub player_list: Vec<String>,
    pub game_started: bool,
}

impl Lobby {
    pub fn player_in_room(&self, player: &String) -> bool {
        self.player_list.contains(player)
    }
}

pub struct MatchLobbies {
    pub rooms: Mutex<Vec<Lobby>>,
}

impl MatchLobbies {
    pub fn new() -> Self {
        Self {
            rooms: Mutex::new(Vec::new()),
        }
    }

    pub fn get_all(&self) -> Vec<Lobby> {
        return self.rooms.lock().unwrap().clone().to_vec();
    }

    pub fn new_room(&self, room_base: RoomBase) -> Option<String> {
        let mut room_list = self.rooms.lock().unwrap();
        let exposed_vector = &mut *room_list;
        for room in exposed_vector.iter() {
            if room.owner == room_base.owner {
                return None;
            }
        }
        exposed_vector.push(Lobby {
            owner: room_base.owner.to_owned(),
            match_type: room_base.game,
            player_list: vec![room_base.owner.to_owned()],
            game_started: false,
        });
        Some(room_base.owner.to_owned())
    }

    pub fn kick_player(&self, owner: &str, player: &str) {
        let room_list = &mut *self.rooms.lock().unwrap();
        for room in room_list {
            if room.owner == owner {
                room.player_list.retain(|pl| pl != player);
                return;
            }
        }
    }

    pub fn get_by_owner(&self, player_name: &str) -> Option<Lobby> {
        let room_list = &mut *self.rooms.lock().unwrap();
        for room in room_list {
            if room.owner == player_name {
                return Some(room.clone());
            }
        }
        None
    }

    pub fn add_player(&self, owner: String, player: String) -> bool {
        let room_list = &mut *self.rooms.lock().unwrap();
        for room in room_list {
            if room.owner == owner {
                room.player_list.push(player);
                return true;
            }
        }
        false
    }

    pub fn drop(&self, player_name: &str) -> bool {
        let mut room_list = self.rooms.lock().unwrap();
        let exposed_vector = &mut *room_list;
        for (i, room) in exposed_vector.iter().enumerate() {
            if room.owner == player_name {
                exposed_vector.remove(i);
                return true;
            }
        }
        false
    }
}

pub struct ActiveMatchs {
    pub matchs: Mutex<Vec<Match>>,
}

#[allow(dead_code)]
impl ActiveMatchs {
    pub fn new() -> Self {
        Self {
            matchs: Mutex::new(Vec::new()),
        }
    }
    pub fn append(&self, player_list: &Vec<String>, match_type: MatchType) -> bool {
        let matches_list = &mut *self.matchs.lock().unwrap();
        let new_match = Match::new(player_list, player_list[0].to_owned(), match_type);
        if new_match.is_none() {
            return false;
        }
        matches_list.push(new_match.unwrap());
        true
    }

    pub fn get_match(&self, player: &String) -> Option<Match> {
        let matchs_list = &mut *self.matchs.lock().unwrap();
        for match_ in matchs_list {
            if match_.unwrap().contains_player(player) {
                return Some(match_.clone());
            }
        }
        None
    }

    pub fn drop(&self, owner: &String) {
        let matchs_list = &mut *self.matchs.lock().unwrap();
        for (i, match_) in matchs_list.iter().enumerate() {
            if match_.get_owner() == *owner {
                matchs_list.remove(i);
                return;
            }
        }
    }

    pub fn make_move(&self, owner: &String, player_move: PlayerMove) -> Option<PlayerMoveResult> {
        let matchs_list = &mut *self.matchs.lock().unwrap();
        for match_ in matchs_list {
            let exposed_match = match_.unwrap();
            if exposed_match.owner == *owner {
                return Some(exposed_match.make_move(player_move));
            }
        }
        None
    }
}
