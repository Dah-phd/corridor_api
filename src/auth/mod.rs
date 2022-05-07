extern crate chrono;
extern crate jsonwebtoken;
extern crate rocket;
use std::sync::Mutex;

#[derive(rocket::serde::Serialize, rocket::serde::Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum User {
    Guest(String),
    ActiveUser(String, String),
}

#[derive(Clone)]
pub struct Session {
    token: String,
    user: String,
    timestamp: i64,
}

pub struct ActiveSessions {
    secret_key: String,
    sessions: Mutex<Vec<Session>>,
}

impl ActiveSessions {
    pub fn new() -> Self {
        Self {
            secret_key: "hardcoregameengine".to_owned(),
            sessions: Mutex::new(Vec::new()),
        }
    }
    pub fn new_session(&self, user: User) -> String {
        let mut sessions_list = self.sessions.lock().unwrap();
        let exposed_vector = &mut *sessions_list;
        let username = match &user {
            User::ActiveUser(v, _) => v.clone(),
            User::Guest(v) => v.clone(),
        };
        for session in exposed_vector.iter() {
            if session.user == username {
                return session.token.clone();
            }
        }
        let token = self.get_token(user);
        exposed_vector.push(Session {
            token: token.to_owned(),
            user: "dah".to_owned(),
            timestamp: 0,
        });
        return token;
    }

    // make private after testing
    pub fn get_token(&self, user: User) -> String {
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &user,
            &jsonwebtoken::EncodingKey::from_secret(self.secret_key.as_ref()),
        );
        token.unwrap()
    }

    pub fn get_user(&self, token: &String) -> Option<String> {
        let mut sessions_list = self.sessions.lock().unwrap();
        let exposed_vector = &mut *sessions_list;
        for session in exposed_vector {
            if &session.token == token {
                return Some(session.user.to_owned());
            }
        }
        None
    }
}

pub struct Auth {}
pub struct AuthDB {}
