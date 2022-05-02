#[macro_use]
extern crate rocket;
use rocket::serde::{Deserialize, Serialize};
mod game_logic;
use game_logic::{print_state, Corridor};
mod sessions;
use sessions::{CorridorSession, GameSession};

#[derive(Debug, Serialize, Deserialize, Clone, FromForm)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
pub struct MoveEvent {
    msg: String,
}

impl MoveEvent {
    pub fn update() -> Self {
        MoveEvent { msg: "upd".to_owned() }
    }
}

#[get("/move")]
fn make_move(queue: &rocket::State<rocket::tokio::sync::broadcast::Sender<MoveEvent>>) {
    let _res = queue.send(MoveEvent::update());
}

#[get("/status/update/<session>")]
async fn get_state(session: i64) {}

#[get("/events")]
async fn events(
    queue: &rocket::State<rocket::tokio::sync::broadcast::Sender<MoveEvent>>,
    mut end: rocket::Shutdown,
) -> rocket::response::stream::EventStream![] {
    let mut rx = queue.subscribe();
    rocket::response::stream::EventStream! {
        loop {
            let msg = rocket::tokio::select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(rocket::tokio::sync::broadcast::error::RecvError::Closed) => break,
                    Err(rocket::tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield rocket::response::stream::Event::json(&msg);
        }
    }
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
struct Trigger {
    pub message: String,
}

// #[launch]
// fn rocket() -> _ {
//     let ACTIVE_SESSIONS: Vec<CorridorSession> = Vec::new();
//     rocket::build()
//         .manage(rocket::tokio::sync::broadcast::channel::<MoveEvent>(1024).0)
//         .mount("/", routes![events, make_move])
//         .mount("/", rocket::fs::FileServer::from(rocket::fs::relative!("static")))
// }

fn main() {
    fn a_() -> bool {
        println!("runs god damn it!");
        true
    }

    if true && a_() {
        println!("works")
    }
}
