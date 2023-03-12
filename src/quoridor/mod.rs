extern crate a_star_traitbased;
use crate::messages::{PlayerMove, PlayerMoveResult};
pub mod cpu;
use serde::Serialize;

const AFK_CC_TIMER: i64 = 180;

#[derive(Debug, Serialize, Clone)]
pub struct Quoridor {
    up_player: (usize, usize),
    down_player: (usize, usize),
    up_player_free_walls: usize,
    down_player_free_walls: usize,
    vertical_walls: Vec<(usize, usize)>,   // (row, col)
    horizontal_walls: Vec<(usize, usize)>, // (row, col)
}

impl Quoridor {
    fn new() -> Self {
        Self {
            up_player: (0, 4),
            down_player: (8, 4),
            up_player_free_walls: 9,
            down_player_free_walls: 9,
            vertical_walls: Vec::new(),
            horizontal_walls: Vec::new(),
        }
    }

    fn get_shortest_path(
        &self,
        player: (usize, usize),
        target: usize,
    ) -> Option<Vec<(usize, usize)>> {
        return a_star_traitbased::AStar::run(self, player, (Some(target), None));
    }

    fn player_can_win(&self, start_position: (usize, usize), target: usize) -> bool {
        self.get_shortest_path(start_position, target).is_some()
    }

    fn try_moving_up_player(&mut self, new_position: (usize, usize)) -> bool {
        if self.is_move_blocked_by_wall_or_wrong(self.up_player, new_position) {
            return false;
        }
        self.up_player = new_position;
        true
    }

    fn try_moving_down_player(&mut self, new_position: (usize, usize)) -> bool {
        if self.is_move_blocked_by_wall_or_wrong(self.down_player, new_position) {
            return false;
        }
        self.down_player = new_position;
        true
    }

    fn new_h_wall(&mut self, wall: (usize, usize)) -> bool {
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

    fn new_v_wall(&mut self, wall: (usize, usize)) -> bool {
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

    fn is_move_blocked_by_wall_or_wrong(
        &self,
        start_position: (usize, usize),
        possible_path: (usize, usize),
    ) -> bool {
        if start_position.0 == possible_path.0 {
            let column_move = Self::sort_positions(possible_path.1, start_position.1);
            if column_move.1 - column_move.0 != 1 {
                return true;
            };
            for wall in &self.vertical_walls {
                if wall.0 == start_position.0 && wall.1 == column_move.0 {
                    return true;
                }
                if start_position.0 != 0
                    && wall.0 == start_position.0 - 1
                    && wall.1 == column_move.0
                {
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
        return possible_paths;
    }
}

impl a_star_traitbased::PathGenerator for Quoridor {
    fn generate_paths(&self, from_position: (usize, usize)) -> Vec<(usize, usize)> {
        self.build_possible_paths(from_position)
    }
    fn calculate_heuristic_cost(
        &self,
        position: (usize, usize),
        target: (Option<usize>, Option<usize>),
    ) -> usize {
        let target_row = target.0.unwrap();
        if position.0 > target_row {
            return position.0 - target_row;
        }
        target_row - position.0
    }

    #[allow(unused_variables)]
    fn calculate_cost(
        &self,
        current_position: (usize, usize),
        next_position: (usize, usize),
    ) -> usize {
        1
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct QuoridorMatch {
    #[serde(skip_serializing)]
    timestamp: i64,
    up_player: String,
    down_player: String,
    game: Quoridor,
    turn: usize,
    current: String,
    winner: Option<String>,
    only_player_moves_allowed: bool,
}

impl QuoridorMatch {
    pub fn new(player_list: &Vec<String>, timestamp: i64) -> Self {
        QuoridorMatch {
            up_player: player_list[0].to_owned(),
            timestamp,
            down_player: if player_list.len() >= 2 {
                player_list[1].to_owned()
            } else {
                cpu::CPU.to_owned()
            },
            game: Quoridor::new(),
            turn: 0,
            current: player_list[0].to_owned(),
            winner: None,
            only_player_moves_allowed: false,
        }
    }

    pub fn is_expaired(&self) -> bool {
        self.get_timestamp() + AFK_CC_TIMER + 30 < chrono::Utc::now().timestamp()
    }

    fn refresh_timestamp(&mut self) {
        self.set_timestamp(chrono::Utc::now().timestamp())
    }

    pub fn timeout_guard(&mut self, player: &String) {
        if self.get_timestamp() + AFK_CC_TIMER < chrono::Utc::now().timestamp()
            && &self.current != player
        {
            self.make_move(PlayerMove::Concede, &player);
        }
    }

    pub fn make_move(&mut self, player_move: PlayerMove, player: &str) -> PlayerMoveResult {
        if self.winner.is_some() {
            return PlayerMoveResult::GameFinished;
        }
        let result = match player_move {
            PlayerMove::QuoridorWallH{row, col} => self.new_h_wall(player, (row, col)),
            PlayerMove::QuoridorWallV{row, col} => self.new_v_wall(player, (row, col)),
            PlayerMove::QuoridorMove{row, col} => self.move_player(player, (row, col)),
            PlayerMove::Concede => self.concede(player),
            _ => PlayerMoveResult::Unknown,
        };
        if result.is_ok() {
            self.end_turn();
        };
        result
    }

    pub fn get_winner(&self) -> Option<String> {
        self.winner.clone()
    }

    pub fn contains_player(&self, player: &str) -> bool {
        self.up_player == player || self.down_player == player
    }

    fn get_timestamp(&self) -> i64 {
        self.timestamp
    }
    fn set_timestamp(&mut self, timestamp: i64) {
        self.timestamp = timestamp
    }

    fn get_current_player(&self) -> &String {
        &self.current
    }
}

impl QuoridorMatch {
    fn move_player(&mut self, player: &str, new_position: (usize, usize)) -> PlayerMoveResult {
        if self.current != player {
            return PlayerMoveResult::WrongPlayerTurn;
        }
        if player == self.up_player {
            if self.game.try_moving_up_player(new_position) {
                self.check_and_set_winner(&new_position, 8);
                return PlayerMoveResult::Ok;
            }
        } else if self.game.try_moving_down_player(new_position) {
            self.check_and_set_winner(&new_position, 0);
            return PlayerMoveResult::Ok;
        };
        PlayerMoveResult::Disallowed
    }

    fn concede(&mut self, player: &str) -> PlayerMoveResult {
        if self.winner.is_none() {
            if player == &self.up_player {
                self.winner = Some(self.down_player.to_owned())
            } else {
                self.winner = Some(self.up_player.to_owned())
            }
        }
        PlayerMoveResult::Ok
    }

    fn check_and_set_winner(&mut self, new_position: &(usize, usize), expected: usize) {
        if new_position.0 == expected {
            self.winner = Some(self.current.to_owned())
        }
    }

    fn new_h_wall(&mut self, player: &str, position: (usize, usize)) -> PlayerMoveResult {
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

    fn new_v_wall(&mut self, player: &str, position: (usize, usize)) -> PlayerMoveResult {
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

    fn remove_border_from_player(&mut self, player: &str) {
        if player == self.up_player {
            self.game.up_player_free_walls -= 1
        } else {
            self.game.down_player_free_walls -= 1
        }
    }

    fn end_turn(&mut self) {
        self.turn += 1;
        if self.game.up_player == self.game.down_player {
            self.only_player_moves_allowed = true;
        } else {
            self.only_player_moves_allowed = false;
            self.switch_player()
        }
        if self.current == cpu::CPU {
            self.cpu_player_move();
        }
    }

    fn switch_player(&mut self) {
        if self.current == self.up_player {
            self.current = self.down_player.to_owned()
        } else {
            self.current = self.up_player.to_owned()
        }
    }

    fn cpu_player_move(&mut self) {
        let cpu_move = cpu::CpuPlayer::get_cpu_move(&self.game, self.only_player_moves_allowed);
        self.make_move(cpu_move, cpu::CPU);
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
        let mut new_game = QuoridorMatch::new(&vec!["pl1".to_owned(), "pl2".to_owned()], 0);
        assert!(new_game
            .make_move(PlayerMove::QuoridorMove{row:1, col:4}, "pl1")
            .is_ok());
        assert_eq!(new_game.current, "pl2");
        assert!(new_game
            .make_move(PlayerMove::QuoridorMove{row:8, col:5}, "pl2")
            .is_ok());
        assert_eq!(new_game.current, "pl1");
    }

    #[test]
    fn new_match_make_borders() {
        let mut new_game = QuoridorMatch::new(&vec!["pl1".to_owned(), "pl2".to_owned()], 0);
        assert!(new_game
            .make_move(PlayerMove::QuoridorWallH{row:1, col:0}, "pl1")
            .is_ok());
        assert_eq!(new_game.current, "pl2");
        assert!(new_game
            .make_move(PlayerMove::QuoridorWallH{row:1, col:2}, "pl2")
            .is_ok());
        assert_eq!(new_game.current, "pl1");
        assert!(new_game
            .make_move(PlayerMove::QuoridorWallH{row:1, col:4}, "pl1")
            .is_ok());
        assert_eq!(new_game.current, "pl2");
        assert!(new_game
            .make_move(PlayerMove::QuoridorWallH{row:1, col:6}, "pl2")
            .is_ok());
        assert_eq!(new_game.current, "pl1");
        assert!(new_game
            .make_move(PlayerMove::QuoridorWallV{row:2, col:6}, "pl1")
            .is_ok());
        assert_eq!(new_game.current, "pl2");
        assert!(!new_game
            .make_move(PlayerMove::QuoridorWallH{row:3, col:7}, "pl2")
            .is_ok());
        assert_eq!(new_game.current, "pl2");
    }

    #[test]
    fn test_cpu() {
        let mut new_game = QuoridorMatch::new(&vec!["pl1".to_owned()], 0);
        new_game.make_move(PlayerMove::QuoridorMove{row:1, col:4}, "pl1");
        let cpu_move = cpu::CpuPlayer::get_cpu_move(&new_game.game, false);
        if let PlayerMove::QuoridorWallH{row, col} = cpu_move {
            assert_eq!((row, col), (2, 3))
        }
        assert_eq!(new_game.current, "pl1".to_owned());
    }
}
