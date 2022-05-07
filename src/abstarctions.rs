use chrono;
use rocket::serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Mutex;
// importing Matchs for each game
use crate::quoridor::QuoridorMatch;

pub trait GameMatch {
    type Position;
    type Spec;
    fn new(player_list: &Vec<String>, id: i32) -> Self;
    fn move_player(&mut self, player: &str, new_position: Self::Position, options: Self::Spec) -> PlayerMoveResult;
    fn get_id(&self) -> i32;
    fn make_move(&mut self, player_move: PlayerMove) -> PlayerMoveResult;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub enum ChatID {
    RoomID(String),
    MatchID(i32),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Messages {
    pub id: ChatID,
    pub player: String,
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub enum PlayerMove {
    QuoridorWallV((usize, usize), String),
    QuoridorWallH((usize, usize), String),
    QuoridorMove((usize, usize), String),
    ChessMove((usize, usize), (usize, usize)), // ((from)=>(to))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub enum PlayerMoveResult {
    Ok,
    WrongPlayer,
    Unallowed,
    Unknown,
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
    fn new(player_list: &Vec<String>, free_id: i32, match_type: MatchType) -> Option<Self> {
        match match_type {
            MatchType::Quoridor => Some(Self::ActiveQuoridor(QuoridorMatch::new(player_list, free_id))),
            _ => None,
        }
    }
    pub fn get_id(&self) -> i32 {
        match self {
            Match::ActiveQuoridor(v) => return v.id,
            _ => panic!("NotFound method called!"),
        }
    }

    pub fn expose(&mut self) -> &mut QuoridorMatch {
        match self {
            Match::ActiveQuoridor(v) => v,
            _ => panic!("NotFound method called!"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Room {
    pub owner: String,
    pub match_type: MatchType,
    pub player_list: Vec<String>,
    pub game_id: Option<i32>,
}

pub struct MatchRooms {
    pub rooms: Mutex<Vec<Room>>,
}

impl MatchRooms {
    pub fn new() -> Self {
        Self {
            rooms: Mutex::new(Vec::new()),
        }
    }

    pub fn get_all(&self) -> Vec<Room> {
        return self.rooms.lock().unwrap().clone().to_vec();
    }

    pub fn new_room(&self, player_name: &str, match_type: MatchType) -> bool {
        let mut room_list = self.rooms.lock().unwrap();
        let exposed_vector = &mut *room_list;
        for room in exposed_vector.iter() {
            if room.owner == player_name {
                return false;
            }
        }
        exposed_vector.push(Room {
            owner: player_name.to_owned(),
            match_type,
            player_list: vec![player_name.to_owned()],
            game_id: None,
        });
        true
    }

    pub fn kick_player(&self, owner: &str, player: &str) {
        let mut room_list = self.rooms.lock().unwrap();
        let exposed_vector = &mut *room_list;
        for room in exposed_vector {
            if room.owner == owner {
                room.player_list.retain(|pl| pl != player);
                return;
            }
        }
    }

    pub fn get_by_owner(&self, player_name: &str) -> Option<Room> {
        let mut room_list = self.rooms.lock().unwrap();
        let exposed_vector = &mut *room_list;
        for room in exposed_vector {
            if room.owner == player_name {
                return Some(room.clone());
            }
        }
        None
    }

    pub fn add_player(&self, owner: String, player: String) -> bool {
        let mut room_list = self.rooms.lock().unwrap();
        let exposed_vector = &mut *room_list;
        for room in exposed_vector {
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

impl ActiveMatchs {
    pub fn new() -> Self {
        Self {
            matchs: Mutex::new(Vec::new()),
        }
    }
    pub fn append(&self, player_list: &Vec<String>, match_type: MatchType) -> Result<i32, &str> {
        let mut matchs_list = self.matchs.lock().unwrap();
        let exposed_vector = &mut *matchs_list;
        let mut id = 0;
        while ActiveMatchs::is_id_taken(exposed_vector, id) {
            id += 1
        }
        let new_match = Match::new(player_list, id, match_type);
        match new_match {
            Some(match_) => {
                exposed_vector.push(match_);
                Ok(id)
            }
            None => Err("Unable to build match!"),
        }
    }

    pub fn get_match(&self, id: i32) -> Option<Match> {
        let mut matchs_list = self.matchs.lock().unwrap();
        let exposed_vector = &mut *matchs_list;
        for match_ in exposed_vector {
            if match_.expose().id == id {
                return Some(match_.clone());
            }
        }
        None
    }

    pub fn drop(&self, id: i32) {
        let mut matchs_list = self.matchs.lock().unwrap();
        let exposed_vector = &mut *matchs_list;
        for (i, match_) in exposed_vector.iter().enumerate() {
            if match_.get_id() == id {
                exposed_vector.remove(i);
                return;
            }
        }
    }

    pub fn make_move(&self, id: i32, player_move: PlayerMove) -> Option<PlayerMoveResult> {
        let mut matchs_list = self.matchs.lock().unwrap();
        let exposed_vector = &mut *matchs_list;
        for match_ in exposed_vector {
            let exposed_match = match_.expose();
            if exposed_match.id == id {
                return Some(exposed_match.make_move(player_move));
            }
        }
        None
    }

    fn is_id_taken(exposed_vector: &Vec<Match>, id: i32) -> bool {
        for match_ in exposed_vector {
            if match_.get_id() == id {
                return true;
            }
        }
        false
    }
}

pub struct EventListener {
    event_map: HashMap<String, Vec<&'static fn(PlayerMove) -> bool>>,
}

impl EventListener {
    pub fn new() -> EventListener {
        EventListener {
            event_map: HashMap::new(),
        }
    }

    pub fn subscribe_to_event(&mut self, event: String, func: &'static fn(PlayerMove) -> bool) {
        match self.event_map.entry(event) {
            Entry::Vacant(e) => {
                e.insert(vec![func]);
            }
            Entry::Occupied(mut e) => {
                e.get_mut().push(func);
            }
        }
    }
    pub fn pass_event(&self, event: String, data: PlayerMove) {
        match self.event_map.get(&event) {
            Some(func_ls) => {
                for func in func_ls {
                    func(data.clone());
                }
            }
            None => (),
        }
    }
}
