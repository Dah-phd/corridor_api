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

fn print_state(game: &game_logic::Corridor) {
    for row_id in 0..9 {
        let mut line = String::new();
        let mut underline = String::new();
        for col_id in 0..9 {
            if (row_id, col_id) == game.up_player || (row_id, col_id) == game.down_player {
                line.push_str("[X]");
            } else {
                line.push_str("[ ]")
            }
            if game.vertcal_borders.contains(&(row_id, col_id))
                || row_id >= 1 && game.vertcal_borders.contains(&(row_id - 1, col_id))
            {
                line.push_str("|")
            } else {
                line.push_str(" ")
            }
            if game.horizontal_borders.contains(&(row_id, col_id))
                || col_id >= 1 && game.horizontal_borders.contains(&(row_id, col_id - 1))
            {
                underline.push_str("---")
            } else {
                underline.push_str("   ")
            }
        }
        println!("{line}");
        println!("{underline}")
    }
}

fn main() {
    let mut game = game_logic::Corridor::new();
    print_state(&game);
    println!();
    println!("{}", game.move_player((1, 4), "up"));
    print_state(&game);
    println!();
    println!("{}", game.move_player((7, 4), "down"));
    print_state(&game);
    println!();
}

// #[launch]
// fn rocket() -> _ {
//     let asd = Board::new();
//     rocket::build()
//         .mount("/", routes![new_game])
//         .mount("/", FileServer::from(relative!("static")))
// }
