use rocket::serde::{Deserialize, Serialize};
use std::sync::Mutex;
extern crate rand;
use crate::game_interface::{GenericGame, PlayerMove, PlayerMoveResult};
use crate::game_lobbies::Lobby;
use rand::{distributions::Alphanumeric, Rng};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "rocket::serde")]
pub enum GameType {
    Quoridor,
    Unknown,
}

impl GameType {
    pub fn get_expected_players(&self) -> usize {
        match self {
            Self::Quoridor => 2,
            _ => 0,
        }
    }
}

pub struct ActiveGames {
    list_of_games: Mutex<Vec<GenericGame>>,
}

impl ActiveGames {
    pub fn new() -> Self {
        Self {
            list_of_games: Mutex::new(Vec::new()),
        }
    }

    pub fn create_cpu_game(&self, player: &str, game_type: GameType) -> Option<String> {
        let id_len = 8;
        let mut id = generate_rand_string(id_len);
        let games = &mut *self.list_of_games.lock().unwrap();
        while games.iter().any(|x| x.get_owner() == id) {
            id = generate_rand_string(id_len)
        }
        let new_game = GenericGame::new(&vec![player.to_owned()], &id, game_type);
        if let Some(game) = new_game {
            games.push(game);
            return Some(id);
        }
        None
    }

    pub fn append(&self, lobby: &Lobby) -> bool {
        self.drop_finished();
        let games = &mut *self.list_of_games.lock().unwrap();
        let new_game = GenericGame::new(&lobby.player_list, &lobby.player_list[0], lobby.match_type);
        if new_game.is_none() {
            return false;
        }
        games.push(new_game.unwrap());
        true
    }

    pub fn get_game_by_owner(&self, owner: &String) -> Option<GenericGame> {
        let games = &mut *self.list_of_games.lock().unwrap();
        for game in games {
            if game.get_owner() == owner.to_owned() {
                game.timeout_guard(owner);
                return Some(game.clone());
            }
        }
        None
    }

    pub fn get_game_by_player(&self, player: &String) -> Option<GenericGame> {
        let games = &mut *self.list_of_games.lock().unwrap();
        for game in games {
            if game.contains_player(player) {
                game.timeout_guard(player);
                return Some(game.clone());
            }
        }
        None
    }

    pub fn make_move(&self, owner: &String, player_move: PlayerMove) -> Option<PlayerMoveResult> {
        let games = &mut *self.list_of_games.lock().unwrap();
        for game in games {
            if game.get_owner() == *owner {
                return Some(game.make_move(player_move));
            };
        }
        None
    }

    pub fn drop_by_owner(&self, owner: &String) {
        self.drop_finished();
        let games = &mut *self.list_of_games.lock().unwrap();
        games.retain(|x| &x.get_owner() != owner)
    }

    fn drop_finished(&self) {
        let games = &mut *self.list_of_games.lock().unwrap();
        games.retain(|x| x.get_winner().is_none() || !x.is_expaired())
    }
}

pub fn generate_rand_string(len: usize) -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();
    s
}
