use bcrypt::{hash, verify, DEFAULT_COST};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

use crate::messages::JsonMessage;

#[derive(Serialize, Deserialize)]
struct UserData {
    username: String,
    password_hash: String,
}

pub struct Users {
    db: sled::Db,
    email_check: Regex,
}

impl Default for Users {
    fn default() -> Self {
        Self {
            db: sled::open("users").expect("Unable to start DB!"),
            email_check:Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})")
            .expect("Regex creation should not fail!"),
        }
    }
}

impl Users {
    pub fn get(&self, email: &str, password: &str, token:String) -> JsonMessage {
        if let Some(username) = self.is_authenticated(email, password) {
            return JsonMessage::User {
                email: email.to_owned(),
                auth_token: token,
                username,
            };
        }
        JsonMessage::Unauthorized
    }

    pub fn new_user(&self, username: String, email: String, password: String, token:String) -> JsonMessage {
        if !self.email_check.is_match(&email) {
            return JsonMessage::NotAnEmail;
        }
        if let Ok(user_exists) = self.db.contains_key(&email) {
            if user_exists {
                return JsonMessage::EmailAlreadyInUse;
            }
            if let Ok(password_hash) = hash(password, DEFAULT_COST) {
                let user_payload = UserData {
                    username: username.to_owned(),
                    password_hash,
                };
                if let Ok(value) = to_string(&user_payload) {
                    if let Ok(maybe_record) = self.db.insert(&email, value.as_bytes()) {
                        if maybe_record.is_some() {
                            return JsonMessage::User {
                                auth_token: token,
                                email,
                                username,
                            };
                        }
                    }
                }
            }
        } else {
            return JsonMessage::EmailAlreadyInUse;
        }
        JsonMessage::ServerError
    }

    pub fn is_authenticated(&self, email: &str, password: &str) -> Option<String> {
        if let Ok(Some(record)) = self.db.get(email) {
            if let Ok(user_payload) = std::str::from_utf8(&record) {
                if let Ok(user) = from_str::<UserData>(user_payload) {
                    if let Ok(is_auth) = verify(password, &user.password_hash) {
                        if is_auth {
                            return Some(user.username);
                        }
                    }
                }
            }
        }
        None
    }
}