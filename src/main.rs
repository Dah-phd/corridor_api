#[macro_use]
extern crate rocket;
use rocket::serde::{Deserialize, Serialize};
mod abstarctions;
use abstarctions::{ActiveMatchs, ChatID, Match, MatchRooms, MatchType, Messages, PlayerMove, PlayerMoveResult, Room};
mod auth;
mod quoridor;
#[macro_use]
extern crate diesel;
mod models;

//general

#[post("/chat/sender", data = "<msg>")]
fn post_message(
    msg: rocket::serde::json::Json<Messages>,
    token: auth::Token,
    queue: &rocket::State<rocket::tokio::sync::broadcast::Sender<Messages>>,
) {
    let _res = queue.send(msg.into_inner());
}

// rooms

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct RoomBase {
    owner: String,
    game: MatchType,
}

#[post("/create_room", data = "<room>")]
fn make_room(room: rocket::serde::json::Json<RoomBase>, rooms: &rocket::State<MatchRooms>) -> rocket::serde::json::Json<bool> {
    rocket::serde::json::Json(rooms.new_room(&room.owner, room.game))
}

#[get("/join/<owner>")]
fn join_room(
    owner: String,
    token: auth::Token,
    rooms: &rocket::State<MatchRooms>,
    room_queue: &rocket::State<rocket::tokio::sync::broadcast::Sender<Room>>,
) {
    if !token.is_active() {
        return;
    }
    if rooms.add_player(owner.to_owned(), token.user.to_owned()) {
        match rooms.get_by_owner(&owner) {
            Some(room) => {
                let _res = room_queue.send(room);
            }
            _ => (),
        };
    }
}

#[get("/opened_rooms")]
fn get_all_rooms(rooms: &rocket::State<MatchRooms>, token: auth::Token) -> rocket::serde::json::Json<Vec<Room>> {
    rocket::serde::json::Json(rooms.get_all())
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct RoomToMatch(Option<i32>);

#[get("/start_game")]
fn room_to_session(
    token: auth::Token,
    rooms: &rocket::State<MatchRooms>,
    room_queue: &rocket::State<rocket::tokio::sync::broadcast::Sender<Room>>,
    sessions: &rocket::State<ActiveMatchs>,
) {
    if !token.is_active() {}
    let owner = token.user;
    let maybe_room = rooms.get_by_owner(&owner);
    match maybe_room {
        Some(mut room) => {
            room.game_started = true;
            if sessions.append(&room.player_list, room.match_type) {
                let _res = room_queue.send(room);
            };
        }
        None => (),
    }
}

#[get("/room_chat/<room_owner>")]
async fn room_chat(
    room_owner: String,
    rooms: &rocket::State<MatchRooms>,
    queue: &rocket::State<rocket::tokio::sync::broadcast::Sender<Messages>>,
    token: auth::Token,
    mut end: rocket::Shutdown,
) -> rocket::response::stream::EventStream![] {
    let room = rooms.get_by_owner(&room_owner);
    // match room {
    //     _ => rocket::response::Redirect::to();
    // }
    let mut rx = queue.subscribe();
    rocket::response::stream::EventStream! {
        loop {
            let msg = rocket::tokio::select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => if msg.id == ChatID::RoomID(room_owner.to_owned()) {msg} else {continue},
                    Err(rocket::tokio::sync::broadcast::error::RecvError::Closed) => break,
                    Err(rocket::tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield rocket::response::stream::Event::json(&msg);
        }
    }
}

#[get("/room_events/<owner>")]
async fn room_events<'a>(
    owner: String,
    queue: &'a rocket::State<rocket::tokio::sync::broadcast::Sender<Room>>,
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
    player_move: rocket::serde::json::Json<PlayerMove>,
    sessions: &rocket::State<ActiveMatchs>,
    queue: &rocket::State<rocket::tokio::sync::broadcast::Sender<Match>>,
) -> rocket::serde::json::Json<PlayerMoveResult> {
    let move_result: PlayerMoveResult = match sessions.make_move(&owner, player_move.into_inner()) {
        Some(v) => v,
        None => return rocket::serde::json::Json(PlayerMoveResult::Unknown),
    };
    if let PlayerMoveResult::Ok = move_result {
        let _ = queue.send(sessions.get_match(&owner).unwrap());
    }
    rocket::serde::json::Json(move_result)
}

#[get("/state/<owner>")]
fn get_state(owner: String, active_sessions: &rocket::State<ActiveMatchs>) -> rocket::serde::json::Json<Match> {
    let session_state = active_sessions.get_match(&owner);
    match session_state {
        None => rocket::serde::json::Json(Match::NotFound),
        Some(active_session) => rocket::serde::json::Json(active_session),
    }
}

#[get("/events/<owner>")]
async fn events<'a>(
    owner: String,
    queue: &'a rocket::State<rocket::tokio::sync::broadcast::Sender<Match>>,
    toket: auth::Token,
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

#[get("/session_chat/<owner>")]
async fn session_chat(
    owner: String,
    queue: &rocket::State<rocket::tokio::sync::broadcast::Sender<Messages>>,
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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                auth::unauthorized,
                auth::login,
                auth::register,
                post_message,
                make_move,
                session_chat,
                events,
                get_state,
                room_to_session,
                get_all_rooms,
                join_room,
                make_room,
                room_events,
                room_chat,
            ],
        )
        .mount("/", rocket::fs::FileServer::from(rocket::fs::relative!("static/build")))
        .register("/", catchers![auth::forbidden])
        .manage(rocket::tokio::sync::broadcast::channel::<Messages>(1024).0)
        .manage(rocket::tokio::sync::broadcast::channel::<Match>(1024).0)
        .manage(ActiveMatchs::new())
        .manage(rocket::tokio::sync::broadcast::channel::<Room>(1024).0)
        .manage(MatchRooms::new())
        .manage(models::DBLink::new("./db.sqlite3"))
        .manage(models::UserModel::new())
}
