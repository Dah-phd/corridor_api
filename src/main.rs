#[macro_use]
extern crate rocket;
use rocket::serde::{Deserialize, Serialize};
mod game_logic;
use game_logic::{print_state, Quoridor};
mod sessions;
use sessions::{GameSession, QuoridorSession};

#[get("/quor/move")]
fn make_move(queue: &rocket::State<rocket::tokio::sync::broadcast::Sender<QuoridorSession>>) {
    let _res = queue.send(QuoridorSession::new(&vec!["dah", "pesho"], 32));
}

#[get("/state")]
fn get_state(active_sessions: Vec<QuoridorSession>) {}

#[get("/quor/events/<session>")]
async fn events(
    session: i32,
    queue: &rocket::State<rocket::tokio::sync::broadcast::Sender<QuoridorSession>>,
    mut end: rocket::Shutdown,
) -> rocket::response::stream::EventStream![] {
    let mut rx = queue.subscribe();
    rocket::response::stream::EventStream! {
        loop {
            let msg = rocket::tokio::select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => if msg.id == session {msg} else {continue},
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
    let active_sessions: Vec<QuoridorSession> = Vec::new();
    rocket::build()
        .manage(active_sessions)
        .manage(rocket::tokio::sync::broadcast::channel::<QuoridorSession>(1024).0)
        .mount("/", routes![events, make_move, get_state])
        .mount("/", rocket::fs::FileServer::from(rocket::fs::relative!("static")))
}
