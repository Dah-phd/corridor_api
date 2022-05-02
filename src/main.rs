#[macro_use]
extern crate rocket;
use rocket::serde::{Deserialize, Serialize};
mod game_logic;
use game_logic::{print_state, Corridor};
mod sessions;
use sessions::{CorridorSession, GameSession};
use std::time::Instant;

fn make_session<Session: GameSession>(game_type: &str, player_ids: &Vec<&str>) -> Option<Session> {
    if game_type == "corridor" && player_ids.len() >= 2 {
        return Some(CorridorSession::new(&player_ids));
    }
    None
}

#[get("/status/update/<session>")]
async fn get_state(session: i64) {}

#[get("/events")]
async fn events(
    queue: &rocket::State<rocket::tokio::sync::broadcast::Sender<Corridor>>,
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

#[get("/test")]
fn board() -> String {
    let mut new_game = Corridor::new();
    let result = match rocket::serde::json::serde_json::to_string(&new_game) {
        Ok(v) => return v,
        _ => return "error".to_owned(),
    };
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
struct Trigger {
    pub message: String,
}

// #[launch]
// fn rocket() -> _ {
//     main__();
//     let ACTIVE_SESSIONS: Vec<CorridorSession> = Vec::new();
//     rocket::build()
//         .manage(rocket::tokio::sync::broadcast::channel::<Corridor>(1024).0)
//         .mount("/", routes![events, board])
//         .mount("/", rocket::fs::FileServer::from(rocket::fs::relative!("static")))
// }

fn main_() -> Corridor {
    let mut game = Corridor::new();
    print_state(&game);
    println!();
    println!("{}", game.new_border((1, 1), "h"));
    println!("{}", game.new_border((1, 0), "v"));
    println!("{}", game.new_border((2, 4), "v"));

    println!("{}", game.new_border((1, 3), "h"));
    println!("{}", game.new_border((3, 5), "h"));
    println!("{}", game.new_border((1, 7), "h"));

    println!("{}", game.new_border((2, 6), "v"));

    println!("{}", game.new_border((0, 0), "h"));

    print_state(&game);
    println!();
    println!("{}", game.move_player((7, 4), "down"));
    print_state(&game);
    println!();
    println!("{:?}", Instant::now());
    game
}

fn test<const N: usize>(s: [usize; N]) {
    println!("{:?}", s)
}
fn main() {
    let b = [1, 2, 3];
    test(b);
    let c = [1, 2, 3, 4];
    test(c);
}
