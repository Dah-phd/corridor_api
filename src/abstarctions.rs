use rocket::serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Mutex;
// importing sessiong
use crate::quoridor::QuoridorSession;

pub trait GameSession {
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
    SessionID(i32),
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

pub enum PlayerMoveResult {
    Ok,
    WrongPlayer,
    Unallowed,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "rocket::serde")]
pub enum SessionType {
    Quoridor,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub enum Session {
    ActiveQuoridor(QuoridorSession),
    NotFound,
}

impl Session {
    fn new(player_list: &Vec<String>, free_id: i32, session_type: SessionType) -> Option<Self> {
        match session_type {
            SessionType::Quoridor => Some(Self::ActiveQuoridor(QuoridorSession::new(player_list, free_id))),
            _ => None,
        }
    }
    pub fn get_id(&self) -> i32 {
        match self {
            Session::ActiveQuoridor(v) => v.get_id(),
            _ => panic!("insert get_id for new Session!"),
        }
    }
    pub fn make_move(&mut self, player_move: PlayerMove) -> PlayerMoveResult {
        match self {
            Session::ActiveQuoridor(v) => v.make_move(player_move),
            _ => PlayerMoveResult::Unknown,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Room {
    pub owner: String,
    pub session_type: SessionType,
    pub player_list: Vec<String>,
    pub game_id: Option<i32>,
}

pub struct SessionRooms {
    pub rooms: Mutex<Vec<Room>>,
}

impl SessionRooms {
    pub fn new() -> Self {
        Self {
            rooms: Mutex::new(Vec::new()),
        }
    }

    pub fn get_all(&self) -> Vec<Room> {
        return self.rooms.lock().unwrap().clone().to_vec();
    }

    pub fn new_room(&self, player_name: &str, session_type: SessionType) -> bool {
        let mut room_list = self.rooms.lock().unwrap();
        let exposed_vector = &mut *room_list;
        for room in exposed_vector.iter() {
            if room.owner == player_name {
                return false;
            }
        }
        exposed_vector.push(Room {
            owner: player_name.to_owned(),
            session_type,
            player_list: vec![player_name.to_owned()],
            game_id: None,
        });
        true
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

pub struct ActiveSessions {
    pub sessions: Mutex<Vec<Session>>,
}

impl ActiveSessions {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(Vec::new()),
        }
    }
    pub fn append(&self, player_list: &Vec<String>, session_type: SessionType) -> Result<i32, &str> {
        let mut sessions_list = self.sessions.lock().unwrap();
        let exposed_vector = &mut *sessions_list;
        let mut id = 0;
        while ActiveSessions::is_id_taken(exposed_vector, id) {
            id += 1
        }
        let new_session = Session::new(player_list, id, session_type);
        match new_session {
            Some(session) => {
                exposed_vector.push(session);
                Ok(id)
            }
            None => Err("Unable to build session!"),
        }
    }

    pub fn get_session(&self, id: i32) -> Option<Session> {
        let mut sessions_list = self.sessions.lock().unwrap();
        let exposed_vector = &mut *sessions_list;
        for session in exposed_vector {
            if session.get_id() == id {
                return Some(session.clone());
            }
        }
        None
    }

    pub fn drop(&self, id: i32) {
        let mut sessions_list = self.sessions.lock().unwrap();
        let exposed_vector = &mut *sessions_list;
        for (i, session) in exposed_vector.iter().enumerate() {
            if session.get_id() == id {
                exposed_vector.remove(i);
                return;
            }
        }
    }

    pub fn make_move(&self, id: i32, player_move: PlayerMove) -> Option<PlayerMoveResult> {
        let mut sessions_list = self.sessions.lock().unwrap();
        let exposed_vector = &mut *sessions_list;
        for session in exposed_vector {
            if session.get_id() == id {
                return Some(session.make_move(player_move));
            }
        }
        None
    }

    fn is_id_taken(exposed_vector: &Vec<Session>, id: i32) -> bool {
        for session in exposed_vector {
            if session.get_id() == id {
                return true;
            }
        }
        false
    }
}

pub struct EventListener {
    event_map: HashMap<String, Vec<&'static dyn Fn(PlayerMove) -> bool>>,
}

impl EventListener {
    pub fn new() -> EventListener {
        EventListener {
            event_map: HashMap::new(),
        }
    }

    pub fn subscribe_to_event(&mut self, event: String, func: &'static dyn Fn(PlayerMove) -> bool) {
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
