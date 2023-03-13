mod game_lobbies;
use serde_json::{from_str, to_string};
use std::sync::Arc;
mod messages;
use axum::response::Response;
use messages::{ChatID, JsonMessage, Message, PlayerMove, PlayerMoveResult, UserCreate, UserLogin};
mod auth;
mod quoridor;
mod state;
use axum::extract::ws::{WebSocket, WebSocketUpgrade};
use axum::extract::{Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use state::AppState;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
const TOKEN: &str = "auth_token";

async fn login(
    State(app_state): State<Arc<AppState>>,
    cookies: Cookies,
    Json(payload): Json<UserLogin>,
) -> Json<JsonMessage> {
    let users = app_state.users();
    let maybe_user = users.get(&payload.email, &payload.password);
    if let JsonMessage::User {
        email: _,
        username: _,
        auth_token,
    } = &maybe_user
    {
        app_state
            .sessions
            .lock()
            .expect("DEADLOCK on sessions!")
            .insert(auth_token.clone(), maybe_user.clone());
        cookies.add(Cookie::new(TOKEN.to_owned(), auth_token.to_owned()));
    }
    maybe_user.into()
}

async fn create_user(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<UserCreate>,
) -> Json<JsonMessage> {
    let users = app_state.users();
    users
        .new_user(payload.username, payload.email, payload.password)
        .into()
}

async fn create_lobby(
    State(app_state): State<Arc<AppState>>,
    cookies: Cookies,
) -> Json<JsonMessage> {
    if let Some(bearer) = cookies.get(TOKEN) {
        if let Some(JsonMessage::User {
            email,
            username: _,
            auth_token: _,
        }) = app_state
            .sessions
            .lock()
            .expect("DEADLOCK on sessions!")
            .get(bearer.value())
        {
            return app_state.new_lobby(email.to_owned()).into();
        }
    }
    JsonMessage::Unauthorized.into()
}

//chat

// #[post("/chat/sender", data = "<msg>")]
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
// let _res = queue.send(msg.into_inner());
// }

// #[get("/game_chat/<owner>")]
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

// lobbies

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

// #[post("/create_lobby", data = "<lobby_base>")]
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

// #[get("/join/<owner>")]
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

// #[get("/active_lobbies")]
// async fn get_all_lobbies(lobbies: &State<MatchLobbies>, _auth: auth::Token) -> Json<Vec<Lobby>> {
//     Json(lobbies.get_all())
// }

// #[get("/lobby_events/<owner>")]
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

// sessions

// #[post("/move/<owner>", data = "<player_move>")]
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

// #[get("/game_state/<owner>")]
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

async fn quoridor_game(
    cookies: Cookies,
    mut ws: WebSocketUpgrade,
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Response {
    ws.on_upgrade(|mut socket: WebSocket| async move {
        let player = if let Some(session) = cookies.get(TOKEN) {
            if let Some(JsonMessage::User {
                username: _,
                email,
                auth_token: _,
            }) = app_state
                .sessions
                .lock()
                .expect("DEADLOCK on sessions!")
                .get(session.value())
            {
                email.to_owned()
            } else {
                return;
            }
        } else {
            return;
        };

        if let Some(token) = cookies.get(TOKEN) {
        } else {
            return;
        }
        if let Some(game) = app_state.get_game_by_id(&id) {
            if let Ok(game_json) = to_string(&game) {
                if socket
                    .send(axum::extract::ws::Message::Text(game_json))
                    .await
                    .is_ok()
                {
                    while let Some(msg) = socket.recv().await {
                        let msg = if let Ok(msg) = msg {
                            if let Ok(msg) = msg.into_text() {
                                if let Ok(player_move) = from_str::<PlayerMove>(&msg) {
                                    app_state.make_quoridor_move(&id, player_move, &player)
                                } else {
                                    return;
                                }
                            } else {
                                return;
                            }
                        } else {
                            // client disconnected
                            return;
                        };
                        if let Ok(msg_to_send) = to_string(&msg) {
                            if socket
                                .send(axum::extract::ws::Message::Text(msg_to_send))
                                .await
                                .is_err()
                            {
                                // client disconnected
                                return;
                            }
                        } else {
                            return;
                        }
                    }
                }
            }
        }
    })
}

// #[get("/game_events/<owner>")]
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
    let state = AppState::new_as_arc();
    let state_for_thread = state.clone();

    tokio::task::spawn(async move {
        loop {
            state_for_thread.recurent_clean_up();
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    });

    let app = Router::new()
        .route("/auth/login", post(login))
        .route("/auth/register", post(create_user))
        .route("/create_lobby", get(create_lobby))
        .route("/quoridor_events/:id", get(quoridor_game))
        .with_state(state)
        .layer(CookieManagerLayer::new());

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await;
}
