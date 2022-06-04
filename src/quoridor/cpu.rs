use super::*;
use std::collections::HashMap;
pub const CPU: &str = "|CPU|";

pub struct CpuPlayer {
    game: Quoridor,
    cpu_path: Vec<(usize, usize)>,
    player_path: Vec<(usize, usize)>,
}

impl CpuPlayer {
    pub fn get_cpu_move(game: &Quoridor, only_palyer_moves_allowed: bool) -> PlayerMove {
        let mut instance = Self::new(game.clone());
        if instance.cpu_path.len() <= instance.player_path.len() {
            let new_position = instance.cpu_path.pop().unwrap();
            if instance.can_enemy_jump_over_cpu(new_position) {
                return PlayerMove::QuoridorMove(new_position, CPU.to_owned());
            }
        }
        if !only_palyer_moves_allowed {
            let maybe_wall = instance.get_best_wall();
            if maybe_wall.is_some() {
                return maybe_wall.unwrap();
            }
        }
        return PlayerMove::QuoridorMove(instance.cpu_path.pop().unwrap(), CPU.to_owned());
    }

    fn new(game: Quoridor) -> Self {
        Self {
            cpu_path: game.get_shortest_path(game.down_player, 0).unwrap(),
            player_path: game.get_shortest_path(game.up_player, 8).unwrap(),
            game,
        }
    }

    fn can_enemy_jump_over_cpu(&self, position: (usize, usize)) -> bool {
        self.get_difference_between_total_positions(position, self.game.up_player) == 1
    }

    fn get_difference_between_total_positions(&self, position_x: (usize, usize), position_y: (usize, usize)) -> usize {
        let x = position_x.0 + position_x.1;
        let y = position_y.0 + position_y.1;
        if x > y {
            x - y
        } else {
            y - x
        }
    }

    fn get_best_wall(&mut self) -> Option<PlayerMove> {
        let mut path_results_h: Vec<(usize, (usize, usize))> = Vec::new();
        let mut path_results_v: Vec<(usize, (usize, usize))> = Vec::new();
        for idx in 0..self.player_path.len() - 1 {
            self.add_new_hwall_path_result(self.player_path[idx], &mut path_results_h);
            self.add_new_vwall_path_result(self.player_path[idx], &mut path_results_v);
        }
        let max_h = path_results_h.iter().max_by(|a, b| a.cmp(&b));
        let max_v = path_results_v.iter().max_by(|a, b| a.cmp(&b));
        if max_h.is_none() && max_v.is_none() {
            return None;
        }
        if max_h.is_none() {
            return Some(PlayerMove::QuoridorWallV(max_v.unwrap().1, CPU.to_owned()));
        }
        if max_v.is_none() {
            return Some(PlayerMove::QuoridorWallH(max_h.unwrap().1, CPU.to_owned()));
        }
        let max_v = max_v.unwrap();
        let max_h = max_h.unwrap();
        if max_v.0 > max_h.0 {
            return Some(PlayerMove::QuoridorWallV(max_v.1, CPU.to_owned()));
        } else {
            return Some(PlayerMove::QuoridorWallH(max_h.1, CPU.to_owned()));
        }
    }

    fn add_new_hwall_path_result(&mut self, position: (usize, usize), storage: &mut Vec<(usize, (usize, usize))>) {
        if self.game.new_h_wall(position) {
            self.game.horizontal_walls.pop();
            storage.push((self.game.get_shortest_path(self.game.up_player, 0).unwrap().len(), position));
        }
    }
    fn add_new_vwall_path_result(&mut self, position: (usize, usize), storage: &mut Vec<(usize, (usize, usize))>) {
        if self.game.new_v_wall(position) {
            self.game.vertical_walls.pop();
            storage.push((self.game.get_shortest_path(self.game.up_player, 0).unwrap().len(), position));
        }
    }
}
