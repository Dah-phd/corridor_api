#[derive(Debug, Clone)]
pub struct Borders {
    pub h_borders: Vec<(usize, usize)>, // (row, column)
    pub v_borders: Vec<(usize, usize)>, // (row, column)
}

impl Borders {
    pub fn new() -> Borders {
        Borders {
            h_borders: Vec::new(),
            v_borders: Vec::new(),
        }
    }
    pub fn add_borders(&mut self, row: usize, col: usize, plain_h_or_v: &str) -> bool {
        if plain_h_or_v == "h" {
            self.h_borders.push((row, col));
            return true;
        }
        if plain_h_or_v == "v" {
            self.v_borders.push((row, col));
            return true;
        }
        false
    }
    pub fn remove(&mut self, plain_h_or_v: &str) {
        if plain_h_or_v == "h" {
            self.h_borders.pop();
        }
        if plain_h_or_v == "v" {
            self.v_borders.pop();
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    // efemeral board of 9 by 9 where the players are positioned
    pub player_up: (usize, usize),
    pub player_down: (usize, usize),
}
impl Board {
    pub fn new() -> Board {
        Board {
            player_up: (0, 4),
            player_down: (8, 4),
        }
    }
    pub fn move_player(&mut self, player: &str, new_position: (usize, usize)) -> bool {
        if player == "up" {
            self.player_up = new_position;
            true
        } else if player == "down" {
            self.player_down = new_position;
            true
        } else {
            false
        }
    }
    fn is_possible() -> bool {
        true
    }
}
