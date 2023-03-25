mod auth;
mod messages;
mod quoridor;
mod state;
use axum::http::StatusCode;
//internals
use messages::{
    GuestLogin, JsonMessage, PlayerMove, PlayerMoveResult, QuoridorMatchMeta, UserContext,
    UserCreate, UserLogin,
};
use state::AppState;
//std
use std::sync::Arc;
// extern creates
use axum::extract::ws::{WebSocket, WebSocketUpgrade};
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use futures::{sink::SinkExt, stream::StreamExt};
use serde_json::{from_str, to_string};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

const TOKEN: &str = "auth_token";

//support

async fn login(
    State(app_state): State<Arc<AppState>>,
    cookies: Cookies,
    Json(payload): Json<UserLogin>,
) -> Response {
    let maybe_user = app_state.user_get_with_session(&payload.email, &payload.password);
    if let JsonMessage::User { auth_token, .. } = &maybe_user {
        cookies.add(Cookie::new(TOKEN.to_owned(), auth_token.to_owned()));
    }
    maybe_user.into_response()
}

async fn logout(State(app_state): State<Arc<AppState>>, cookies: Cookies) -> StatusCode {
    let maybe_token = cookies.get(TOKEN);
    cookies.remove(Cookie::named(TOKEN));
    if let Some(token) = maybe_token {
        app_state.user_end_session(token)
    }
    StatusCode::OK
}

async fn login_guest(
    State(app_state): State<Arc<AppState>>,
    cookies: Cookies,
    Json(payload): Json<GuestLogin>,
) -> Response {
    let maybe_user = app_state.user_guest_session(payload.username);
    if let JsonMessage::User { auth_token, .. } = &maybe_user {
        cookies.add(Cookie::new(TOKEN, auth_token.to_owned()))
    };
    maybe_user.into_response()
}

async fn create_user(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<UserCreate>,
) -> Response {
    app_state
        .user_create_with_session(payload.username, payload.email, payload.password)
        .into_response()
}

async fn auth_context(
    State(app_state): State<Arc<AppState>>,
    cookies: Cookies,
) -> Json<UserContext> {
    let maybe_user = app_state.get_session(cookies.get(TOKEN));
    let mut game = JsonMessage::NotFound;
    if let JsonMessage::User { email, .. } = &maybe_user {
        if let Some(active_game) = app_state.quoridor_get_id_by_player(email) {
            game = JsonMessage::QuoridorID(active_game);
        }
    }
    UserContext {
        user: maybe_user,
        active_match: game,
    }
    .into()
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

async fn quoridor_cpu(cookies: Cookies, State(app_state): State<Arc<AppState>>) -> Response {
    if let JsonMessage::User { email, .. } = app_state.get_session(cookies.get(TOKEN)) {
        if let Some(game_id) = app_state.quoridor_new_game(&vec![email]) {
            JsonMessage::QuoridorID(game_id).into_response()
        } else {
            JsonMessage::ServerError.into_response()
        }
    } else {
        JsonMessage::Unauthorized.into_response()
    }
}

async fn quoridor_que(
    cookies: Cookies,
    ws: WebSocketUpgrade,
    State(app_state): State<Arc<AppState>>,
) -> Response {
    let session = app_state.get_session(cookies.get(TOKEN));
    let player = if let JsonMessage::User { email, .. } = session {
        email
    } else {
        return session.into_response();
    };

    ws.on_upgrade(|mut socket| async move {
        if let Some(game_id) = app_state.quoridor_que_check(player.to_owned()) {
            if let Ok(msg) = to_string(&JsonMessage::QuoridorID(game_id.to_owned())) {
                if let Err(_data) = socket.send(msg.into()).await {
                    app_state.quoridor_drop_by_id(&game_id);
                }
            }
            return;
        }

        let (sender, receiver) = tokio::sync::oneshot::channel::<String>();
        app_state.quoridor_que_join(player, sender);

        match receiver.await {
            Ok(game_id) => {
                if let Ok(msg) = to_string(&JsonMessage::QuoridorID(game_id.to_owned())) {
                    if let Err(_data) = socket.send(msg.into()).await {
                        app_state.quoridor_drop_by_id(&game_id)
                    }
                }
            }
            Err(_data) => {
                if let Ok(msg) = to_string(&JsonMessage::ServerError) {
                    let _ = socket.send(msg.into()).await;
                }
            }
        };
    })
}

async fn debug_index() -> axum::response::Html<String> {
    let markup = tokio::fs::read_to_string("src/test/index.html")
        .await
        .unwrap();
    axum::response::Html(markup)
}

async fn quoridor_get_matches(cookies: Cookies, State(app_state): State<Arc<AppState>>) {
    let session = app_state.get_session(cookies.get(TOKEN));
    if matches!(session, JsonMessage::User { .. }) {
        let data: Vec<QuoridorMatchMeta> = app_state
            .quoridor_games
            .lock()
            .unwrap()
            .iter()
            .map(|(key, (game, _))| (key.to_owned(), game.read().unwrap().clone()).into())
            .collect();
        Json::from(data).into_response()
    } else {
        session.into_response()
    };
}

async fn quoridor_game(
    cookies: Cookies,
    ws: WebSocketUpgrade,
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Response {
    let session = app_state.get_session(cookies.get(TOKEN));
    let player = if let JsonMessage::User { email, .. } = session {
        email
    } else {
        return session.into_response();
    };

    ws.on_upgrade(|mut socket: WebSocket| async move {
        let (channel_send, game) = if let Some((game, sender)) = app_state.quoridor_get_full(&id) {
            (sender, game)
        } else {
            return; // also checks if the game exist!
        };

        let game_snapshot = to_string(&game.read().unwrap().clone());
        if let Ok(msg) = game_snapshot {
            let _ = socket.send(msg.into()).await;
        }

        let mut channel_recv = channel_send.subscribe();

        let (mut sender, mut reciever) = socket.split();

        let sender_game = Arc::clone(&game);
        // todo: add logic to send state only if move is Ok else send message only to the player that his move is forbidden;

        let mut sender_task = tokio::spawn(async move {
            while let Ok(msg) = channel_recv.recv().await {
                let game_snapshot = to_string(&sender_game.read().unwrap().clone());
                if let Ok(snapshot) = game_snapshot {
                    let _ = sender.send(snapshot.into()).await;
                    if matches!(msg, PlayerMoveResult::GameFinished) {
                        return;
                    }
                }
            }
        });

        let mut recv_task = tokio::spawn(async move {
            while let Some(Ok(msg)) = reciever.next().await {
                if let Ok(msg) = msg.into_text() {
                    if let Ok(player_move) = from_str::<PlayerMove>(&msg) {
                        let move_result = game.write().unwrap().make_move(player_move, &player);
                        let _ = channel_send.send(move_result);
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
        .route("/", get(debug_index))
        .route("/auth/login", post(login))
        .route("/auth/guest_login", post(login_guest))
        .route("/auth/context", get(auth_context))
        .route("/auth/logout", delete(logout))
        .route("/auth/register", post(create_user))
        .route("/quoridor/matches", get(quoridor_get_matches))
        .route("/quoridor/solo", get(quoridor_cpu))
        .route("/quoridor/join", get(quoridor_que))
        .route("/quoridor/events/:id", get(quoridor_game))
        .with_state(state)
        .layer(CookieManagerLayer::new());

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
