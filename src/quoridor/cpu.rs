use super::*;
const CPU: &str = "|CPU|";

pub struct CpuPlayer {
    game: Quoridor,
    cpu_path: Vec<(usize, usize)>,
    player_path: Vec<(usize, usize)>,
    tested_v_walls: Vec<(usize, usize)>,
    tested_h_walls: Vec<(usize, usize)>,
}

impl CpuPlayer {
    pub fn get_cpu_move(game: &Quoridor) -> PlayerMove {
        let mut instance = Self::new(game.clone());
        if instance.cpu_path.len() <= instance.player_path.len() {
            let new_position = instance.cpu_path.pop().unwrap();
            if instance.can_enemy_jump_over_cpu(new_position) {
                return PlayerMove::QuoridorMove(new_position, CPU.to_owned());
            }
        }
        todo!()
    }
    fn new(game: Quoridor) -> Self {
        Self {
            cpu_path: game.get_shortest_path(game.down_player, 0).unwrap(),
            player_path: game.get_shortest_path(game.up_player, 8).unwrap(),
            game,
            tested_v_walls: Vec::new(),
            tested_h_walls: Vec::new(),
        }
    }

    fn can_enemy_jump_over_cpu(&self, position: (usize, usize)) -> bool {
        let total_position_player = self.game.up_player.0 + self.game.up_player.1;
        let total_position_cpu = position.0 + position.1;
        let difference = if total_position_cpu > total_position_player {
            total_position_cpu - total_position_player
        } else {
            total_position_player - total_position_cpu
        };
        if difference == 1 {
            return true;
        }
        false
    }

    fn new_wall_path_len(&mut self, position: (usize, usize)) -> Option<usize> {
        if self.game.new_h_wall(position) {
            self.game.horizontal_walls.pop();
            return Some(self.game.get_shortest_path(self.game.up_player, 0).unwrap().len());
        }
        if self.game.new_v_wall(position) {
            self.game.vertical_walls.pop();
            return Some(self.game.get_shortest_path(self.game.up_player, 0).unwrap().len());
        }
        None
    }
}
