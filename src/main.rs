#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use rocket::tokio::sync::broadcast::Sender;
use rocket::State;
mod game_abstractions;
use game_abstractions::{
    ActiveMatchs, GameMatch, Lobby, LobbyBase, Match, MatchLobbies, MatchType, PlayerMove, PlayerMoveResult,
};
mod a_star_generic;
mod auth;
mod messages;
use messages::{ChatID, Message};
mod quoridor;
#[macro_use]
extern crate diesel;
mod models;

//chat
#[post("/chat/sender", data = "<msg>")]
fn post_message(msg: Json<Message>, token: auth::Token, queue: &State<Sender<Message>>, sessions: &State<ActiveMatchs>) {
    match &msg.id {
        ChatID::MatchID(owner) => {
            let lobby = sessions.get_match(&owner);
            if !lobby.is_some() || !lobby.unwrap().contains_player(&token.user) {
                return;
            }
        }
    }
    let _res = queue.send(msg.into_inner());
}

#[get("/game_chat/<owner>")]
async fn session_chat(
    owner: String,
    queue: &State<Sender<Message>>,
    mut end: rocket::Shutdown,
) -> rocket::response::stream::EventStream![] {
    let mut rx = queue.subscribe();
    rocket::response::stream::EventStream! {
        loop {
            let msg = rocket::tokio::select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => if msg.id == ChatID::MatchID(owner.to_owned()) {msg} else {continue},
                    Err(rocket::tokio::sync::broadcast::error::RecvError::Closed) => break,
                    Err(rocket::tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield rocket::response::stream::Event::json(&msg);
        }
    }
}

// lobbies
#[post("/create_lobby", data = "<lobby>")]
fn make_lobby(lobby: Json<LobbyBase>, token: auth::Token, lobbies: &State<MatchLobbies>) -> Json<Option<String>> {
    if lobby.owner == token.user && !token.is_guest() {
        if let Some(owner) = lobbies.new_lobby(lobby.into_inner()) {
            return Json(Some(owner));
        }
    }
    return Json(None);
}

#[get("/join/<owner>")]
fn join_lobby(
    owner: String,
    token: auth::Token,
    lobbies: &State<MatchLobbies>,
    sessions: &State<ActiveMatchs>,
    lobby_queue: &State<Sender<Lobby>>,
) -> Json<Option<String>> {
    if owner == quoridor::cpu::CPU {
        return Json(sessions.create_cpu_game(&token.user, MatchType::Quoridor));
    }
    if let Some(lobby) = lobbies.add_player_to_lobby(&owner, &token.user) {
        if lobby.is_ready() {
            sessions.append(&lobby);
        }
        let _res = lobby_queue.send(lobby);
        return Json(Some(owner.to_owned()));
    }
    return Json(None);
}

#[get("/active_lobbies")]
fn get_all_lobbies(lobbies: &State<MatchLobbies>, _auth: auth::Token) -> Json<Vec<Lobby>> {
    Json(lobbies.get_all())
}

#[get("/lobby_events/<owner>")]
async fn lobby_events(
    owner: String,
    queue: &State<Sender<Lobby>>,
    mut end: rocket::Shutdown,
) -> rocket::response::stream::EventStream![] {
    let mut rx = queue.subscribe();
    rocket::response::stream::EventStream! {
        loop {
            let msg = rocket::tokio::select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => if msg.owner == owner {msg} else {continue},
                    Err(rocket::tokio::sync::broadcast::error::RecvError::Closed) => break,
                    Err(rocket::tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };
            yield rocket::response::stream::Event::json(&msg);
        }
    }
}

// sessions
#[post("/move/<owner>", data = "<player_move>")]
fn make_move(
    owner: String,
    player_move: Json<PlayerMove>,
    sessions: &State<ActiveMatchs>,
    token: auth::Token,
    queue: &State<Sender<Match>>,
) -> Json<PlayerMoveResult> {
    if !player_move.confirm_player(&token.user) {
        return Json(PlayerMoveResult::Unauthorized);
    }
    let move_result: PlayerMoveResult = match sessions.make_move(&owner, player_move.into_inner()) {
        Some(v) => v,
        None => return Json(PlayerMoveResult::Unknown),
    };
    if let PlayerMoveResult::Ok = move_result {
        let _ = queue.send(sessions.get_match(&owner).unwrap());
    }
    Json(move_result)
}

#[get("/game_state/<owner>")]
fn get_game_state_by_owner(owner: String, _auth: auth::Token, active_sessions: &State<ActiveMatchs>) -> Json<Match> {
    let session_state = active_sessions.get_match(&owner);
    match session_state {
        None => Json(Match::NotFound),
        Some(active_session) => Json(active_session),
    }
}

#[get("/game_events/<owner>")]
async fn match_events(
    owner: String,
    queue: &State<Sender<Match>>,
    _auth: auth::Token,
    mut end: rocket::Shutdown,
) -> rocket::response::stream::EventStream![] {
    let mut rx = queue.subscribe();
    rocket::response::stream::EventStream! {
        loop {
            let msg = rocket::tokio::select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => if msg.get_owner() == owner {msg} else {continue},
                    Err(rocket::tokio::sync::broadcast::error::RecvError::Closed) => break,
                    Err(rocket::tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };
            yield rocket::response::stream::Event::json(&msg);
        };
    }
}

#[get("/quoridor-test")]
fn test_quoridor_board() -> Json<Match> {
    let pl_ls = vec!["a".to_owned(), "b".to_owned()];
    let mut quoridor = quoridor::QuoridorMatch::new(&pl_ls, "a".to_owned());
    quoridor.make_move(PlayerMove::QuoridorWallH((2, 2), "a".to_owned()));
    quoridor.make_move(PlayerMove::QuoridorWallV((5, 5), "b".to_owned()));
    quoridor.make_move(PlayerMove::QuoridorMove((1, 4), "a".to_owned()));
    return Json(Match::ActiveQuoridor(quoridor));
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                test_quoridor_board,
                auth::login,
                auth::register,
                auth::get_user_name_from_token,
                post_message,
                make_move,
                session_chat,
                match_events,
                get_game_state_by_owner,
                get_all_lobbies,
                join_lobby,
                make_lobby,
                lobby_events,
            ],
        )
        .mount("/", rocket::fs::FileServer::from(rocket::fs::relative!("static/build")))
        .register("/", catchers![auth::forbidden])
        .manage(rocket::tokio::sync::broadcast::channel::<Message>(1024).0)
        .manage(rocket::tokio::sync::broadcast::channel::<Match>(1024).0)
        .manage(ActiveMatchs::new())
        .manage(rocket::tokio::sync::broadcast::channel::<Lobby>(1024).0)
        .manage(MatchLobbies::new())
        .manage(models::DBLink::new("./db.sqlite3"))
        .manage(auth::AuthTokenServices::new())
}
