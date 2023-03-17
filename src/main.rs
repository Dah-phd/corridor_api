mod game_lobbies;
use serde_json::{from_str, to_string};
use std::sync::Arc;
mod messages;
use axum::response::Response;
use messages::{JsonMessage, PlayerMove, PlayerMoveResult, UserCreate, UserLogin};
mod auth;
mod quoridor;
mod state;
use axum::extract::ws::{WebSocket, WebSocketUpgrade};
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use futures::{sink::SinkExt, stream::StreamExt};
use state::AppState;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

const TOKEN: &str = "auth_token";

//support
pub fn verify_cookie(
    maybe_cookie: Option<Cookie>,
    app_state: Arc<AppState>,
) -> Option<JsonMessage> {
    if let Some(session) = maybe_cookie {
        return app_state
            .sessions
            .lock()
            .expect("DEADLOCK on sessions!")
            .get(session.value())
            .cloned();
    };
    None
}

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
    if let Some(JsonMessage::User {
        email,
        username: _,
        auth_token: _,
    }) = verify_cookie(cookies.get(TOKEN), app_state.clone())
    {
        return app_state.new_lobby(email).into();
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

async fn join_lobby(
    State(app_state): State<Arc<AppState>>,
    Path(mathc_id): Path<String>,
    cookies: Cookies,
) -> Json<JsonMessage> {
    if let Some(JsonMessage::User {
        email,
        username: _,
        auth_token: _,
    }) = verify_cookie(cookies.get(TOKEN), app_state.clone())
    {}
    todo!()
}

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

async fn quoridor_game(
    cookies: Cookies,
    ws: WebSocketUpgrade,
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Response {
    ws.on_upgrade(|mut socket: WebSocket| async move {
        let player = if let Some(JsonMessage::User {
            username: _,
            email,
            auth_token: _,
        }) = verify_cookie(cookies.get(TOKEN), app_state.clone())
        {
            email
        } else {
            return;
        };

        let channel_send = if let Some((game, sender)) = app_state.get_quoridor_channel_by_id(&id) {
            if let Ok(msg) = to_string(&game) {
                socket.send(msg.into()).await.unwrap();
            }
            sender
        } else {
            return; // also checks if the game exist!
        };

        let mut channel_recv = channel_send.subscribe();

        let (mut sender, mut reciever) = socket.split();

        // todo: add logic to send state only if move is Ok else send message only to the player that his move is forbidden;

        let mut sender_task = tokio::spawn(async move {
            while let Ok(msg) = channel_recv.recv().await {
                if let Ok(msg_to_send) = to_string(&msg) {
                    sender.send(msg_to_send.into()).await.unwrap();
                }
            }
        });

        let mut recv_task = tokio::spawn(async move {
            while let Some(Ok(msg)) = reciever.next().await {
                if let Ok(msg) = msg.into_text() {
                    if let Ok(player_move) = from_str::<PlayerMove>(&msg) {
                        if let Some(result) =
                            app_state.make_quoridor_move(&id, player_move, &player)
                        {
                            if matches!(result, PlayerMoveResult::Ok) {
                                if let Some(game) = app_state.get_quoridor_state_by_id(&id) {
                                    channel_send.send(game).expect("failed to send msg");
                                }
                            }
                        };
                    }
                }
            }
        });

        tokio::select! {
            _rv_a = (&mut sender_task) => {
                recv_task.abort();
            },
            _rv_b = (&mut recv_task) => {
                sender_task.abort();
            }
        }
    })
}

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
        .route("/join_lobby/:match_id", get(join_lobby))
        .route("/quoridor_events/:id", get(quoridor_game))
        .with_state(state)
        .layer(CookieManagerLayer::new());

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
