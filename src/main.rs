#[macro_use]
extern crate rocket;
use rocket::serde::{Deserialize, Serialize};
mod abstarctions;
use abstarctions::{ActiveSessions, ChatID, Messages, PlayerMove, Room, Session, SessionRooms, SessionType};
mod quoridor;

#[post("/move/<session>", data = "<player_move>")]
fn make_move(
    session: i32,
    player_move: rocket::serde::json::Json<PlayerMove>,
    sessions: &rocket::State<ActiveSessions>,
    queue: &rocket::State<rocket::tokio::sync::broadcast::Sender<Session>>,
) {
    sessions.make_move(session, player_move.into_inner());
}

#[get("/state/<session>")]
fn get_state(session: i32, active_sessions: &rocket::State<ActiveSessions>) -> rocket::serde::json::Json<Session> {
    let session_state = active_sessions.get_session(session);
    match session_state {
        None => rocket::serde::json::Json(Session::NotFound),
        Some(active_session) => rocket::serde::json::Json(active_session),
    }
}

// init room

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct RoomBase {
    owner: String,
    game: SessionType,
}

#[post("/create_room", data = "<room>")]
fn make_room(room: rocket::serde::json::Json<RoomBase>, rooms: &rocket::State<SessionRooms>) -> rocket::serde::json::Json<bool> {
    rocket::serde::json::Json(rooms.new_room(&room.owner, room.game))
}

#[get("/join/<owner>/<player>")]
fn join_room(
    owner: String,
    player: String,
    rooms: &rocket::State<SessionRooms>,
    room_queue: &rocket::State<rocket::tokio::sync::broadcast::Sender<Room>>,
) {
    if rooms.add_player(owner.to_owned(), player) {
        match rooms.get_by_owner(&owner) {
            Some(room) => {
                let _res = room_queue.send(room);
            }
            _ => (),
        };
    }
}

// init new game from room

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct RoomToSession(Option<i32>);

#[get("/start_game/<owner>")]
fn room_to_session(
    owner: String,
    rooms: &rocket::State<SessionRooms>,
    room_queue: &rocket::State<rocket::tokio::sync::broadcast::Sender<Room>>,
    sessions: &rocket::State<ActiveSessions>,
) {
    let maybe_room = rooms.get_by_owner(&owner);
    match maybe_room {
        Some(mut room) => {
            let new_session = sessions.append(&room.player_list, room.session_type);
            match new_session {
                Ok(id) => {
                    room.game_id = Some(id);
                    let _res = room_queue.send(room);
                }
                Err(_) => (),
            }
        }
        None => (),
    }
}

#[get("/opened_rooms")]
fn get_all_rooms(rooms: &rocket::State<SessionRooms>) -> rocket::serde::json::Json<Vec<Room>> {
    rocket::serde::json::Json(rooms.get_all())
}

#[post("/chat/sender", data = "<msg>")]
fn post_message(
    msg: rocket::serde::json::Json<Messages>,
    queue: &rocket::State<rocket::tokio::sync::broadcast::Sender<Messages>>,
) {
    queue.send(msg.into_inner());
}

#[get("/chat/<room_owner>")]
async fn room_chat(
    room_owner: String,
    queue: &rocket::State<rocket::tokio::sync::broadcast::Sender<Messages>>,
    mut end: rocket::Shutdown,
) -> rocket::response::stream::EventStream![] {
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
async fn room_events(
    owner: String,
    queue: &rocket::State<rocket::tokio::sync::broadcast::Sender<Room>>,
    active_rooms: &rocket::State<SessionRooms>,
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
        // active_rooms.drop(&owner);
    }
}

#[get("/events/<session>")]
async fn events(
    session: i32,
    queue: &rocket::State<rocket::tokio::sync::broadcast::Sender<Session>>,
    active_sessions: &rocket::State<ActiveSessions>,
    mut end: rocket::Shutdown,
) -> rocket::response::stream::EventStream![] {
    let mut rx = queue.subscribe();
    rocket::response::stream::EventStream! {
        loop {
            let msg = rocket::tokio::select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => if msg.expose().id == session {msg} else {continue},
                    Err(rocket::tokio::sync::broadcast::error::RecvError::Closed) => break,
                    Err(rocket::tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield rocket::response::stream::Event::json(&msg);
        }
        // active_sessions.drop(session);
    }
}

#[get("/chat/<session>")]
async fn session_chat(
    session: i32,
    queue: &rocket::State<rocket::tokio::sync::broadcast::Sender<Messages>>,
    mut end: rocket::Shutdown,
) -> rocket::response::stream::EventStream![] {
    let mut rx = queue.subscribe();
    rocket::response::stream::EventStream! {
        loop {
            let msg = rocket::tokio::select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => if msg.id == ChatID::SessionID(session) {msg} else {continue},
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
        .mount("/", routes![events, make_move])
        .mount("/", rocket::fs::FileServer::from(rocket::fs::relative!("static/build")))
        .manage(rocket::tokio::sync::broadcast::channel::<Messages>(1024).0)
        .manage(rocket::tokio::sync::broadcast::channel::<Session>(1024).0)
        .manage(ActiveSessions::new())
        .manage(rocket::tokio::sync::broadcast::channel::<Room>(1024).0)
        .manage(SessionRooms::new())
}
