use std::cmp::min;

#[derive(Debug, Clone)]
pub struct Corridor {
    pub up_player: (usize, usize),
    pub down_player: (usize, usize),
    pub vertcal_borders: Vec<(usize, usize)>,    // (row, col)
    pub horizontal_borders: Vec<(usize, usize)>, // (row, col)
    pub winner: Option<bool>,
}

impl Corridor {
    pub fn new() -> Corridor {
        Corridor {
            up_player: (0, 4),
            down_player: (8, 4),
            vertcal_borders: Vec::new(),
            horizontal_borders: Vec::new(),
            winner: None,
        }
    }

    pub fn new_border(&mut self, border: (usize, usize), border_type: &str) -> bool {
        if self.border_is_possible(border, border_type) {
            match border_type {
                "h" => self.horizontal_borders.push(border),
                "v" => self.vertcal_borders.push(border),
                _ => (),
            };
            return true;
        }
        false
    }

    fn border_is_possible(&self, new_border: (usize, usize), border_type: &str) -> bool {
        match border_type {
            "h" => {}
            "v" => {}
            _ => (),
        }
        false
    }

    fn is_move_blocked_by_border_or_wrong(&self, start_position: (usize, usize), possible_path: (usize, usize)) -> bool {
        if start_position.0 == possible_path.0 {
            let column_move = if possible_path.1 > start_position.1 {
                (start_position.1, possible_path.1)
            } else {
                (possible_path.1, start_position.1)
            };
            if column_move.1 - column_move.0 != 1 {
                return true;
            }
            for border in &self.vertcal_borders {
                if border.0 == start_position.0 && border.1 == column_move.0 {
                    return true;
                }
                if start_position.0 != 0 && border.0 == start_position.0 - 1 && border.1 == column_move.0 {
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
            for border in &self.horizontal_borders {
                if border.1 == start_position.1 && border.0 == row_move.0 {
                    return true;
                }
                if start_position.1 != 0 && border.1 == start_position.1 - 1 && border.0 == row_move.0 {
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
