mod auth;
mod errors;
mod leader_board;
mod messages;
mod quoridor;
mod state;
//internals
use errors::StateError;
use leader_board::UserLeaderBoard;
use messages::{
    ChatMessage, GuestLogin, PlayerMove, PlayerMoveResult, QuoridorMatchMeta, UserContext, UserCreate, UserLogin,
};
use state::AppState;
//std
use std::sync::Arc;
// extern creates
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use futures::{sink::SinkExt, stream::StreamExt};
use serde_json::{from_str, to_string};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

const TOKEN: &str = "auth_token";

async fn login(
    State(app_state): State<Arc<AppState>>,
    cookies: Cookies,
    Json(payload): Json<UserLogin>,
) -> Result<UserContext, StateError> {
    let mut user = app_state.user_get_with_session(&payload.email, &payload.password)?;
    cookies.add(Cookie::new(TOKEN.to_owned(), user.auth_token.to_owned()));
    user.active_match = app_state.quoridor_get_id_by_player(&user.email);
    Ok(user)
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
) -> Result<UserContext, StateError> {
    let user = app_state.user_guest_session(payload.username)?;
    cookies.add(Cookie::new(TOKEN, user.auth_token.to_owned()));
    Ok(user)
}

async fn create_user(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<UserCreate>,
) -> Result<UserContext, StateError> {
    app_state.user_create_with_session(payload.username, payload.email, payload.password)
}

async fn auth_context(State(app_state): State<Arc<AppState>>, cookies: Cookies) -> Result<UserContext, StateError> {
    let mut user = app_state.get_session(cookies.get(TOKEN))?;
    user.active_match = app_state.quoridor_get_id_by_player(&user.email);
    Ok(user)
}

async fn leaderboard(State(app_state): State<Arc<AppState>>) -> Json<Vec<UserLeaderBoard>> {
    app_state.leaderboard.lock().unwrap().get_full_leader_board().into()
}

async fn get_personal_stats(
    State(app_state): State<Arc<AppState>>,
    cookies: Cookies,
) -> Result<UserLeaderBoard, StateError> {
    let user = app_state.get_session(cookies.get(TOKEN))?;
    app_state.leaderboard.lock().unwrap().get_by_email(&user.email)
}

async fn quoridor_cpu(cookies: Cookies, State(app_state): State<Arc<AppState>>) -> Result<UserContext, StateError> {
    let mut user = app_state.get_session(cookies.get(TOKEN))?;
    user.active_match = app_state.quoridor_new_game(&vec![user.email.to_owned()]);
    Ok(user)
}

async fn quoridor_que_join(
    cookies: Cookies,
    Path(host_name): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<UserContext, StateError> {
    let mut user = app_state.get_session(cookies.get(TOKEN))?;
    if user.email == host_name {
        return Err(StateError::UnsupportedDataType("Same user".into()));
    }
    let sender = app_state
        .quoridor_que
        .lock()
        .unwrap()
        .remove(&host_name)
        .ok_or(StateError::NotFound)?;
    user.active_match = app_state.quoridor_new_game(&vec![host_name, user.email.to_owned()]);
    if let Some(game) = &user.active_match {
        match sender.send(game.to_owned()) {
            Ok(_) => return Ok(user),
            Err(_) => app_state.quoridor_drop_by_id(game),
        }
    }
    Err(StateError::ServerError)
}

async fn quoridor_que_host(cookies: Cookies, ws: WebSocketUpgrade, State(app_state): State<Arc<AppState>>) -> Response {
    let player = match app_state.get_session(cookies.get(TOKEN)) {
        Ok(player) => player.email,
        Err(error) => return error.into_response(),
    };

    ws.on_upgrade(|socket| async move {
        let (channel_send, channel_recv) = tokio::sync::oneshot::channel::<String>();
        let (mut sender, mut reciever) = socket.split();

        app_state
            .quoridor_que
            .lock()
            .unwrap()
            .insert(player.to_owned(), channel_send);

        let mut send_task = tokio::spawn(async move {
            if let Ok(game_id) = channel_recv.await {
                let _ = sender.send(game_id.into()).await;
            }
        });

        tokio::select! {
            _tx_s = (&mut send_task) => {},
            _tx_r = (&mut reciever.next()) => {
                send_task.abort()
            }
        }

        app_state.quoridor_que.lock().unwrap().remove(&player);
    })
}

async fn quoridor_que_get(
    cookies: Cookies,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<String>>, StateError> {
    app_state.get_session(cookies.get(TOKEN))?;
    let que: Json<_> = app_state
        .quoridor_que
        .lock()
        .unwrap()
        .keys()
        .map(|key| key.to_owned())
        .collect::<Vec<_>>()
        .into();
    Ok(que)
}

async fn quoridor_get_matches(
    cookies: Cookies,
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<QuoridorMatchMeta>>, StateError> {
    app_state.get_session(cookies.get(TOKEN))?;
    let data: Vec<QuoridorMatchMeta> = app_state
        .quoridor_games
        .lock()
        .unwrap()
        .iter()
        .map(|(key, (game, _))| (key.to_owned(), game.read().unwrap().clone()).into())
        .collect();
    Ok(data.into())
}

async fn join_chat(
    cookies: Cookies,
    ws: WebSocketUpgrade,
    Path(id): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Response {
    let session = app_state.get_session(cookies.get(TOKEN));
    if session.is_err() {
        return session.into_response();
    }
    let player = session.unwrap().email;

    let channel_send = if let Some(channel) = app_state.chat_channel.read().unwrap().get(&id) {
        channel.clone()
    } else {
        return StateError::NotFound.into_response();
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
                if matches!(&payload, Message::Close(_)) {
                    return;
                }
                if let Ok(message) = payload.into_text() {
                    let _ = channel_send.send(ChatMessage {
                        user: player.to_owned(),
                        message,
                        timestamp: 0,
                    });
                }
            }
        });

        tokio::select! {
            _tx_s = (&mut send_task) => {
                recv_task.abort();
            },
            _tx_r = (&mut recv_task) => {
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
    let user_context = match app_state.get_session(cookies.get(TOKEN)) {
        Ok(user_context) => user_context,
        Err(err) => return err.into_response(),
    };
    let email = user_context.email.to_owned();
    ws.on_upgrade(|mut socket: WebSocket| async move {
        let (game, channel_send) = match app_state.quoridor_get_full(&id) {
            Some(payload) => payload,
            None => return,
        };
        let game_snapshot = to_string(&game.read().unwrap().clone());
        if let Ok(msg) = game_snapshot {
            let _ = socket.send(msg.into()).await;
        }

        let mut channel_recv = channel_send.subscribe();
        let (mut sender, mut reciever) = socket.split();
        let sender_game = Arc::clone(&game);

        let mut send_task = tokio::spawn(async move {
            while let Ok(msg) = channel_recv.recv().await {
                let game_snapshot = sender_game.read().unwrap().clone();
                if let Ok(snapshot) = to_string(&game_snapshot) {
                    let _ = sender.send(snapshot.into()).await;
                    if matches!(msg, PlayerMoveResult::GameFinished) {
                        app_state
                            .leaderboard
                            .lock()
                            .unwrap()
                            .process_game(&user_context, &game_snapshot);
                        return;
                    }
                }
            }
        });

        let mut recv_task = tokio::spawn(async move {
            while let Some(Ok(msg)) = reciever.next().await {
                if matches!(&msg, Message::Close(_)) {
                    return;
                }
                if let Ok(msg) = msg.into_text() {
                    if let Ok(player_move) = from_str::<PlayerMove>(&msg) {
                        let move_result = game.write().unwrap().make_move(player_move, &email);
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
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("debug"))
        .unwrap();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter_layer)
        .init();

    let state = AppState::new_as_arc();
    let state_for_thread = state.clone();

    tokio::task::spawn(async move {
        loop {
            state_for_thread.heart_beat();
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    });

    let app = Router::new()
        .nest_service("/", ServeDir::new("static/build"))
        .route("/leaderboard", get(leaderboard))
        .route("/auth/login", post(login))
        .route("/auth/guest_login", post(login_guest))
        .route("/auth/context", get(auth_context))
        .route("/auth/stats", get(get_personal_stats))
        .route("/auth/logout", delete(logout))
        .route("/auth/register", post(create_user))
        .route("/chat/:id", get(join_chat))
        .route("/quoridor/que", get(quoridor_que_get))
        .route("/quoridor/que/join/:host_name", get(quoridor_que_join))
        .route("/quoridor/que/host", get(quoridor_que_host))
        .route("/quoridor/matches", get(quoridor_get_matches))
        .route("/quoridor/solo", get(quoridor_cpu))
        .route("/quoridor/events/:id", get(quoridor_game))
        .with_state(state)
        .layer(CookieManagerLayer::new());

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
