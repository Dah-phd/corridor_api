use crate::game_logic::Corridor;

pub struct GameSession {
    up_player: String,
    down_player: String,
    game: Corridor,
    turn: usize,
    current: String,
}

const OK: &str = "ok";
const WRONG_PLAYER: &str = "await turn";
const UNALLOWED: &str = "wrong move";

impl GameSession {
    pub fn new(player_1: &str, player_2: &str) -> GameSession {
        GameSession {
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
