use serde::{Deserialize, Serialize};
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
