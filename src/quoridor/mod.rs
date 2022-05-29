extern crate rocket;
use crate::game_abstractions::{GameMatch, PlayerMove, PlayerMoveResult};
mod a_star;
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

    pub fn new_h_wall(&mut self, wall: (usize, usize)) -> bool {
        if wall.0 > 7 || wall.1 > 7 {
            return false;
        }
        if !self.wall_h_is_possible(wall) {
            return false;
        }
        self.horizontal_walls.push(wall);
        if self.player_can_win(self.up_player, &mut Vec::new(), 8, 0)
            && self.player_can_win(self.down_player, &mut Vec::new(), 0, 0)
        {
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
        if self.vertcal_walls.contains(&new_wall) {
            return false;
        }
        true
    }

    pub fn new_v_wall(&mut self, wall: (usize, usize)) -> bool {
        if wall.0 > 7 || wall.1 > 7 {
            return false;
        }
        if !self.wall_v_is_possible(wall) {
            return false;
        }
        self.vertcal_walls.push(wall);
        if self.player_can_win(self.up_player, &mut Vec::new(), 8, 0)
            && self.player_can_win(self.down_player, &mut Vec::new(), 0, 0)
        {
            return true;
        }
        self.vertcal_walls.pop();
        false
    }

    fn wall_v_is_possible(&self, new_wall: (usize, usize)) -> bool {
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
        return possible_paths;
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
            let possible_paths = self.build_possible_paths(start_position);
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

impl a_star::PathGenerator for Quoridor {
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
    fn calculate_cost(&self, current_position: (usize, usize), next_position: (usize, usize)) -> usize {
        1
    }
}

impl Quoridor {
    pub fn get_shortest_path(&self, player: (usize, usize), target: usize) -> Result<Vec<(usize, usize)>, String> {
        use a_star::AStar;
        return AStar::run(Box::new(self), player, (Some(target), None));
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct QuoridorMatch {
    #[serde(skip_serializing)]
    pub owner: String,
    up_player: String,
    down_player: String,
    game: Quoridor,
    turn: usize,
    current: String,
}

impl QuoridorMatch {
    pub fn new_h_wall(&mut self, player: &str, position: (usize, usize)) -> PlayerMoveResult {
        let player_status = self.player_is_valid(player);
        match player_status {
            PlayerMoveResult::Ok => (),
            _ => return player_status,
        }
        if !self.game.new_h_wall(position) {
            return PlayerMoveResult::Unallowed;
        };
        if player == self.up_player {
            self.game.up_player_free_walls -= 1
        } else {
            self.game.down_player_free_walls -= 1
        }
        self.switch_players();
        player_status
    }

    pub fn new_v_wall(&mut self, player: &str, position: (usize, usize)) -> PlayerMoveResult {
        let player_status = self.player_is_valid(player);
        match player_status {
            PlayerMoveResult::Ok => (),
            _ => return player_status,
        }
        if !self.game.new_v_wall(position) {
            return PlayerMoveResult::Unallowed;
        }
        if player == self.up_player {
            self.game.up_player_free_walls -= 1
        } else {
            self.game.down_player_free_walls -= 1
        }
        self.switch_players();
        player_status
    }

    fn player_is_valid(&self, player: &str) -> PlayerMoveResult {
        if player != self.current {
            return PlayerMoveResult::WrongPlayer;
        }
        if player == self.up_player && 1 > self.game.up_player_free_walls
            || player == self.down_player && 1 > self.game.down_player_free_walls
        {
            return PlayerMoveResult::Unallowed;
        }
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

impl GameMatch for QuoridorMatch {
    type Position = (usize, usize);
    type Spec = ();
    fn new(player_list: &Vec<String>, owner: String) -> Self {
        QuoridorMatch {
            up_player: player_list[0].to_owned(),
            down_player: player_list[1].to_owned(),
            owner,
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

    fn get_owner(&self) -> String {
        self.owner.to_owned()
    }

    fn make_move(&mut self, player_move: PlayerMove) -> PlayerMoveResult {
        match player_move {
            PlayerMove::QuoridorWallH(val, player) => self.new_h_wall(&player, val),
            PlayerMove::QuoridorWallV(val, player) => self.new_v_wall(&player, val),
            PlayerMove::QuoridorMove(val, player) => self.move_player(&player, val, ()),
            _ => PlayerMoveResult::Unknown,
        }
    }

    fn contains_player(&self, player: &str) -> bool {
        if self.up_player == player || self.down_player == player {
            return true;
        }
        false
    }
}

// used for testing
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
