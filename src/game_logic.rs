extern crate rocket;
use rocket::fs::{relative, FileServer};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Quoridor {
    pub up_player: (usize, usize),
    pub down_player: (usize, usize),
    pub up_player_free_walls: usize,
    pub down_player_free_walls: usize,
    pub vertcal_walls: Vec<(usize, usize)>,    // (row, col)
    pub horizontal_walls: Vec<(usize, usize)>, // (row, col)
    pub winner: Option<bool>,
}

impl Quoridor {
    pub fn new() -> Self {
        Self {
            up_player: (0, 4),
            down_player: (8, 4),
            up_player_free_walls: 9,
            down_player_free_walls: 9,
            vertcal_walls: Vec::new(),
            horizontal_walls: Vec::new(),
            winner: None,
        }
    }

    pub fn move_player(&mut self, new_position: (usize, usize), player: &str) -> bool {
        match player {
            "up" => {
                if self.is_move_blocked_by_wall_or_wrong(self.up_player, new_position) {
                    return false;
                } else {
                    self.up_player = new_position;
                    return true;
                }
            }
            "down" => {
                if self.is_move_blocked_by_wall_or_wrong(self.down_player, new_position) {
                    return false;
                } else {
                    self.down_player = new_position;
                    return true;
                }
            }
            _ => return false,
        }
    }

    pub fn new_wall(&mut self, wall: (usize, usize), wall_type: &str) -> bool {
        if wall.0 > 7 || wall.1 > 7 {
            return false;
        }
        if !self.wall_is_possible(wall, wall_type) {
            return false;
        }
        if wall_type == "h" {
            self.horizontal_walls.push(wall)
        } else {
            self.vertcal_walls.push(wall)
        };
        if self.player_can_win(self.up_player, &mut Vec::new(), 8, 0)
            && self.player_can_win(self.down_player, &mut Vec::new(), 0, 0)
        {
            return true;
        }
        if wall_type == "h" {
            self.horizontal_walls.pop();
        } else {
            self.vertcal_walls.pop();
        };
        false
    }

    fn wall_is_possible(&self, new_wall: (usize, usize), wall_type: &str) -> bool {
        match wall_type {
            "h" => {
                for wall in &self.horizontal_walls {
                    if *wall == new_wall {
                        return false;
                    }
                    if wall.1 <= 6 && (wall.0, wall.1 + 1) == new_wall {
                        return false;
                    }
                    if wall.1 >= 1 && (wall.0, wall.1 - 1) == new_wall {
                        return false;
                    }
                }
                if self.vertcal_walls.contains(&new_wall) {
                    return false;
                }
            }
            "v" => {
                for wall in &self.vertcal_walls {
                    if *wall == new_wall {
                        return false;
                    }
                    if wall.0 <= 6 && (wall.0 + 1, wall.1) == new_wall {
                        return false;
                    }
                    if wall.0 >= 1 && (wall.0 - 1, wall.1) == new_wall {
                        return false;
                    }
                }
                if self.horizontal_walls.contains(&new_wall) {
                    return false;
                }
            }
            _ => return false,
        }
        true
    }

    fn is_move_blocked_by_wall_or_wrong(&self, start_position: (usize, usize), possible_path: (usize, usize)) -> bool {
        if start_position.0 == possible_path.0 {
            let column_move = if possible_path.1 > start_position.1 {
                (start_position.1, possible_path.1)
            } else {
                (possible_path.1, start_position.1)
            };
            if column_move.1 - column_move.0 != 1 {
                return true;
            }
            for wall in &self.vertcal_walls {
                if wall.0 == start_position.0 && wall.1 == column_move.0 {
                    return true;
                }
                if start_position.0 != 0 && wall.0 == start_position.0 - 1 && wall.1 == column_move.0 {
                    return true;
                }
            }
            return false;
        } else if start_position.1 == possible_path.1 {
            let row_move = if possible_path.0 > start_position.0 {
                (start_position.0, possible_path.0)
            } else {
                (possible_path.0, start_position.0)
            };
            if row_move.1 - row_move.0 != 1 {
                return true;
            };
            for wall in &self.horizontal_walls {
                if wall.1 == start_position.1 && wall.0 == row_move.0 {
                    return true;
                }
                if start_position.1 != 0 && wall.1 == start_position.1 - 1 && wall.0 == row_move.0 {
                    return true;
                }
            }
            return false;
        }
        true
    }

    fn player_can_win(
        &self,
        start_position: (usize, usize),
        past_position: &mut Vec<(usize, usize)>,
        target: usize,
        target_coordinate: usize, // 0 for row 1 for column
    ) -> bool {
        if target_coordinate == 0 && start_position.0 == target || target_coordinate == 1 && start_position.1 == target {
            return true;
        } else {
            let mut possible_paths = vec![];
            if start_position.0 > 0 {
                possible_paths.push((start_position.0 - 1, start_position.1));
            }
            if start_position.1 < 8 {
                possible_paths.push((start_position.0, start_position.1 + 1));
            }
            if start_position.0 < 8 {
                possible_paths.push((start_position.0 + 1, start_position.1))
            }
            if start_position.1 > 0 {
                possible_paths.push((start_position.0, start_position.1 - 1));
            }
            for possible_path in possible_paths {
                if !past_position.contains(&possible_path)
                    && !self.is_move_blocked_by_wall_or_wrong(start_position, possible_path)
                {
                    past_position.push(possible_path);
                    if self.player_can_win(possible_path, past_position, target, target_coordinate) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

pub fn print_state(game: &Quoridor) {
    for row_id in 0..9 {
        let mut line = String::new();
        let mut underline = String::new();
        for col_id in 0..9 {
            if (row_id, col_id) == game.up_player || (row_id, col_id) == game.down_player {
                line.push_str("[X]");
            } else {
                line.push_str("[ ]")
            }
            if game.vertcal_walls.contains(&(row_id, col_id)) || row_id >= 1 && game.vertcal_walls.contains(&(row_id - 1, col_id))
            {
                line.push_str("|")
            } else {
                line.push_str(" ")
            }
            if game.horizontal_walls.contains(&(row_id, col_id))
                || col_id >= 1 && game.horizontal_walls.contains(&(row_id, col_id - 1))
            {
                underline.push_str("----")
            } else {
                underline.push_str("    ")
            }
        }
        println!("{line}");
        println!("{underline}")
    }
}
