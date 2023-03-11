use serde::{Deserialize, Serialize};
extern crate rand;

#[derive(Deserialize)]
pub struct LobbyBase {
    pub owner: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct Lobby {
    pub owner: String,
    pub player_list: Vec<String>,
    game_started: Option<String>,
    time_stamp: i64,
}

impl Lobby {
    pub fn new(lobby_base: &LobbyBase) -> Self {
        Self {
            owner: lobby_base.owner.to_owned(),
            player_list: vec![lobby_base.owner.to_owned()],
            game_started: None,
            time_stamp: chrono::Utc::now().timestamp() + 600,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.game_started.is_some()
    }
    fn prepare(&mut self) {
        self.game_started = Some(self.owner.to_owned())
    }

    pub fn expaired(&self) -> bool {
        self.time_stamp < chrono::Utc::now().timestamp()
    }
}
