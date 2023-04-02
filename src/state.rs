use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
extern crate rand;
use crate::auth::Users;
use crate::messages::{ChatMessage, JsonMessage, PlayerMoveResult};
use crate::quoridor::QuoridorMatch;
use rand::{distributions::Alphanumeric, Rng};
use regex::Regex;
use tokio::sync::broadcast;
use tower_cookies::Cookie;

const ID_LEN: usize = 8;
const TOKEN_LEN: usize = 16;
const SECONDS_IN_DAY: i64 = 24 * 60 * 60;

type TimeStamp = i64;
type QuoridorPackage = (
    Arc<RwLock<QuoridorMatch>>,
    broadcast::Sender<PlayerMoveResult>,
);
type QuoridorQue = Arc<Mutex<HashMap<String, tokio::sync::oneshot::Sender<String>>>>;

#[derive(Default)]
pub struct AppState {
    pub quoridor_games: Arc<Mutex<HashMap<String, QuoridorPackage>>>,
    pub quoridor_que: QuoridorQue,
    pub chat_channel: Arc<RwLock<HashMap<String, broadcast::Sender<ChatMessage>>>>,
    pub users: Arc<Mutex<Users>>,
    sessions: Arc<Mutex<HashMap<String, (JsonMessage, TimeStamp)>>>,
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
        if sessions.iter().any(
            |(_, (user, ..))| matches!(&user, JsonMessage::User { email, ..} if email == &username),
        ) {
            return JsonMessage::AlreadyTaken;
        }
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

    pub fn quoridor_new_game(&self, lobby: &Vec<String>) -> Option<String> {
        if lobby.is_empty() {
            return None;
        }
        let channel = broadcast::channel::<PlayerMoveResult>(1).0;
        let mut id = generate_id(ID_LEN);
        let new_game = Arc::new(RwLock::new(QuoridorMatch::new(lobby)));
        let mut games = self.quoridor_games.lock().unwrap();
        while games.contains_key(&id) {
            id = generate_id(ID_LEN)
        }
        games.insert(id.to_owned(), (new_game, channel));
        drop(games);
        self.chat_channel
            .write()
            .unwrap()
            .insert(id.to_owned(), broadcast::channel::<ChatMessage>(50).0);
        Some(id)
    }

    pub fn quoridor_get_id_by_player(&self, player: &str) -> Option<String> {
        let games = self.quoridor_games.lock().unwrap();
        games
            .iter()
            .find(|(_key, (game, _))| game.read().unwrap().contains_player(player))
            .map(|(key, _game_package)| (key.clone()))
    }

    pub fn quoridor_get_full(&self, id: &str) -> Option<QuoridorPackage> {
        self.quoridor_games.lock().unwrap().get(id).cloned()
    }

    pub fn quoridor_drop_by_id(&self, id: &str) {
        self.quoridor_games.lock().unwrap().remove(id);
    }

    pub fn recurent_clean_up(&self) {
        let mut chats_to_drop = Vec::new();
        let mut games = self.quoridor_games.lock().unwrap();
        games.retain(|key, (game, sender)| {
            let mut game = game.write().unwrap();
            game.timeout_guard();
            let _ = sender.send(PlayerMoveResult::Ok);
            if game.get_winner().is_some() {
                chats_to_drop.push(key.to_owned());
                false
            } else {
                true
            }
        });
        drop(games);
        self.sessions
            .lock()
            .unwrap()
            .retain(|_, (_, stamp)| *stamp > chrono::Utc::now().timestamp() - 7 * SECONDS_IN_DAY);
        self.chat_channel
            .write()
            .unwrap()
            .retain(|key, _| !chats_to_drop.contains(key));
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
