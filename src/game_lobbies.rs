use serde::Serialize;
extern crate rand;

#[derive(Debug, Serialize, Clone)]
pub struct Lobby {
    pub player_list: Vec<String>,
    time_stamp: i64,
}

impl Lobby {
    pub fn new(email: String) -> Self {
        Self {
            player_list: vec![email],
            time_stamp: chrono::Utc::now().timestamp() + 600,
        }
    }

    pub fn is_expaired(&self) -> bool {
        self.time_stamp < chrono::Utc::now().timestamp()
    }
}
