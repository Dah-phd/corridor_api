use crate::game_logic::Corridor;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub trait GameSession {
    type Positon;
    type Spec;
    fn new(player_list: &Vec<&str>) -> Self;
    fn move_player(&mut self, player: &str, new_position: Self::Positon, options: Self::Spec) -> PlayerMoveResult;
}

pub struct CorridorSession {
    up_player: String,
    down_player: String,
    game: Corridor,
    turn: usize,
    current: String,
}

pub enum PlayerMoveResult {
    Ok,
    WrongPlayer,
    Unallowed,
    Unknown,
}

impl CorridorSession {
    pub fn new_border(&mut self, player: &str, position: (usize, usize), border_type: &str) -> PlayerMoveResult {
        if player != self.current {
            return PlayerMoveResult::WrongPlayer;
        }
        if !self.game.new_border(position, border_type) {
            return PlayerMoveResult::Unallowed;
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

impl GameSession for CorridorSession {
    type Positon = (usize, usize);
    type Spec = ();
    fn new(player_list: &Vec<&str>) -> Self {
        CorridorSession {
            up_player: player_list[0].to_owned(),
            down_player: player_list[1].to_owned(),
            game: Corridor::new(),
            turn: 0,
            current: player_list[0].to_owned(),
        }
    }
    fn move_player(&mut self, player: &str, new_position: Self::Positon, options: Self::Spec) -> PlayerMoveResult {
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
}

pub fn corridor_mover(player_move: PlayerMove, session: &mut CorridorSession) -> PlayerMoveResult {
    match player_move {
        PlayerMove::CorridorBorderH(val, player) => session.new_border(&player, val, "h"),
        PlayerMove::CorridorBorderV(val, player) => session.new_border(&player, val, "v"),
        PlayerMove::CorridorMove(val, player) => session.move_player(&player, val, ()),
        _ => PlayerMoveResult::Unknown,
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
