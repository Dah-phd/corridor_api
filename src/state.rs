use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
extern crate rand;
use crate::auth::Users;
use crate::game_lobbies::Lobby;
use crate::messages::{JsonMessage, PlayerMove, PlayerMoveResult};
use crate::quoridor::QuoridorMatch;
use rand::{distributions::Alphanumeric, Rng};
use tokio::sync::broadcast;

const ID_LEN: usize = 8;
type QuoridorPackage = (QuoridorMatch, broadcast::Sender<QuoridorMatch>);

#[derive(Default)]
pub struct AppState {
    quoridor_games: Arc<Mutex<HashMap<String, QuoridorPackage>>>,
    lobbies: Arc<Mutex<HashMap<String, Lobby>>>,
    pub users: Arc<Mutex<Users>>,
    pub sessions: Arc<Mutex<HashMap<String, JsonMessage>>>,
}

impl AppState {
    pub fn new_as_arc() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn new_lobby(&self, email: String) -> JsonMessage {
        let mut id = generate_id(ID_LEN);
        let mut lobbies = self.lobbies.lock().expect("DEADLOCK on lobbies!");
        while lobbies.contains_key(&id) {
            id = generate_id(ID_LEN)
        }
        lobbies.insert(id.to_owned(), Lobby::new(email));
        JsonMessage::LobbyID(id)
    }

    pub fn new_quoridor_game(&self, lobby: &mut Lobby) -> Option<String> {
        if lobby.player_list.is_empty() {
            return None;
        }
        let (channel, _) = broadcast::channel::<QuoridorMatch>(1);
        let mut id = generate_id(ID_LEN);
        let new_game = QuoridorMatch::new(&lobby.player_list, chrono::Utc::now().timestamp());
        let mut games = self.quoridor_games.lock().expect("DEADLOCK on games!");
        while games.contains_key(&id) {
            id = generate_id(ID_LEN)
        }
        lobby.game_started = Some(id.to_owned());
        games
            .insert(id.to_owned(), (new_game, channel))
            .map(|_| id.to_owned())
    }

    pub fn users(&self) -> MutexGuard<Users> {
        self.users.lock().expect("DEADLOCK on users!")
    }

    pub fn get_quoridor_state_by_id(&self, id: &str) -> Option<QuoridorMatch> {
        let games = self.quoridor_games.lock().expect("DEADLOCK on games!");
        games.get(id).map(|(game, _)| game.clone())
    }

    pub fn get_quoridor_state_by_player(&self, player: &str) -> Option<QuoridorMatch> {
        let games = self.quoridor_games.lock().expect("DEADLOCK on games!");
        games
            .iter()
            .find(|(_key, (game, _))| game.contains_player(player))
            .map(|(_key, (game, _))| game.clone())
    }

    pub fn get_quoridor_channel_by_id(&self, id: &str) -> Option<QuoridorPackage> {
        self.quoridor_games
            .lock()
            .expect("DEADLOCK on games!")
            .get(id)
            .cloned()
    }

    pub fn make_quoridor_move(
        &self,
        id: &str,
        player_move: PlayerMove,
        player: &str,
    ) -> Option<PlayerMoveResult> {
        let mut games = self.quoridor_games.lock().expect("DEADLOCK on games!");
        games
            .get_mut(id)
            .map(|(game, _)| game.make_move(player_move, player))
    }

    pub fn drop_by_id(&self, id: &str) {
        let mut games = self.quoridor_games.lock().expect("DEADLOCK on games!");
        games.remove(id);
    }

    pub fn recurent_clean_up(&self) {
        let mut games = self.quoridor_games.lock().expect("DEADLOCK on games!");
        games.retain(|_key, (game, _)| game.get_winner().is_none() || !game.is_expaired())
    }
}

pub fn generate_id(len: usize) -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();
    s
}
