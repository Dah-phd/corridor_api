use bcrypt::{hash, verify, DEFAULT_COST};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

use crate::errors::StateError;
use crate::messages::UserContext;

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
    pub fn get(
        &self,
        email: &str,
        password: &str,
        token: String,
    ) -> Result<UserContext, StateError> {
        let user_data = self.is_authenticated(email, password)?;
        Ok(UserContext {
            email: email.to_owned(),
            auth_token: token,
            username: user_data.username,
            active_match: None,
        })
    }

    pub fn new_user(
        &self,
        username: String,
        email: String,
        password: String,
        token: String,
    ) -> Result<UserContext, StateError> {
        if !self.email_check.is_match(&email) {
            return Err(StateError::UnsupportedDataType("Not an email!".into()));
        }
        if username == "GUEST" {
            return Err(StateError::UnsupportedDataType("Can not use GUEST as username!".to_owned()));
        }
        if self
            .db
            .contains_key(&email)
            .map_err(|_| StateError::ServerError)?
        {
            return Err(StateError::AlreadyTaken);
        }
        let user_payload = UserData {
            username: username.to_owned(),
            password_hash: hash(password, DEFAULT_COST).map_err(|_| StateError::ServerError)?,
        };
        let user_json = to_string(&user_payload).map_err(|_| StateError::ServerError)?;
        self.db
            .insert(&email, user_json.as_bytes())
            .map_err(|_| StateError::ServerError)?;
        Ok(UserContext {
            active_match: None,
            auth_token: token,
            email,
            username,
        })
    }

    fn is_authenticated(&self, email: &str, password: &str) -> Result<UserData, StateError> {
        let record = self
            .db
            .get(email)
            .map_err(|_| StateError::ServerError)?
            .ok_or(StateError::NotFound)?;
        let serialized_user = std::str::from_utf8(&record).map_err(|_| StateError::ServerError)?;
        let user = from_str::<UserData>(serialized_user).map_err(|_| StateError::ServerError)?;
        if verify(password, &user.password_hash).map_err(|_| StateError::ServerError)? {
            return Ok(user);
        }
        Err(StateError::Unauthorized)
    }
}
