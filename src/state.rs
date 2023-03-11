use std::sync::{Mutex, Arc};
extern crate rand;
use crate::messages::{PlayerMove, PlayerMoveResult};
use crate::game_lobbies::Lobby;
use crate::quoridor::QuoridorMatch;
use rand::{distributions::Alphanumeric, Rng};

pub struct AppState {
    games: Arc<Mutex<Vec<QuoridorMatch>>>,
    lobbies: Arc<Mutex<Vec<Lobby>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            games: Arc::new(Mutex::new(vec![])),
            lobbies: Arc::new(Mutex::new(vec![]))
        }
    }

    pub fn new_as_arc() -> Arc<Self> {
        return Arc::new(Self::new())
    }

    pub fn create_cpu_game(&self, player: &str) -> Option<String> {
        let id_len = 8;
        let mut id = generate_rand_string(id_len);
        let mut games = self.games.lock().unwrap();
        while games.iter().any(|x| x.owner == id) {
            id = generate_rand_string(id_len)
        }
        let new_game = QuoridorMatch::new(&vec![player.to_owned()], id.to_owned(), chrono::Utc::now().timestamp());
        games.push(new_game);
        Some(id)
    }

    pub fn append(&self, lobby: &Lobby) -> bool {
        self.drop_finished();
        let mut games = self.games.lock().unwrap();
        let new_game = QuoridorMatch::new(&lobby.player_list, lobby.player_list[0].to_owned(), chrono::Utc::now().timestamp());
        games.push(new_game);
        true
    }

    pub fn get_game_by_owner(&self, owner: &String) -> Option<QuoridorMatch> {
        let mut games = self.games.lock().unwrap();
        for game in games.iter_mut() {
            if game.owner == owner.to_owned() {
                game.timeout_guard(owner);
                return Some(game.clone());
            }
        }
        None
    }

    pub fn get_game_by_player(&self, player: &String) -> Option<QuoridorMatch> {
        let mut games = self.games.lock().unwrap();
        for game in games.iter_mut() {
            if game.contains_player(player) {
                game.timeout_guard(player);
                return Some(game.clone());
            }
        }
        None
    }

    pub fn make_move(&self, owner: &String, player_move: PlayerMove) -> Option<PlayerMoveResult> {
        let mut games = self.games.lock().unwrap();
        for game in games.iter_mut() {
            if game.owner == *owner {
                return Some(game.make_move(player_move));
            };
        }
        None
    }

    pub fn drop_by_owner(&self, owner: &String) {
        self.drop_finished();
        let mut games = self.games.lock().unwrap();
        games.retain(|x| &x.owner != owner)
    }

    fn drop_finished(&self) {
        let mut games = self.games.lock().unwrap();
        games.retain(|game| game.get_winner().is_none() || !game.is_expaired())
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
