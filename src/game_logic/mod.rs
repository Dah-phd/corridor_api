mod components;

pub struct Corridor {
    up_player: String,
    down_player: String,
    pub id: String,
    pub board: components::Board,
    pub borders: components::Borders,
    pub winner: Option<bool>,
}

impl Corridor {
    pub fn new(id: String, up_player: String, down_player: String) -> Corridor {
        Corridor {
            id: id,
            up_player: up_player,
            down_player: down_player,
            board: components::Board::new(),
            borders: components::Borders::new(),
            winner: None,
        }
    }
    pub fn print_state(&self) {
        for row_id in 0..9 {
            for col_id in 0..9 {
                if self.board.player_up == (row_id, col_id) {
                    print!("[U]")
                } else if self.board.player_down == (row_id, col_id) {
                    print!("[D]")
                } else {
                    print!("[ ]")
                }
            }
            print!("\n")
        }
    }
    pub fn move_up_player(&mut self, new_position: (usize, usize)) -> bool {
        self.move_player(new_position, self.board.player_up, "up")
    }
    pub fn move_down_player(&mut self, new_position: (usize, usize)) -> bool {
        self.move_player(new_position, self.board.player_down, "down")
    }

    fn move_player(&mut self, new_position: (usize, usize), old_position: (usize, usize), player: &str) -> bool {
        if self.is_move_possible(old_position, new_position) {
            self.is_win(player, new_position);
            self.board.move_player(player, new_position)
        } else {
            false
        }
    }
    fn is_win(&self, player: &str, new_position: (usize, usize)) -> bool {
        if player == "up" {
            return true;
        }
        false
    }

    fn is_move_possible(&self, old_position: (usize, usize), new_position: (usize, usize)) -> bool {
        if self.is_move_blocked_by_border_or_wrong(old_position, new_position) {
            return false;
        }
        if new_position.0 > 8 || new_position.1 > 8 {
            return false;
        }
        true
    }

    fn is_move_blocked_by_border_or_wrong(&self, old_position: (usize, usize), new_position: (usize, usize)) -> bool {
        // positon based on (row_idx, col_idx)
        if old_position.0 == new_position.0 {
            if old_position.1 > new_position.1 {
                if old_position.1 - new_position.1 == 1 && !self.borders.v_borders.contains(&(new_position.0, new_position.1)) {
                    return false;
                } else if old_position.1 - new_position.1 == 2
                    && !self.borders.v_borders.contains(&(new_position.0, new_position.1))
                    && !self.borders.v_borders.contains(&(new_position.0, new_position.1 - 1))
                {
                    return false;
                } else {
                    return true;
                }
            } else if old_position.1 < new_position.1 {
                if new_position.1 - old_position.1 == 1 && !self.borders.v_borders.contains(&(new_position.0, old_position.1)) {
                    return false;
                } else if new_position.1 - old_position.1 == 2
                    && !self.borders.v_borders.contains(&(new_position.0, old_position.1))
                    && !self.borders.v_borders.contains(&(new_position.0, old_position.1 + 1))
                {
                    return false;
                } else {
                    return true;
                }
            }
        } else if old_position.1 == new_position.1 {
            if old_position.0 > new_position.0 {
                if old_position.0 - new_position.0 == 1 && self.borders.h_borders.contains(&(new_position.0, old_position.1)) {
                    return false;
                } else if old_position.0 - new_position.0 == 2
                    && self.borders.h_borders.contains(&(new_position.0, old_position.1))
                    && self.borders.h_borders.contains(&(new_position.0 - 1, old_position.1))
                {
                    return false;
                } else {
                    return true;
                }
            } else if old_position.0 < new_position.0 {
                if new_position.0 - old_position.0 == 1 && self.borders.h_borders.contains(&(new_position.0, old_position.1)) {
                    return false;
                } else if new_position.0 - old_position.0 == 2
                    && self.borders.h_borders.contains(&(new_position.0, old_position.1))
                    && self.borders.h_borders.contains(&(new_position.0 + 1, old_position.1))
                {
                    return false;
                } else {
                    return true;
                }
            } else {
                return true;
            }
        }
        true
    }

    pub fn new_border(&mut self, border: (usize, usize), border_type: String) -> bool {
        // border could be "h" for horizontal or "v" for vertical and holds the (row_idx, col_idx) for first point
        if border_type == "h" && !self.borders.h_borders.contains(&border) || !self.borders.h_borders.contains(&border) {
            return false;
        }
        if border_type == "v" && !self.borders.v_borders.contains(&border) || !self.borders.v_borders.contains(&border) {
            return false;
        }
        self.borders.add_borders(border.0, border.1, &border_type);
        if !self.blocks_player(&border, &border_type) {
            return true;
        } else {
            self.borders.remove(&border_type);
            false
        }
    }

    fn blocks_player(&self, border: &(usize, usize), border_type: &str) -> bool {
        // check if step over other border
        if !self.player_can_win(self.board.player_up, &mut vec![self.board.player_up], 0, 0) {
            return true;
        }
        if !self.player_can_win(self.board.player_down, &mut vec![self.board.player_down], 8, 0) {
            return true;
        }
        false
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
            if start_position.0 < 8 {
                possible_paths.push((start_position.0 + 1, start_position.1))
            }
            if start_position.1 > 0 {
                possible_paths.push((start_position.0, start_position.1 - 1));
            }
            if start_position.1 < 8 {
                possible_paths.push((start_position.0, start_position.1 - 1));
            }
            for possible_path in possible_paths {
                if !past_position.contains(&possible_path)
                    && !self.is_move_blocked_by_border_or_wrong(start_position, possible_path)
                    && self.player_can_win(possible_path, past_position, target, target_coordinate)
                {
                    return true;
                }
            }
        }
        false
    }
}

fn get_usize_diff(x: usize, y: usize) -> usize {
    if x > y {
        return x - y;
    } else {
        return y - x;
    }
}
