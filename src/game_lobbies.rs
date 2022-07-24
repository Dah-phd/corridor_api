use crate::game_matches::GameType;
use rocket::serde::{Deserialize, Serialize};
use std::sync::Mutex;
extern crate rand;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LobbyBase {
    pub owner: String,
    pub game: GameType,
}

#[derive(Debug, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Lobby {
    pub owner: String,
    pub match_type: GameType,
    pub player_list: Vec<String>,
    game_started: Option<String>,
    time_stamp: i64,
}

impl Lobby {
    pub fn new(lobby_base: &LobbyBase) -> Self {
        Self {
            owner: lobby_base.owner.to_owned(),
            match_type: lobby_base.game,
            player_list: vec![lobby_base.owner.to_owned()],
            game_started: None,
            time_stamp: chrono::Utc::now().timestamp() + 600,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.game_started.is_some()
    }
    fn prepare(&mut self) {
        if self.match_type.get_expected_players() == self.player_list.len() {
            self.game_started = Some(self.owner.to_owned())
        }
    }

    pub fn expaired(&self) -> bool {
        self.time_stamp < chrono::Utc::now().timestamp()
    }
}

pub struct MatchLobbies {
    pub lobbies: Mutex<Vec<Lobby>>,
}

impl MatchLobbies {
    pub fn new() -> Self {
        Self {
            lobbies: Mutex::new(Vec::new()),
        }
    }

    fn drop_expaired_and_started(&self) {
        let lobbies = &mut *self.lobbies.lock().unwrap();
        lobbies.retain(|x| !x.expaired() && x.game_started.is_none());
    }

    pub fn get_all(&self) -> Vec<Lobby> {
        self.drop_expaired_and_started();
        return self.lobbies.lock().unwrap().clone().to_vec();
    }

    pub fn new_lobby(&self, lobby_base: LobbyBase) -> Option<String> {
        self.drop_expaired_and_started();
        let lobbies = &mut *self.lobbies.lock().unwrap();
        for lobby in lobbies.iter() {
            if lobby.owner == lobby_base.owner {
                return None;
            }
        }
        lobbies.push(Lobby::new(&lobby_base));
        Some(lobby_base.owner.to_owned())
    }

    pub fn add_player_to_lobby(&self, lobby_owner: &String, player: &String) -> Option<Lobby> {
        self.drop_expaired_and_started();
        let lobbies = &mut *self.lobbies.lock().unwrap();
        for lobby in lobbies {
            if &lobby.owner == lobby_owner && !lobby.player_list.contains(player) {
                lobby.player_list.push(player.to_owned());
                lobby.prepare();
                return Some(lobby.clone());
            }
        }
        None
    }

    pub fn drop(&self, lobby_owner: &String) {
        let lobbies = &mut *self.lobbies.lock().unwrap();
        lobbies.retain(|x| &x.owner != lobby_owner);
    }
}
