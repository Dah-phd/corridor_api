use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Quoridor {
    pub up_player: (usize, usize),
    pub down_player: (usize, usize),
    pub up_player_free_walls: usize,
    pub down_player_free_walls: usize,
    pub vertical_walls: Vec<(usize, usize)>,   // (row, col)
    pub horizontal_walls: Vec<(usize, usize)>, // (row, col)
}

impl Quoridor {
    pub fn new() -> Self {
        Self {
            up_player: (0, 4),
            down_player: (8, 4),
            up_player_free_walls: 9,
            down_player_free_walls: 9,
            vertical_walls: Vec::new(),
            horizontal_walls: Vec::new(),
        }
    }

    pub fn get_shortest_path(&self, player: (usize, usize), target: usize) -> Option<Vec<(usize, usize)>> {
        a_star_traitbased::AStar::run(self, player, (Some(target), None))
    }

    fn player_can_win(&self, start_position: (usize, usize), target: usize) -> bool {
        self.get_shortest_path(start_position, target).is_some()
    }

    pub fn try_moving_up_player(&mut self, new_position: (usize, usize)) -> bool {
        if self.is_move_blocked_by_wall_or_wrong(self.up_player, new_position) {
            return false;
        }
        self.up_player = new_position;
        true
    }

    pub fn try_moving_down_player(&mut self, new_position: (usize, usize)) -> bool {
        if self.is_move_blocked_by_wall_or_wrong(self.down_player, new_position) {
            return false;
        }
        self.down_player = new_position;
        true
    }

    pub fn new_h_wall(&mut self, wall: (usize, usize)) -> bool {
        if wall.0 > 7 || wall.1 > 7 {
            return false;
        }
        if !self.wall_h_is_possible(wall) {
            return false;
        }
        self.horizontal_walls.push(wall);
        if self.player_can_win(self.up_player, 8) && self.player_can_win(self.down_player, 0) {
            return true;
        }
        self.horizontal_walls.pop();
        false
    }

    fn wall_h_is_possible(&self, new_wall: (usize, usize)) -> bool {
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
        !self.vertical_walls.contains(&new_wall)
    }

    pub fn new_v_wall(&mut self, wall: (usize, usize)) -> bool {
        if wall.0 > 7 || wall.1 > 7 {
            return false;
        }
        if !self.wall_v_is_possible(wall) {
            return false;
        }
        self.vertical_walls.push(wall);
        if self.player_can_win(self.up_player, 8) && self.player_can_win(self.down_player, 0) {
            return true;
        }
        self.vertical_walls.pop();
        false
    }

    fn wall_v_is_possible(&self, new_wall: (usize, usize)) -> bool {
        for wall in &self.vertical_walls {
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
        !self.horizontal_walls.contains(&new_wall)
    }

    fn is_move_blocked_by_wall_or_wrong(&self, start_position: (usize, usize), possible_path: (usize, usize)) -> bool {
        if start_position.0 == possible_path.0 {
            let column_move = Self::sort_positions(possible_path.1, start_position.1);
            if column_move.1 - column_move.0 != 1 {
                return true;
            };
            for wall in &self.vertical_walls {
                if wall.0 == start_position.0 && wall.1 == column_move.0 {
                    return true;
                }
                if start_position.0 != 0 && wall.0 == start_position.0 - 1 && wall.1 == column_move.0 {
                    return true;
                }
            }
            return false;
        } else if start_position.1 == possible_path.1 {
            let row_move = Self::sort_positions(possible_path.0, start_position.0);
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

    fn sort_positions(x: usize, y: usize) -> (usize, usize) {
        if x > y {
            return (y, x);
        }
        (x, y)
    }

    fn build_possible_paths(&self, from_position: (usize, usize)) -> Vec<(usize, usize)> {
        let mut possible_paths = vec![];
        if from_position.0 > 0 {
            let new_position = (from_position.0 - 1, from_position.1);
            if !self.is_move_blocked_by_wall_or_wrong(from_position, new_position) {
                possible_paths.push(new_position);
            }
        }
        if from_position.1 < 8 {
            let new_position = (from_position.0, from_position.1 + 1);
            if !self.is_move_blocked_by_wall_or_wrong(from_position, new_position) {
                possible_paths.push(new_position);
            }
        }
        if from_position.0 < 8 {
            let new_position = (from_position.0 + 1, from_position.1);
            if !self.is_move_blocked_by_wall_or_wrong(from_position, new_position) {
                possible_paths.push(new_position);
            }
        }
        if from_position.1 > 0 {
            let new_position = (from_position.0, from_position.1 - 1);
            if !self.is_move_blocked_by_wall_or_wrong(from_position, new_position) {
                possible_paths.push(new_position);
            }
        }
        possible_paths
    }
}

impl a_star_traitbased::PathGenerator for Quoridor {
    fn generate_paths(&self, from_position: (usize, usize)) -> Vec<(usize, usize)> {
        self.build_possible_paths(from_position)
    }
    fn calculate_heuristic_cost(&self, position: (usize, usize), target: (Option<usize>, Option<usize>)) -> usize {
        let target_row = target.0.unwrap();
        if position.0 > target_row {
            return position.0 - target_row;
        }
        target_row - position.0
    }

    #[allow(unused_variables)]
    fn calculate_cost(&self, current_position: (usize, usize), next_position: (usize, usize)) -> usize {
        1
    }
}
