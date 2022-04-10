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
    let mut game = game_logic::Corridor::new("9801234781".to_owned(), "dah".to_owned(), "pesho".to_owned());
    game.print_state();

    if vec![3, 4].contains(&3) {
        let mut b: usize = 3;
        let mut c: usize = 4;
    }
    let mut vec = vec![1, 2, 3];
    vec.pop();
    println!("{:?}", vec)
}

// #[launch]
// fn rocket() -> _ {
//     let asd = Board::new();
//     rocket::build()
//         .mount("/", routes![new_game])
//         .mount("/", FileServer::from(relative!("static")))
// }
