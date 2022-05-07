let STATE = {
    connected: false,
    player: "guest"
}


interface QuoridorSession {
    id: number,
    up_player: string,
    down_player: string,
    game: {
        up_player: [number, number],
        down_player: [number, number],
        up_player_free_walls: number,
        down_player_free_walls: number,
        vertcal_walls: [number, number],    // (row, col)
        horizontal_walls: [number, number], // (row, col)
        winner: { Some: boolean } | string,
    },
    turn: number,
    current: string,
}

interface Session {
    ActiveQuoridor?: QuoridorSession
}

function setPlayer(id: string) {
    let name: any = document.getElementById(id)
    if (!name) { return }
    STATE.player = name.value
    name.value = ""
}

function subscribe(uri: string, callback: Function) {
    let retryTime = 1;

    function connect(uri: string) {
        const events = new EventSource(uri);

        events.addEventListener("message", (ev) => { callback(ev) });

        events.addEventListener("open", () => {
            setConnectedStatus(true);
            console.log(`connected to event stream at ${uri}`);
            retryTime = 1;
        });

        events.addEventListener("error", () => {
            setConnectedStatus(false);
            events.close();

            let timeout = retryTime;
            retryTime = Math.min(64, retryTime * 2);
            console.log(`connection lost. attempting to reconnect in ${timeout}s`);
            setTimeout(() => connect(uri), (() => timeout * 1000)());
        });
    }

    connect(uri);
}

// Set the connection status: `true` for connected, `false` for disconnected.
function setConnectedStatus(status: boolean) {
    STATE.connected = status;
}


// subscribe('/events/')


function setButton(button_id: string, callback: Function) {
    let btn = document.getElementById(button_id);
    if (!btn) return
    btn.addEventListener(
        "click", (e) => { callback(e) }
    )
}

function paintGame(obj: string) {
    console.log(obj)
}

function hasGameStarted(obj: MessageEvent, room_sub: Subscribtion) {
    let json = JSON.parse(obj.data)
    console.log(json)
    let id = json['game_id']
    if (id == null) return
    room_sub.kill()
    fetch("/state/" + id, { method: "get" })
        .then(response => response.json())
        .then((data) => paintGame(data))
    new Subscribtion("/events/" + id, paintGame)
}

class Subscribtion {
    status: boolean
    private uri: string
    private event_src: EventSource
    private callback: Function
    private retry: number

    constructor(uri: string, callback: Function, retry: number = 1) {
        this.uri = uri
        this.event_src = new EventSource(uri)
        this.callback = callback
        this.retry = retry
        this.status = false
        this.connect()
    }

    kill() {
        this.event_src.close()
    }

    connect() {
        this.event_src.addEventListener("message", (ev) => { this.callback(ev, this) });

        this.event_src.addEventListener("open", () => {
            this.status = true
            console.log("connected to event stream at " + this.uri);
        });

        this.event_src.addEventListener("error", () => {
            this.status = false;
            this.event_src.close();
            let timeout = this.retry;
            this.retry = Math.min(64, this.retry * 2);
            console.log(`connection lost. attempting to reconnect in ${timeout}s`);
            setTimeout(() => { this.event_src = new EventSource(this.uri); this.connect() }, (() => timeout * 1000)());
        });
    }

}

function injectEvents() {
    setButton("build_room", (_: any) => {
        fetch("/create_room", {
            method: "post",
            body: JSON.stringify({
                owner: STATE.player,
                game: "Quoridor"
            })
        })
            .then(response => response.json())
            .then((data) => {
                console.log(data); if (data === true) new Subscribtion("/room_events/" + STATE.player, hasGameStarted)
            })
            .catch(console.error)
    })
    setButton(
        "make_player", (_: any) => {
            setPlayer("player")
            let head = document.getElementById("head")
            if (!head) return
            head.innerHTML = STATE.player
            fetch("/opened_rooms", { method: "get" }).then(response => response.json()).then((data) => console.log(data)).catch(console.error)
        }
    )
    setButton(
        "join_room", (_: any) => {
            let room_owner: any = document.getElementById('room')
            if (!room_owner) return
            new Subscribtion("/room_events/" + room_owner.value, hasGameStarted)
            fetch("/join/" + room_owner.value + "/" + STATE.player, { method: "get" }).catch(console.error)
        }
    )
    setButton(
        "start_game", (_: any) => {
            fetch('/start_game/' + STATE.player, { method: "get" })
        }
    )
    setButton(
        "token", (_: any) => {
            fetch('/login', {
                method: "post", body: JSON.stringify({
                    Guest: "dah"
                })
            })
                .then(response => response.json())
                .then((data) => { console.log(data) })
                .catch(console.error)
        }
    )
}


window.addEventListener(
    "load", (_) => {
        injectEvents()
    }
)