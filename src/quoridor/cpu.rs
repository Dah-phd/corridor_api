use super::*;
pub const CPU: &str = "|QCPU|";

pub struct CpuPlayer {
    game: Quoridor,
    cpu_path: Vec<(usize, usize)>,
    player_path: Vec<(usize, usize)>,
}

impl CpuPlayer {
    pub fn get_cpu_move(game: &Quoridor, only_palyer_moves_allowed: bool) -> PlayerMove {
        let mut instance = Self::new(game.clone());
        let new_position = instance.cpu_path[instance.cpu_path.len() - 2];
        if !only_palyer_moves_allowed && !instance.is_cpu_closer(new_position) && instance.game.down_player_free_walls != 0 {
            let maybe_wall = instance.get_best_wall();
            if maybe_wall.is_some() {
                return maybe_wall.unwrap();
            }
        }
        return PlayerMove::QuoridorMove(new_position, CPU.to_owned());
    }

    fn is_cpu_closer(&self, position: (usize, usize)) -> bool {
        self.is_cpu_closer_or_rng() && !self.can_enemy_jump_over_cpu(position)
            || self.can_cpu_jump_over(position)
            || self.player_wins_next_turn()
    }

    fn new(game: Quoridor) -> Self {
        Self {
            cpu_path: game.get_shortest_path(game.down_player, 0).unwrap(),
            player_path: game.get_shortest_path(game.up_player, 8).unwrap(),
            game,
        }
    }

    fn get_max_from_vec_len_to_wall(&self, len_to_positions: Vec<(usize, (usize, usize))>) -> Option<(usize, (usize, usize))> {
        if len_to_positions.len() == 0 {
            return None;
        }
        let mut maxed_out = len_to_positions[0];
        for len_to_pos in len_to_positions {
            if len_to_pos.0 >= maxed_out.0 {
                maxed_out = len_to_pos;
            }
        }
        Some(maxed_out)
    }

    fn can_cpu_jump_over(&self, position: (usize, usize)) -> bool {
        self.get_difference_between_total_positions(position, self.game.up_player) == 0
    }

    fn can_enemy_jump_over_cpu(&self, position: (usize, usize)) -> bool {
        self.get_difference_between_total_positions(position, self.game.up_player) == 1
    }

    fn is_cpu_closer_or_rng(&self) -> bool {
        use rand::Rng;
        let num: usize = rand::thread_rng().gen_range(0..=1);
        self.cpu_path.len() <= self.player_path.len() + num
    }

    fn player_wins_next_turn(&self) -> bool {
        self.player_path.len() == 2
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
        for idx in 0..self.player_path.len() {
            self.add_new_hwall_path_result(self.player_path[idx], &mut path_results_h);
            self.add_new_vwall_path_result(self.player_path[idx], &mut path_results_v);
        }
        let max_h = self.get_max_from_vec_len_to_wall(path_results_h);
        let max_v = self.get_max_from_vec_len_to_wall(path_results_v);
        Self::create_move_from_wall_results(max_h, max_v)
    }

    fn create_move_from_wall_results(
        h: Option<(usize, (usize, usize))>,
        v: Option<(usize, (usize, usize))>,
    ) -> Option<PlayerMove> {
        if h.is_none() && v.is_none() {
            return None;
        } else if h.is_none() {
            return Some(PlayerMove::QuoridorWallV(v.unwrap().1, CPU.to_owned()));
        } else if v.is_none() {
            return Some(PlayerMove::QuoridorWallH(h.unwrap().1, CPU.to_owned()));
        }
        let v = v.unwrap();
        let h = h.unwrap();
        if v.0 > h.0 {
            return Some(PlayerMove::QuoridorWallV(v.1, CPU.to_owned()));
        } else {
            return Some(PlayerMove::QuoridorWallH(h.1, CPU.to_owned()));
        }
    }

    fn add_new_hwall_path_result(&mut self, position: (usize, usize), storage: &mut Vec<(usize, (usize, usize))>) {
        if self.game.new_h_wall(position) {
            storage.push((self.game.get_shortest_path(self.game.up_player, 8).unwrap().len(), position));
            self.game.horizontal_walls.pop();
        }
    }
    fn add_new_vwall_path_result(&mut self, position: (usize, usize), storage: &mut Vec<(usize, (usize, usize))>) {
        if self.game.new_v_wall(position) {
            storage.push((self.game.get_shortest_path(self.game.up_player, 8).unwrap().len(), position));
            self.game.vertical_walls.pop();
        }
    }
}
