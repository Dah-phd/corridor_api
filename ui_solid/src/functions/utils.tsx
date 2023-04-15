//AUTH paths
export const LOGIN_URL = "/auth/login";
export const LOGOUT = "/auth/logout";
export const REGISTER_URL = "/auth/register";
export const USER_CONTEXT = "/auth/context/";
// export const UPDATE_PASS = "/auth/update_pass";
// export const UPDATE_EMAIL = "/auth/update_email";

//CHAT paths
export const CHAT_CHANNEL = "/chat/"; //WS

// QUORIDOR paths
export const QUORIDOR_QUE = "/quoridor/que"
export const QUORIDOR_SOLO = "/quoridor/solo"
export const QUORIDOR_HOST = QUORIDOR_QUE + "/host"; //WS
export const QUORIDOR_JOIN = QUORIDOR_QUE + "/join/";
export const GAME_CHANNEL = "/quoridor/events/"; //WS

//TOKEN name
const TOKEN = 'auth_token'

export function createSocket<T>(uri_str: string, setter: (ev: T | null) => void, closeCallback?: (ev: any) => void): WebSocket {
    let url = new URL(uri_str, window.location.href);
    url.protocol = url.protocol.replace("http", "ws");
    let socket = new WebSocket(url);
    socket.onopen = () => console.log(`socket open on ${url}`);
    socket.onmessage = (ev: MessageEvent) => {setter(ev.data)};
    socket.onclose = (ev) => {
        if (closeCallback) { closeCallback(ev) }
        setter(null)
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