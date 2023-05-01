use std::cmp::Ordering;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

use crate::{
    errors::StateError,
    messages::UserContext,
    quoridor::{cpu::CPU, QuoridorMatch},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLeaderBoard {
    pub username: String,
    pub wins: i32,
    pub loses: i32,
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
        let mut board: Vec<UserLeaderBoard> = self.get().into_iter().filter(|data| data.wins > data.loses).collect();
        board.sort_unstable_by(|a, b| {
            let order = b.wins.cmp(&a.wins);
            match order {
                Ordering::Equal => (b.wins / b.loses).cmp(&(a.wins / a.loses)),
                _ => order,
            }
        });
        board.truncate(30);
        board
    }

    fn get(&self) -> Vec<UserLeaderBoard> {
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

    pub fn get_by_email(&self, email: &str) -> Result<UserLeaderBoard, StateError> {
        let record = self
            .db
            .get(email)
            .map_err(|_| StateError::ServerError)?
            .ok_or(StateError::NotFound)?;
        let serialized_record = std::str::from_utf8(&record).map_err(|_| StateError::ServerError)?;
        from_str(serialized_record).map_err(|_| StateError::ServerError)
    }

    pub fn process_game(&self, user_context: &UserContext, snapshot: &QuoridorMatch) {
        if user_context.username == "GUEST" || snapshot.contains_player(CPU) {
            return;
        }
        if let Some(winner) = &snapshot.winner {
            if &user_context.email == winner {
                self.add_win(user_context);
            } else {
                self.add_lose(user_context)
            }
        }
    }

    fn add_win(&self, user: &UserContext) {
        let record = if let Ok(mut record) = self.get_by_email(&user.email) {
            record.wins += 1;
            record
        } else {
            UserLeaderBoard {
                username: user.username.to_owned(),
                wins: 1,
                loses: 0,
            }
        };
        if let Ok(value) = to_string(&record) {
            let _ = self.db.insert(&user.email, value.as_bytes());
        }
    }

    fn add_lose(&self, user: &UserContext) {
        if user.username == "GUEST" {
            return;
        }
        let record = if let Ok(mut record) = self.get_by_email(&user.email) {
            record.loses += 1;
            record
        } else {
            UserLeaderBoard {
                username: user.username.to_owned(),
                wins: 0,
                loses: 1,
            }
        };
        if let Ok(value) = to_string(&record) {
            let _ = self.db.insert(&user.email, value.as_bytes());
        }
    }
}
