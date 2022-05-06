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

function subscribe(uri: string) {
    var retryTime = 1;

    function connect(uri: string) {
        const events = new EventSource(uri);

        events.addEventListener("message", (ev) => {
            console.log("raw data", JSON.stringify(ev.data));
            console.log("decoded data", JSON.stringify(JSON.parse(ev.data)));
            const msg = JSON.parse(ev.data);
            if (!("message" in msg) || !("room" in msg) || !("username" in msg)) return;
            console.log(msg.msg);
        });

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


function injectEvents() {
    setButton("build_room", (_: any) => {
        fetch("/create_room", {
            method: "post",
            body: JSON.stringify({
                owner: STATE.player,
                game: "Quoridor"
            })
        }).then(response => response.json()).then((data) => { console.log(data) }).catch(console.error)
    })
    setButton(
        "make_player", (_: any) => {
            setPlayer("player")
            let head = document.getElementById("head")
            if (!head) return
            head.innerHTML = STATE.player
            fetch("")
        }
    )
}


window.addEventListener(
    "load", (_) => {
        injectEvents()
    }
)