extern crate chrono;
extern crate jsonwebtoken;
extern crate rocket;

pub struct ActiveSessions {
    toket: String,
    user: String,
    timestamp: i64,
}

pub struct Auth {}
pub struct AuthDB {}
