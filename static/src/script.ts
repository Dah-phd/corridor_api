const LOGIN_URL = "/auth/login"
const REGISTER_URL = "/auth/register"

const STATE = {
    connected: false,
}

function login(username: string, password: string) {
    fetch(
        LOGIN_URL,
        { method: 'post', body: JSON.stringify({ User: [username, password] }) }
    )
}

function registerUser(username: string, password: string, email: string) {
    if (password.length > 72) return "password too long"
    fetch(
        REGISTER_URL,
        {
            method: 'post', body: JSON.stringify({
                User: {
                    user: username,
                    password: password,
                    email: email,
                }
            })
        }
    )
        .then(resp => resp.json())
        .then((json_data) => {
            if ("None" in json_data) { alert("Username already taken") } else { setToken(json_data["Some"]) }
        })
        .catch(alert)
}

function registerGues(username: string) {
    fetch(
        LOGIN_URL,
        { method: 'post', body: JSON.stringify({ Guest: username }) }
    )
        .then(resp => resp.json())
        .then((json_data) => {
            if ("None" in json_data) { "Guest username already taken" } else { setToken(json_data["Some"]) }
        })
        .catch(alert)
}

function setToken(token: string) { setCookie('token', token) }

function setCookie(name: string, value: string, days: number = 30) {
    let expires = "";
    if (days) {
        let date = new Date();
        date.setTime(date.getTime() + (days * 24 * 60 * 60 * 1000));
        expires = "; expires=" + date.toUTCString();
    }
    document.cookie = name + "=" + (value || "") + expires + "; path=/";
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
