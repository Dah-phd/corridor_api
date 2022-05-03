#[macro_use]
extern crate rocket;
use rocket::serde::{Deserialize, Serialize};
mod game_logic;
use game_logic::{print_state, Quoridor};
mod sessions;
use sessions::{GameSession, QuoridorSession};
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub enum Session {
    ActiveQuoridor(QuoridorSession),
}

impl Session {
    fn get_id(&self) -> i32 {
        match self {
            Session::ActiveQuoridor(v) => v.get_id(),
            _ => panic!("insert get_id for new Session!"),
        }
    }
}

struct ActiveSessions {
    sessions: Mutex<Vec<Session>>,
}

#[get("/quor/move")]
fn make_move(queue: &rocket::State<rocket::tokio::sync::broadcast::Sender<Session>>) {
    let _res = queue.send(Session::ActiveQuoridor(QuoridorSession::new(&vec!["dah", "pesho"], 32)));
}

#[get("/state")]
fn get_state(active_sessions: &rocket::State<ActiveSessions>) {
    let mut sessions_list = active_sessions.sessions.lock().unwrap();
    let protected_val = &mut *sessions_list;
}

#[get("/state")]
fn create_session(active_sessions: &rocket::State<ActiveSessions>) {
    let mut sessions_list = active_sessions.sessions.lock().unwrap();
    let protected_val = &mut *sessions_list;
}

#[get("/quor/events/<session>")]
async fn events(
    session: i32,
    queue: &rocket::State<rocket::tokio::sync::broadcast::Sender<Session>>,
    mut end: rocket::Shutdown,
) -> rocket::response::stream::EventStream![] {
    let mut rx = queue.subscribe();
    rocket::response::stream::EventStream! {
        loop {
            let msg = rocket::tokio::select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => if msg.get_id() == session {msg} else {continue},
                    Err(rocket::tokio::sync::broadcast::error::RecvError::Closed) => break,
                    Err(rocket::tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield rocket::response::stream::Event::json(&msg);
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![events, make_move])
        .mount("/", rocket::fs::FileServer::from(rocket::fs::relative!("static")))
        .manage(rocket::tokio::sync::broadcast::channel::<Session>(1024).0)
        .manage(ActiveSessions {
            sessions: Mutex::new(Vec::new()),
        })
}
