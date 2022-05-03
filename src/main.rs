#[macro_use]
extern crate rocket;
use rocket::serde::{Deserialize, Serialize};
mod abstarctions;
use abstarctions::{ActiveSessions, GameSession, Room, Session, SessionRooms, SessionType};
mod quoridor;
use quoridor::{print_state, QuoridorSession};

#[get("/quor/move")]
fn make_move(queue: &rocket::State<rocket::tokio::sync::broadcast::Sender<Session>>) {
    let _res = queue.send(Session::ActiveQuoridor(QuoridorSession::new(&vec!["dah", "pesho"], 32)));
}

#[get("/state/<session>")]
fn get_state(session: i32, active_sessions: &rocket::State<ActiveSessions>) {
    let session_state = active_sessions.get_session(session);
}

#[get("/state")]
fn create_session(active_sessions: &rocket::State<ActiveSessions>) {
    let mut sessions_list = active_sessions.sessions.lock().unwrap();
    let protected_val = &mut *sessions_list;
}

#[get("/opened_rooms")]
fn get_all_rooms() {
    todo!()
}

#[get("/room_events/<owner>")]
async fn room_events(
    owner: String,
    queue: &rocket::State<rocket::tokio::sync::broadcast::Sender<Room>>,
    mut end: rocket::Shutdown,
) -> rocket::response::stream::EventStream![] {
    let mut rx = queue.subscribe();
    rocket::response::stream::EventStream! {
        loop {
            let msg = rocket::tokio::select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => if msg.owner == owner {msg} else {continue},
                    Err(rocket::tokio::sync::broadcast::error::RecvError::Closed) => break,
                    Err(rocket::tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield rocket::response::stream::Event::json(&msg);
        }
    }
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
        .manage(ActiveSessions::new())
        .manage(rocket::tokio::sync::broadcast::channel::<Room>(1024).0)
        .manage(SessionRooms::new())
}
