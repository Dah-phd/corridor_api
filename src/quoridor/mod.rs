extern crate rocket;
use crate::a_star_generic;
use crate::game_abstractions::{GameMatch, PlayerMove, PlayerMoveResult};
use rocket::serde::{Deserialize, Serialize};
mod cpu;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Quoridor {
    pub up_player: (usize, usize),
    pub down_player: (usize, usize),
    pub up_player_free_walls: usize,
    pub down_player_free_walls: usize,
    pub vertical_walls: Vec<(usize, usize)>,   // (row, col)
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
            vertical_walls: Vec::new(),
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
            let column_move = if possible_path.1 > start_position.1 {
                (start_position.1, possible_path.1)
            } else {
                (possible_path.1, start_position.1)
            };
            if column_move.1 - column_move.0 != 1 {
                return true;
            }
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

    fn player_can_win(&self, start_position: (usize, usize), target: usize) -> bool {
        self.get_shortest_path(start_position, target).is_some()
    }

    pub fn get_shortest_path(&self, player: (usize, usize), target: usize) -> Option<Vec<(usize, usize)>> {
        return a_star_generic::AStar::run(Box::new(self), player, (Some(target), None));
    }
}

impl a_star_generic::PathGenerator for Quoridor {
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
    only_player_moves_allowed: bool,
}

impl GameMatch for QuoridorMatch {
    type Position = (usize, usize);
    type Spec = ();
    fn new(player_list: &Vec<String>, owner: String) -> Self {
        QuoridorMatch {
            up_player: player_list[0].to_owned(),
            down_player: if player_list.len() >= 2 {
                player_list[1].to_owned()
            } else {
                cpu::CPU.to_owned()
            },
            owner,
            game: Quoridor::new(),
            turn: 0,
            current: player_list[0].to_owned(),
            only_player_moves_allowed: false,
        }
    }

    #[allow(unused_variables)]
    fn move_player(&mut self, player: &str, new_position: Self::Position, options: Self::Spec) -> PlayerMoveResult {
        if self.current != player {
            return PlayerMoveResult::WrongPlayerTurn;
        }
        let player_code = if player == self.up_player { "up" } else { "down" };
        if !self.game.move_player(new_position, player_code) {
            return PlayerMoveResult::Disallowed;
        }
        self.only_player_moves_allowed = false;
        PlayerMoveResult::Ok
    }

    fn make_move(&mut self, player_move: PlayerMove) -> PlayerMoveResult {
        let result = match player_move {
            PlayerMove::QuoridorWallH(val, player) => self.new_h_wall(&player, val),
            PlayerMove::QuoridorWallV(val, player) => self.new_v_wall(&player, val),
            PlayerMove::QuoridorMove(val, player) => self.move_player(&player, val, ()),
            _ => PlayerMoveResult::Unknown,
        };
        if result.is_ok() {
            self.end_turn();
        };
        result
    }

    fn contains_player(&self, player: &str) -> bool {
        self.up_player == player || self.down_player == player
    }
}

impl QuoridorMatch {
    pub fn new_h_wall(&mut self, player: &str, position: (usize, usize)) -> PlayerMoveResult {
        let player_status = self.player_is_valid(player);
        if !player_status.is_ok() {
            return player_status;
        }
        if self.only_player_moves_allowed || !self.game.new_h_wall(position) {
            return PlayerMoveResult::Disallowed;
        };
        self.remove_border_from_player(player);
        player_status
    }

    pub fn new_v_wall(&mut self, player: &str, position: (usize, usize)) -> PlayerMoveResult {
        let player_status = self.player_is_valid(player);
        if !player_status.is_ok() {
            return player_status;
        }
        if self.only_player_moves_allowed || !self.game.new_v_wall(position) {
            return PlayerMoveResult::Disallowed;
        }
        self.remove_border_from_player(player);
        player_status
    }

    fn remove_border_from_player(&mut self, player: &str) {
        if player == self.up_player {
            self.game.up_player_free_walls -= 1
        } else {
            self.game.down_player_free_walls -= 1
        }
    }

    fn player_is_valid(&self, player: &str) -> PlayerMoveResult {
        if player != self.current {
            return PlayerMoveResult::WrongPlayerTurn;
        }
        if player == self.up_player && 1 > self.game.up_player_free_walls
            || player == self.down_player && 1 > self.game.down_player_free_walls
        {
            return PlayerMoveResult::Disallowed;
        }
        PlayerMoveResult::Ok
    }

    fn end_turn(&mut self) {
        if self.game.up_player == self.game.down_player {
            self.only_player_moves_allowed = true;
        } else {
            self.current = if self.current == self.up_player {
                self.down_player.to_owned()
            } else {
                self.up_player.to_owned()
            }
        }
        if self.current == cpu::CPU {
            self.cpu_player_move();
        }
    }

    fn cpu_player_move(&mut self) {
        let cpu_move = cpu::CpuPlayer::get_cpu_move(&self.game, self.only_player_moves_allowed);
        self.make_move(cpu_move.clone());
    }
}

// used for testing
#[allow(dead_code)]
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
            if game.vertical_walls.contains(&(row_id, col_id))
                || row_id >= 1 && game.vertical_walls.contains(&(row_id - 1, col_id))
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn create_wall() {
        let mut new_game = Quoridor::new();
        assert!(new_game.new_h_wall((1, 0)));
        assert!(new_game.new_h_wall((1, 2)));
        assert!(new_game.new_h_wall((1, 4)));
        assert!(new_game.new_h_wall((1, 6)));
        assert!(new_game.new_v_wall((2, 6)));
        assert!(!new_game.new_h_wall((3, 7)));
    }

    #[test]
    fn make_move() {
        let expected_path = Some(vec![
            (8, 8),
            (7, 8),
            (6, 8),
            (5, 8),
            (4, 8),
            (3, 8),
            (2, 8),
            (1, 8),
            (1, 7),
            (1, 6),
            (1, 5),
            (1, 4),
            (0, 4),
        ]);
        let mut new_game = Quoridor::new();
        new_game.new_h_wall((1, 0));
        new_game.new_h_wall((1, 2));
        new_game.new_h_wall((1, 4));
        new_game.new_h_wall((1, 6));
        new_game.new_v_wall((2, 6));
        assert_eq!(new_game.get_shortest_path((0, 4), 8), expected_path);
    }

    #[test]
    fn new_match_player_moves() {
        let mut new_game = QuoridorMatch::new(&vec!["pl1".to_owned(), "pl2".to_owned()], "pl1".to_owned());
        assert!(new_game.make_move(PlayerMove::QuoridorMove((1, 4), "pl1".to_owned())).is_ok());
        assert_eq!(new_game.current, "pl2");
        assert!(new_game.make_move(PlayerMove::QuoridorMove((8, 5), "pl2".to_owned())).is_ok());
        assert_eq!(new_game.current, "pl1");
    }

    #[test]
    fn new_match_make_borders() {
        let mut new_game = QuoridorMatch::new(&vec!["pl1".to_owned(), "pl2".to_owned()], "pl1".to_owned());
        assert!(new_game
            .make_move(PlayerMove::QuoridorWallH((1, 0), "pl1".to_owned()))
            .is_ok());
        assert_eq!(new_game.current, "pl2");
        assert!(new_game
            .make_move(PlayerMove::QuoridorWallH((1, 2), "pl2".to_owned()))
            .is_ok());
        assert_eq!(new_game.current, "pl1");
        assert!(new_game
            .make_move(PlayerMove::QuoridorWallH((1, 4), "pl1".to_owned()))
            .is_ok());
        assert_eq!(new_game.current, "pl2");
        assert!(new_game
            .make_move(PlayerMove::QuoridorWallH((1, 6), "pl2".to_owned()))
            .is_ok());
        assert_eq!(new_game.current, "pl1");
        assert!(new_game
            .make_move(PlayerMove::QuoridorWallV((2, 6), "pl1".to_owned()))
            .is_ok());
        assert_eq!(new_game.current, "pl2");
        assert!(!new_game
            .make_move(PlayerMove::QuoridorWallH((3, 7), "pl2".to_owned()))
            .is_ok());
        assert_eq!(new_game.current, "pl2");
    }

    #[test]
    fn test_cpu() {
        let mut new_game = QuoridorMatch::new(&vec!["pl1".to_owned()], "pl1".to_owned());
        new_game.make_move(PlayerMove::QuoridorMove((1, 4), "pl1".to_owned()));
        let cpu_move = cpu::CpuPlayer::get_cpu_move(&new_game.game, false);
        if let PlayerMove::QuoridorWallH(v, _) = cpu_move {
            assert_eq!(v, (2, 3))
        } else {
            assert!(false)
        }
        assert_eq!(new_game.current, "pl1".to_owned());
    }
}
