use rocket::serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Mutex;
// importing sessiong
use crate::quoridor::QuoridorSession;

pub trait GameSession {
    type Position;
    type Spec;
    fn new(player_list: &Vec<&str>, id: i32) -> Self;
    fn move_player(&mut self, player: &str, new_position: Self::Position, options: Self::Spec) -> PlayerMoveResult;
    fn get_id(&self) -> i32;
}

#[derive(Clone)]
pub enum PlayerMove {
    QuoridorWallV((usize, usize), String),
    QuoridorWallH((usize, usize), String),
    QuoridorMove((usize, usize), String),
}

pub enum PlayerMoveResult {
    Ok,
    WrongPlayer,
    Unallowed,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub enum SessionType {
    Quoridor,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub enum Session {
    ActiveQuoridor(QuoridorSession),
}

impl Session {
    fn new(player_list: &Vec<&str>, free_id: i32, session_type: SessionType) -> Option<Self> {
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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Room {
    pub owner: String,
    pub session_type: SessionType,
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
    fn append(&self, player_list: &Vec<&str>, session_type: SessionType) -> Result<i32, &str> {
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

    fn is_id_taken(exposed_vector: &Vec<Session>, id: i32) -> bool {
        for session in exposed_vector {
            if session.get_id() == id {
                return true;
            }
        }
        false
    }
}

pub fn quoridor_mover(player_move: PlayerMove, session: &mut QuoridorSession) -> PlayerMoveResult {
    match player_move {
        PlayerMove::QuoridorWallH(val, player) => session.new_wall(&player, val, "h"),
        PlayerMove::QuoridorWallV(val, player) => session.new_wall(&player, val, "v"),
        PlayerMove::QuoridorMove(val, player) => session.move_player(&player, val, ()),
        _ => PlayerMoveResult::Unknown,
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
