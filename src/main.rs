mod game_lobbies;
use game_lobbies::{Lobby, LobbyBase};
mod messages;
use messages::{ChatID, Message, PlayerMove, PlayerMoveResult};
mod quoridor;
mod auth;
mod state;
use state::AppState;
use axum::routing::{get, post};
use axum::{Router, Json};
use axum::extract::{Query, State};
use axum::extract::ws::WebSocket;
use tower_cookies::{Cookie, Cookies, CookieManagerLayer};

const TOKEN: &str = "qr-token";

// //chat
// // #[post("/chat/sender", data = "<msg>")]
// async fn post_message(
//     msg: Json<Message>,
//     cookies: Cookies,
//     app_state: State<ActiveGames>,
// ) {
//     todo!();
//     match &msg.id {
//         ChatID::MatchID(owner) => {
//             let lobby = app_state.get_game_by_player(&owner);
//             if !lobby.is_some() || !lobby.unwrap().contains_player(cookies.get(TOKEN).unwrap().value()) {
//                 return;
//             }
//         }
//     }
//     // let _res = queue.send(msg.into_inner());
// }

// // #[get("/game_chat/<owner>")]
// async fn session_chat(
//     owner: String,
//     queue: &State<Sender<Message>>,
//     mut end: rocket::Shutdown,
// ) -> rocket::response::stream::EventStream![] {
//     let mut rx = queue.subscribe();
//     rocket::response::stream::EventStream! {
//         loop {
//             let msg = rocket::tokio::select! {
//                 msg = rx.recv() => match msg {
//                     Ok(msg) => if msg.id == ChatID::MatchID(owner.to_owned()) {msg} else {continue},
//                     Err(rocket::tokio::sync::broadcast::error::RecvError::Closed) => break,
//                     Err(rocket::tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
//                 },
//                 _ = &mut end => break,
//             };
//             yield rocket::response::stream::Event::json(&msg);
//         }
//     }
// }

// // lobbies
// pub fn concede_active_games_by_player(
//     token: auth::Token,
//     active_games: &State<ActiveGames>,
//     game_events: &State<Sender<GenericGame>>,
// ) {
//     while let Some(mut game) = active_games.get_game_by_player(&token.user) {
//         let owner = game.owner;
//         game.make_move(PlayerMove::Concede(token.user.to_owned()));
//         let _res = game_events.send(game);
//         active_games.drop_by_owner(&owner);
//     }
// }

// // #[post("/create_lobby", data = "<lobby_base>")]
// async fn make_lobby(
//     lobby_base: Json<LobbyBase>,
//     token: auth::Token,
//     lobbies: &State<MatchLobbies>,
//     active_games: &State<ActiveGames>,
//     game_events: &State<Sender<GenericGame>>,
// ) -> Json<Option<String>> {
//     let lobby = lobby_base.into_inner();
//     if let GameType::Unknown = lobby.game {
//         lobbies.drop(&token.user);
//     } else if lobby.owner == token.user && !token.is_guest() {
//         if let Some(owner) = lobbies.new_lobby(lobby) {
//             concede_active_games_by_player(token, active_games, game_events);
//             return Json(Some(owner));
//         }
//     }
//     return Json(None);
// }

// // #[get("/join/<owner>")]
// async fn join_lobby(
//     owner: String,
//     token: auth::Token,
//     lobbies: &State<MatchLobbies>,
//     active_games: &State<ActiveGames>,
//     lobby_events: &State<Sender<Lobby>>,
//     game_events: &State<Sender<GenericGame>>,
// ) -> Json<Option<String>> {
//     if owner == quoridor::cpu::CPU {
//         return Json(active_games.create_cpu_game(&token.user, GameType::Quoridor));
//     }
//     if let Some(lobby) = lobbies.add_player_to_lobby(&owner, &token.user) {
//         concede_active_games_by_player(token, active_games, game_events);
//         if lobby.is_ready() {
//             active_games.append(&lobby);
//         }
//         let _res = lobby_events.send(lobby);
//         return Json(Some(owner.to_owned()));
//     }
//     return Json(None);
// }

// // #[get("/active_lobbies")]
// async fn get_all_lobbies(lobbies: &State<MatchLobbies>, _auth: auth::Token) -> Json<Vec<Lobby>> {
//     Json(lobbies.get_all())
// }

// // #[get("/lobby_events/<owner>")]
// async fn lobby_events(
//     _token: auth::Token,
//     owner: String,
//     queue: &State<Sender<Lobby>>,
//     mut end: rocket::Shutdown,
// ) -> rocket::response::stream::EventStream![] {
//     let mut rx = queue.subscribe();
//     rocket::response::stream::EventStream! {
//         loop {
//             let msg = rocket::tokio::select! {
//                 msg = rx.recv() => match msg {
//                     Ok(msg) => if msg.owner == owner {msg} else {continue},
//                     Err(rocket::tokio::sync::broadcast::error::RecvError::Closed) => break,
//                     Err(rocket::tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
//                 },
//                 _ = &mut end => break,
//             };
//             yield rocket::response::stream::Event::json(&msg);
//         }
//     }
// }

// // sessions
// // #[post("/move/<owner>", data = "<player_move>")]
// async fn make_move(
//     owner: String,
//     player_move: Json<PlayerMove>,
//     sessions: &State<ActiveGames>,
//     token: auth::Token,
//     queue: &State<Sender<GenericGame>>,
// ) -> Json<PlayerMoveResult> {
//     if !player_move.confirm_player(&token.user) {
//         return Json(PlayerMoveResult::Unauthorized);
//     }
//     let move_result: PlayerMoveResult = match sessions.make_move(&owner, player_move.into_inner()) {
//         Some(v) => v,
//         None => return Json(PlayerMoveResult::Unknown),
//     };
//     if let PlayerMoveResult::Ok = move_result {
//         let _ = queue.send(sessions.get_game_by_owner(&owner).unwrap());
//     }
//     Json(move_result)
// }

// // #[get("/game_state/<owner>")]
// async fn get_game_state_by_owner(
//     owner: String,
//     token: auth::Token,
//     active_sessions: &State<ActiveGames>,
// ) -> Result<Json<GenericGame>, Status> {
//     if let Some(game) = active_sessions.get_game_by_owner(&owner) {
//         if game.contains_player(&token.user) {
//             return Ok(Json(game));
//         }
//         return Err(Status::Forbidden);
//     }
//     Err(Status::NotFound)
// }

// // #[get("/game_events/<owner>")]
// async fn match_events(
//     owner: String,
//     queue: &State<Sender<GenericGame>>,
//     _auth: auth::Token,
//     mut end: rocket::Shutdown,
// ) -> rocket::response::stream::EventStream![] {
//     let mut rx = queue.subscribe();
//     rocket::response::stream::EventStream! {
//         loop {
//             let msg = rocket::tokio::select! {
//                 msg = rx.recv() => match msg {
//                     Ok(msg) => if msg.get_owner() == owner {msg} else {continue},
//                     Err(rocket::tokio::sync::broadcast::error::RecvError::Closed) => break,
//                     Err(rocket::tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
//                 },
//                 _ = &mut end => break,
//             };
//             yield rocket::response::stream::Event::json(&msg);
//         };
//     }
// }

#[tokio::main]
async fn main() {
    let app = Router::new()
        .with_state(AppState::new_as_arc())
        .layer(CookieManagerLayer::new());


    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await;
}
