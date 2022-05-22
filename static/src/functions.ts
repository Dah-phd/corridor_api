const LOGIN_URL = "/auth/login"
const REGISTER_URL = "/auth/register"
const GET_USER_NAME_URL = "/auth/"

const STATE = {
}

function get_user_name() { }

function login(username: string, password: string) {
    fetch(
        LOGIN_URL,
        { method: 'post', body: JSON.stringify({ User: [username, password] }) }
    )
}

function logout() { setCookie('gamertag', '', -1) }

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
            if ("Ok"! in json_data) { setToken(json_data["Some"]) } else { alert("Username already taken!") }
        })
        .catch(alert)
}

function registerGuest(username: string) {
    fetch(
        LOGIN_URL,
        { method: 'post', body: JSON.stringify({ Guest: username }) }
    )
        .then(resp => resp.json())
        .then((json_data) => {
            if (!json_data) { alert('To be fixed') } else { setToken(json_data) }
        })
        .catch(alert)
}

function setToken(token: string) { setCookie('gamertag', token) }

function getToken(): string { return getCookie('gamertag') }

function setCookie(name: string, value: string, days: number = 30) {
    let expires = "";
    if (days) {
        let date = new Date();
        date.setTime(date.getTime() + (days * 24 * 60 * 60 * 1000));
        expires = "; expires=" + date.toUTCString();
    }
    document.cookie = name + "=" + (value || "") + expires + "; path=/";
}

function getCookie(cookie_name: string): string {
    let re = new RegExp(`(?<=${cookie_name}=)[^;]*`)
    let cookie_result = document.cookie.match(re)
    if (!cookie_result) { return "this-cookie-doesn't-exist" }
    return cookie_result[0]
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


function setButton(button_id: string, callback: Function) {
    let btn = document.getElementById(button_id);
    if (!btn) return
    btn.addEventListener(
        "click", (e) => { callback(e) }
    )
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

window.addEventListener('load', () => {
    setButton('register_guest', (_: any) => {
        let guest_username = document.getElementById('guest') as HTMLInputElement | null
        console.log(guest_username)
        if (guest_username) {
            console.log(guest_username.value)
            registerGuest(guest_username.value)
        }
    })
})
