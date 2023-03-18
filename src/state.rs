use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
extern crate rand;
use crate::auth::Users;
use crate::messages::{JsonMessage, PlayerMove, PlayerMoveResult};
use crate::quoridor::QuoridorMatch;
use rand::{distributions::Alphanumeric, Rng};
use tokio::sync::broadcast;
use tower_cookies::Cookie;

const ID_LEN: usize = 8;

type QuoridorPackage = (QuoridorMatch, broadcast::Sender<QuoridorMatch>);

#[derive(Default)]
pub struct AppState {
    quoridor_games: Arc<Mutex<HashMap<String, QuoridorPackage>>>,
    pub quoridor_que: Arc<Mutex<Option<tokio::sync::oneshot::Sender<String>>>>,
    pub users: Arc<Mutex<Users>>,
    pub sessions: Arc<Mutex<HashMap<String, JsonMessage>>>,
}

impl AppState {
    pub fn new_as_arc() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn get_session(&self, cookie: Option<Cookie>) -> JsonMessage {
        if let Some(token) = cookie {
            if let Some(user) = self
                .sessions
                .lock()
                .expect("DEADLOCK on sessions!")
                .get(token.value())
            {
                return user.clone();
            }
        }
        JsonMessage::Unauthorized
    }

    pub fn users(&self) -> MutexGuard<Users> {
        self.users.lock().expect("DEADLOCK on users!")
    }

    pub fn quoridor_que_join(&self, user: String) {
        let maybe_sender = self.quoridor_que.lock().expect("DEADLOCK on quoridor_que!");
    }

    pub fn quoridor_new_game(&self, lobby: &Vec<String>) -> Option<String> {
        if lobby.is_empty() {
            return None;
        }
        let (channel, _) = broadcast::channel::<QuoridorMatch>(1);
        let mut id = generate_id(ID_LEN);
        let new_game = QuoridorMatch::new(lobby);
        let mut games = self.quoridor_games.lock().expect("DEADLOCK on games!");
        while games.contains_key(&id) {
            id = generate_id(ID_LEN)
        }
        games
            .insert(id.to_owned(), (new_game, channel))
            .map(|_| id.to_owned())
    }

    pub fn quoridor_get_state_by_id(&self, id: &str) -> Option<QuoridorMatch> {
        let games = self.quoridor_games.lock().expect("DEADLOCK on games!");
        games.get(id).map(|(game, _)| game.clone())
    }

    pub fn quoridor_get_state_by_player(&self, player: &str) -> Option<QuoridorMatch> {
        let games = self.quoridor_games.lock().expect("DEADLOCK on games!");
        games
            .iter()
            .find(|(_key, (game, _))| game.contains_player(player))
            .map(|(_key, (game, _))| game.clone())
    }

    pub fn quoridor_get_full(&self, id: &str) -> Option<QuoridorPackage> {
        self.quoridor_games
            .lock()
            .expect("DEADLOCK on games!")
            .get(id)
            .cloned()
    }

    pub fn quoridor_make_move(
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

    pub fn quoridor_drop_by_id(&self, id: &str) {
        let mut games = self.quoridor_games.lock().expect("DEADLOCK on games!");
        games.remove(id);
    }

    pub fn recurent_clean_up(&self) {
        let mut games = self.quoridor_games.lock().expect("DEADLOCK on games!");
        games.retain(|_key, (game, _)| game.get_winner().is_none() || !game.is_expaired());
        drop(games);
    }
}

fn generate_id(len: usize) -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();
    s
}
