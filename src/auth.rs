use bcrypt::{hash, verify, DEFAULT_COST};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
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
}

impl Users {
    pub fn init() -> Result<Users, sled::Error> {
        let db = sled::open("users")?;
        Ok(Self { db })
    }

    pub fn get(&self, email: &str, password: &str) -> JsonMessage {
        if let Some(username) = self.is_authenticated(email, password) {
            return JsonMessage::User {
                email: email.to_owned(),
                auth_token: Self::tokenize(&username, email),
                username,
            };
        }
        JsonMessage::Unauthorized
    }

    fn tokenize(username: &str, email: &str) -> String {
        let mc = new_magic_crypt!(email, 256);
        mc.encrypt_bytes_to_base64(username)
    }

    pub fn new_user(&self, username: String, email: String, password: String) -> JsonMessage {
        if let Ok(user_exists) = self.db.contains_key(&username) {
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
                                auth_token: Self::tokenize(&username, &email),
                                email,
                                username,
                            };
                        }
                    }
                }
            }
        }
        JsonMessage::ServerErrror
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