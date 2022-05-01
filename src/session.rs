use crate::game_logic::Corridor;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct CorridorSession {
    up_player: String,
    down_player: String,
    game: Corridor,
    turn: usize,
    current: String,
}

const OK: &str = "ok";
const WRONG_PLAYER: &str = "await turn";
const UNALLOWED: &str = "wrong move";

impl CorridorSession {
    pub fn new(player_1: &str, player_2: &str) -> CorridorSession {
        CorridorSession {
            up_player: player_1.to_owned(),
            down_player: player_2.to_owned(),
            game: Corridor::new(),
            turn: 0,
            current: player_1.to_owned(),
        }
    }
    pub fn new_border(&mut self, player: &str, position: (usize, usize), border_type: &str) -> &str {
        if player != self.current {
            return WRONG_PLAYER;
        }
        if !self.game.new_border(position, border_type) {
            return UNALLOWED;
        }
        if self.current == self.up_player {
            self.current = self.down_player.to_owned();
        } else {
            self.current = self.up_player.to_owned()
        }
        OK
    }
    pub fn move_player(&mut self, new_position: (usize, usize), player: &str) -> &str {
        if self.current != player {
            return WRONG_PLAYER;
        }
        let player_code = if player == self.up_player { "up" } else { "down" };
        if !self.game.move_player(new_position, player_code) {
            return UNALLOWED;
        }
        OK
    }
}

pub fn corridor_mover(player_move: PlayerMove, session: &'static mut CorridorSession) -> &'static str {
    // TODO be ware of lifetimes
    match player_move {
        PlayerMove::CorridorBorderH(val, player) => session.new_border(&player, val, "h"),
        PlayerMove::CorridorBorderV(val, player) => session.new_border(&player, val, "v"),
        PlayerMove::CorridorMove(val, player) => session.move_player(val, &player),
        _ => "unknown move",
    }
}

#[derive(Clone)]
pub enum PlayerMove {
    CorridorBorderV((usize, usize), String),
    CorridorBorderH((usize, usize), String),
    CorridorMove((usize, usize), String),
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
