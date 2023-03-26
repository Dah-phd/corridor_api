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

use crate::messages::ChatMessage;

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

async fn join_chat(
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

    let channel_send = if let Some(channel) = app_state.chat_channel.read().unwrap().get(&id) {
        channel.clone()
    } else {
        return JsonMessage::NotFound.into_response();
    };

    ws.on_upgrade(|socket: WebSocket| async move {
        let mut channel_recv = channel_send.subscribe();
        let (mut sender, mut reciever) = socket.split();

        let mut send_task = tokio::spawn(async move {
            while let Ok(message) = channel_recv.recv().await {
                if let Ok(json_msg) = to_string(&message) {
                    let _ = sender.send(json_msg.into()).await;
                }
            }
        });
        let mut recv_task = tokio::spawn(async move {
            while let Some(Ok(payload)) = reciever.next().await {
                if let Ok(json_message) = payload.into_text() {
                    if let Ok(chat_message) = from_str::<ChatMessage>(&json_message) {
                        if chat_message.user == player {
                            let _ = channel_send.send(chat_message);
                        }
                    }
                }
            }
        });

        tokio::select! {
            _rv_a = (&mut send_task) => {
                recv_task.abort();
            },
            _rv_b = (&mut recv_task) => {
                send_task.abort();
            }
        }
    })
}

async fn quoridor_game(
    cookies: Cookies,
    ws: WebSocketUpgrade,
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Response {
    let session = app_state.get_session(cookies.get(TOKEN));
    let player_recv = if let JsonMessage::User { email, .. } = session {
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
        let player_send = player_recv.to_owned();

        let mut send_task = tokio::spawn(async move {
            while let Ok(msg) = channel_recv.recv().await {
                let game_snapshot = to_string(&sender_game.read().unwrap().clone());
                if let Ok(snapshot) = game_snapshot {
                    if matches!(&msg, PlayerMoveResult::Disallowed(name) if name == &player_send) {
                        continue;
                    }
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
                        let move_result =
                            game.write().unwrap().make_move(player_move, &player_recv);
                        let _ = channel_send.send(move_result);
                    }
                }
            }
        });

        tokio::select! {
            _rv_a = (&mut send_task) => {
                recv_task.abort();
            },
            _rv_b = (&mut recv_task) => {
                send_task.abort();
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
        .route("/quoridor/chat", get(join_chat))
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
