use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use sled::IVec;

use crate::messages::UserContext;

#[derive(Serialize, Deserialize)]
pub struct UserLeaderBoard {
    pub username: String,
    pub wins: i32,
    pub losses: i32,
}

pub struct LeaderBoard {
    db: sled::Db,
}

impl Default for LeaderBoard {
    fn default() -> Self {
        Self {
            db: sled::open("games").expect("Unable to start DB!"),
        }
    }
}

impl LeaderBoard {
    pub fn get_full_leader_board(&self) -> Vec<UserLeaderBoard> {
        let mut board:Vec<UserLeaderBoard> = self.get().into_iter().filter(|data| data.losses > data.wins).collect();
        board.sort_by(|a, b| a.wins.cmp(&b.wins));
        board
    }
 
    pub fn get(&self) -> Vec<UserLeaderBoard> {
        self.db
            .into_iter()
            .flatten()
            .filter_map(|(_, record)| {
                std::str::from_utf8(&record)
                    .ok()
                    .and_then(|data| from_str::<UserLeaderBoard>(data).ok())
            })
            .collect()
    }

    fn get_by_email(&self, email: &str) -> Option<UserLeaderBoard> {
        let record = self.db.get(email).ok()??;
        std::str::from_utf8(&record)
            .ok()
            .and_then(|data| from_str::<UserLeaderBoard>(data).ok())
    }

    pub fn add_win(&self, user: &UserContext) {
        let record = if let Some(mut record) = self.get_by_email(&user.email) {
            record.wins += 1;
            record
        } else {
            UserLeaderBoard {
                username: user.username.to_owned(),
                wins: 1,
                losses: 0,
            }
        };
        if let Ok(value) = to_string(&record) {
            let _ = self.db.insert(&user.email, value.as_bytes());
        }
    }

    pub fn add_lose(&self, user: &UserContext) {
        let record = if let Some(mut record) = self.get_by_email(&user.email) {
            record.losses += 1;
            record
        } else {
            UserLeaderBoard {
                username: user.username.to_owned(),
                wins: 0,
                losses: 1,
            }
        };
        if let Ok(value) = to_string(&record) {
            let _ = self.db.insert(&user.email, value.as_bytes());
        }
    }
}
