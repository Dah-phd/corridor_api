#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};
use rocket::serde::{Deserialize, Serialize};
mod game_logic;

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct GameData {
    #[field(validate = len(..30))]
    pub id: String,
    #[field(validate = len(..20))]
    pub player_up: String,
    pub player_down: String,
}

#[get("/messages")]
async fn new_game() {}

fn main() {
    let m = if 3 > 4 { 3 } else { 4 };
    println!("{}", m);
    let mut game = game_logic::Corridor::new();
}

// #[launch]
// fn rocket() -> _ {
//     let asd = Board::new();
//     rocket::build()
//         .mount("/", routes![new_game])
//         .mount("/", FileServer::from(relative!("static")))
// }
