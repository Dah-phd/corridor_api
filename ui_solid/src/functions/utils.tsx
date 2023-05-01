//AUTH paths
const AUTH = "/auth/"
export const LOGIN_URL = AUTH + "login";
export const GUEST_URL = AUTH + "guest_login"
export const LOGOUT = AUTH + "logout";
export const REGISTER_URL = AUTH + "register";
export const USER_CONTEXT = AUTH + "context/";
export const USER_STATS = AUTH + "stats";
// export const UPDATE_PASS = AUTH + "update_pass";
// export const UPDATE_EMAIL = AUTH + "update_email";

//CHAT paths
export const CHAT_CHANNEL = "/chat/"; //WS

//LEADERBOARD
export const LEADERBOARD = "/leaderboard";

// QUORIDOR paths
const QUORIDOR = "/quoridor/";
export const GAME_CHANNEL = QUORIDOR + "events/"; //WS
export const QUORIDOR_QUE = QUORIDOR + "que";
export const QUORIDOR_HOST = QUORIDOR_QUE + "/host"; //WS
export const QUORIDOR_JOIN = QUORIDOR_QUE + "/join/";
export const QUORIDOR_SOLO = QUORIDOR + "solo";

//TOKEN name
const TOKEN = 'auth_token'

export function createSocket<T>(uri_str: string, setter: (ev: T | null) => void, closeCallback?: (ev: any) => void): WebSocket {
    let url = new URL(uri_str, window.location.href);
    url.protocol = url.protocol.replace("http", "ws");
    let socket = new WebSocket(url);
    socket.onopen = () => console.log(`socket open on ${url}`);
    socket.onmessage = (ev: MessageEvent) => {
        setter(ev.data)
    };
    socket.onclose = (ev) => {
        if (closeCallback) { closeCallback(ev) }
    };
    return socket
}

export function setCookie(cookieValue: string, cookieName:string = TOKEN, daysAlive:number = 7) {
    const dt = new Date();
    dt.setTime(dt.getTime() + (daysAlive * 24 * 60 * 60 * 1000));
    document.cookie = cookieName + "=" + cookieValue + ";expires=" + dt.toUTCString() + ";path=/";
}

export function getCookie(cookieName:string = TOKEN):string | undefined {
    let name = cookieName + "=";
    let cookies = document.cookie.split(';');
    for (let i = 0; i < cookies.length; i++) {
        let cookie = cookies[i];
        while (cookie.charAt(0) == ' ') {
            cookie = cookie.substring(1);
        }
        if (cookie.indexOf(name) == 0) {
            return cookie.substring(name.length, cookie.length);
        }
    }
}

export type UserStats = {
    username: string,
    wins: number,
    loses: number,
}

export function LeaderBoardStat(props: { stats?: UserStats }) {
    const kdr = (w: number | undefined, l: number | undefined) => {
        if (w && l) return (w / (w + l));
        return 0
    }
    return (
        <>
            <p style="color:red;font-size:large;">{props.stats?.username} </p>
            <p>{"Wins: " + props.stats?.wins} | {"Loses: " + props.stats?.loses} | { "K/D: " + kdr(props.stats?.wins, props.stats?.loses)}</p>
            <hr />
        </>
    )
}