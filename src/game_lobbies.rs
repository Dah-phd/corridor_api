use crate::messages::JsonMessage;
use crate::state::AppState;
use serde::Serialize;
use std::sync::Arc;
use tower_cookies::Cookie;
extern crate rand;

#[derive(Debug, Serialize, Clone)]
pub struct Lobby {
    pub player_list: Vec<String>,
    pub game_started: Option<String>,
    time_stamp: i64,
}

impl Lobby {
    pub fn new(email: String) -> Self {
        Self {
            player_list: vec![email],
            game_started: None,
            time_stamp: chrono::Utc::now().timestamp() + 600,
        }
    }

    pub fn expaired(&self) -> bool {
        self.time_stamp < chrono::Utc::now().timestamp()
    }
}

pub fn verify_cookie(
    maybe_cookie: Option<Cookie>,
    app_state: Arc<AppState>,
) -> Option<JsonMessage> {
    if let Some(session) = maybe_cookie {
        return app_state
            .sessions
            .lock()
            .expect("DEADLOCK on sessions!")
            .get(session.value())
            .cloned();
    };
    None
}
