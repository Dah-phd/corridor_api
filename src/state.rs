use std::collections::HashMap;
use std::sync::{Arc, Mutex};
extern crate rand;
use crate::auth::Users;
use crate::messages::{JsonMessage, PlayerMove, PlayerMoveResult};
use crate::quoridor::QuoridorMatch;
use rand::{distributions::Alphanumeric, Rng};
use regex::Regex;
use tokio::sync::broadcast;
use tower_cookies::Cookie;

const ID_LEN: usize = 8;
const TOKEN_LEN: usize = 16;
const SECONDS_IN_DAY: i64 = 24 * 60 * 60;

type TimeStamp = i64;
type QuoridorPackage = (QuoridorMatch, broadcast::Sender<QuoridorMatch>);
type QuoridorQue = Arc<Mutex<Vec<(String, tokio::sync::oneshot::Sender<String>)>>>;

#[derive(Default)]
pub struct AppState {
    quoridor_games: Arc<Mutex<HashMap<String, QuoridorPackage>>>,
    pub quoridor_que: QuoridorQue,
    pub users: Arc<Mutex<Users>>,
    sessions: Arc<Mutex<HashMap<String, (JsonMessage, TimeStamp)>>>,
    pub guests: Arc<Mutex<HashMap<String, String>>>,
}

impl AppState {
    pub fn new_as_arc() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn user_create_with_session(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> JsonMessage {
        let mut token = generate_id(TOKEN_LEN);
        let mut sessions = self.sessions.lock().unwrap();
        while sessions.contains_key(&token) {
            token = generate_id(TOKEN_LEN)
        }
        let user = self
            .users
            .lock()
            .unwrap()
            .new_user(username, email, password, token.to_owned());
        if let JsonMessage::User { .. } = user {
            sessions.insert(token, (user.clone(), chrono::Utc::now().timestamp()));
        }
        user
    }

    pub fn user_get_with_session(&self, email: &str, password: &str) -> JsonMessage {
        let mut token = generate_id(TOKEN_LEN);
        let mut sessions = self.sessions.lock().unwrap();
        while sessions.contains_key(&token) {
            token = generate_id(TOKEN_LEN)
        }
        let user = self
            .users
            .lock()
            .unwrap()
            .get(email, password, token.to_owned());
        if let JsonMessage::User { .. } = user {
            sessions.insert(token, (user.clone(), chrono::Utc::now().timestamp()));
        }
        user
    }

    pub fn user_end_session(&self, token: Cookie) {
        self.sessions
            .lock()
            .expect("DEADLOCK in sessions!")
            .remove(token.value());
    }

    pub fn user_guest_session(&self, username: String) -> JsonMessage {
        if Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
        )
        .unwrap()
        .is_match(&username)
        {
            return JsonMessage::ShouldNotBeEmail;
        }
        let mut sessions = self.sessions.lock().unwrap();
        if sessions.iter().any(|(_, (game, ..))| {
            if let JsonMessage::User { email, .. } = game {
                email == &username
            } else {
                false
            }
        }) {}
        let mut token = generate_id(TOKEN_LEN);
        while sessions.contains_key(&token) {
            token = generate_id(TOKEN_LEN)
        }
        let user = JsonMessage::User {
            email: username,
            username: "GUEST".to_owned(),
            auth_token: token.to_owned(),
        };
        sessions.insert(
            token,
            (
                user.clone(),
                chrono::Utc::now().timestamp() - SECONDS_IN_DAY * 6,
            ),
        );
        user
    }

    pub fn get_session(&self, cookie: Option<Cookie>) -> JsonMessage {
        if let Some(token) = cookie {
            if let Some((user, stamp)) = self.sessions.lock().unwrap().get_mut(token.value()) {
                *stamp = chrono::Utc::now().timestamp();
                return user.clone();
            }
        }
        JsonMessage::Unauthorized
    }

    pub fn quoridor_que_check(&self, player: String) -> Option<String> {
        let mut que = self.quoridor_que.lock().unwrap();
        if let Some((qued_player, sender)) = que.drain(0..1).next() {
            if let Some(game_id) = self.quoridor_new_game(&vec![qued_player, player]) {
                if let Ok(..) = sender.send(game_id.to_owned()) {
                    return Some(game_id);
                }
            }
        }
        None
    }

    pub fn quoridor_que_join(
        &self,
        new_peer: String,
        new_sender: tokio::sync::oneshot::Sender<String>,
    ) {
        self.quoridor_que
            .lock()
            .unwrap()
            .push((new_peer, new_sender));
    }

    pub fn quoridor_new_game(&self, lobby: &Vec<String>) -> Option<String> {
        if lobby.is_empty() {
            return None;
        }
        let (channel, _) = broadcast::channel::<QuoridorMatch>(1);
        let mut id = generate_id(ID_LEN);
        let new_game = QuoridorMatch::new(lobby);
        let mut games = self.quoridor_games.lock().unwrap();
        while games.contains_key(&id) {
            id = generate_id(ID_LEN)
        }
        if games.insert(id.to_owned(), (new_game, channel)).is_some() {
            None
        } else {
            Some(id)
        }
    }

    pub fn quoridor_get_state_by_id(&self, id: &str) -> Option<QuoridorMatch> {
        let games = self.quoridor_games.lock().unwrap();
        games.get(id).map(|(game, _)| game.clone())
    }

    pub fn quoridor_get_state_by_player(&self, player: &str) -> Option<String> {
        let games = self.quoridor_games.lock().unwrap();
        games
            .iter()
            .find(|(_key, (game, _))| game.contains_player(player))
            .map(|(key, _game_package)| (key.clone()))
    }

    pub fn quoridor_get_full(&self, id: &str) -> Option<QuoridorPackage> {
        self.quoridor_games.lock().unwrap().get(id).cloned()
    }

    pub fn quoridor_make_move(
        &self,
        id: &str,
        player_move: PlayerMove,
        player: &str,
    ) -> Option<PlayerMoveResult> {
        self.quoridor_games
            .lock()
            .unwrap()
            .get_mut(id)
            .map(|(game, _)| game.make_move(player_move, player))
    }

    pub fn quoridor_drop_by_id(&self, id: &str) {
        self.quoridor_games.lock().unwrap().remove(id);
    }

    pub fn recurent_clean_up(&self) {
        let mut games = self.quoridor_games.lock().unwrap();
        games.retain(|_key, (game, sender)| {
            game.timeout_guard();
            let _ = sender.send(game.clone());
            game.get_winner().is_none()
        });
        drop(games);
        let mut sessions = self.sessions.lock().unwrap();
        sessions
            .retain(|_, (_, stamp)| *stamp > chrono::Utc::now().timestamp() - 7 * SECONDS_IN_DAY);
        drop(sessions)
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
