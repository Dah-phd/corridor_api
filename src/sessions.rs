use crate::game_logic::Quoridor;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
extern crate rocket;
use rocket::fs::{relative, FileServer};
use rocket::serde::{Deserialize, Serialize};

pub trait GameSession {
    type Position;
    type Spec;
    fn new(player_list: &Vec<&str>, id: i32) -> Self;
    fn move_player(&mut self, player: &str, new_position: Self::Position, options: Self::Spec) -> PlayerMoveResult;
    fn get_id(&self) -> i32;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct QuoridorSession {
    pub id: i32,
    up_player: String,
    down_player: String,
    game: Quoridor,
    turn: usize,
    current: String,
}

pub enum PlayerMoveResult {
    Ok,
    WrongPlayer,
    Unallowed,
    Unknown,
}

impl QuoridorSession {
    pub fn new_wall(&mut self, player: &str, position: (usize, usize), wall_type: &str) -> PlayerMoveResult {
        if player != self.current {
            return PlayerMoveResult::WrongPlayer;
        }
        if player == self.up_player {
            if 1 > self.game.up_player_free_walls || !self.game.new_wall(position, wall_type) {
                return PlayerMoveResult::Unallowed;
            }
            self.game.up_player_free_walls -= 1
        }
        if player == self.down_player {
            if 1 > self.game.down_player_free_walls || !self.game.new_wall(position, wall_type) {
                return PlayerMoveResult::Unallowed;
            }
            self.game.down_player_free_walls -= 1
        }
        self.switch_players();
        PlayerMoveResult::Ok
    }

    fn switch_players(&mut self) {
        if self.current == self.up_player {
            self.current = self.down_player.to_owned();
        } else {
            self.current = self.up_player.to_owned()
        }
    }
}

impl GameSession for QuoridorSession {
    type Position = (usize, usize);
    type Spec = ();
    fn new(player_list: &Vec<&str>, id: i32) -> Self {
        QuoridorSession {
            up_player: player_list[0].to_owned(),
            down_player: player_list[1].to_owned(),
            id,
            game: Quoridor::new(),
            turn: 0,
            current: player_list[0].to_owned(),
        }
    }
    fn move_player(&mut self, player: &str, new_position: Self::Position, options: Self::Spec) -> PlayerMoveResult {
        if self.current != player {
            return PlayerMoveResult::WrongPlayer;
        }
        let player_code = if player == self.up_player { "up" } else { "down" };
        if !self.game.move_player(new_position, player_code) {
            return PlayerMoveResult::Unallowed;
        }
        if self.game.up_player != self.game.down_player {
            self.switch_players();
        }
        PlayerMoveResult::Ok
    }

    fn get_id(&self) -> i32 {
        self.id
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

#[derive(Clone)]
pub enum PlayerMove {
    QuoridorWallV((usize, usize), String),
    QuoridorWallH((usize, usize), String),
    QuoridorMove((usize, usize), String),
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
