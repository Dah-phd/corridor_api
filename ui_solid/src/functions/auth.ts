import { Setter, createSignal } from "solid-js";
import { showMessage } from "../Message";
import { LOGIN_URL, LOGOUT, REGISTER_URL, USER_CONTEXT, getCookie, setCookie } from "./utils"
import { joinQuoriodrGame } from "./lobbies";
import { finishTransition, startTransition } from "../Transition";
import { QuoridorSession } from "./game_quoridor";

export interface UserContext {
    email: string,
    username: string,
    authToken: string,
    activeMatch: string | null,
}

type UserResult = UserContext
    | "AlreadyTaken"
    | { UnsupportedDataType: string };

export function getContext(
    contextSetter: Setter<UserContext | null>,
    setWS: Setter<WebSocket | null>,
    setSession: Setter<QuoridorSession | null>,
) {
    if (!getCookie()) { contextSetter(null); return; }
    fetch(USER_CONTEXT)
    .then(data => handleAuthResponse(data, contextSetter, setWS, setSession))
    .catch(console.log)
}

export function login(
    email: string, password: string, 
    contextSetter: Setter<UserContext | null>,
    setWS: Setter<WebSocket | null>,
    setSession: Setter<QuoridorSession | null>,
    after?: () => void
) {
    fetch(LOGIN_URL, {
        method: 'post',
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ email: email, password: password })
    })
        .then(data => handleAuthResponse(data, contextSetter, setWS, setSession))
        .catch(console.log)
        .finally(() => { if (after) after() })
}

export function logout(contextSetter: Setter<UserContext | null>) {
    fetch(LOGOUT).then(_ => contextSetter(null)).catch(console.log);
}

export function registerUser(
    username: string, password: string, email: string, 
    contextSetter: Setter<UserContext | null>,
    setWS: Setter<WebSocket | null>,
    setSession: Setter<QuoridorSession | null>,
    after?: () => void
) {
    if (password.length > 72) { showMessage("Password too long!"); return }
    fetch(REGISTER_URL, {
        method: 'post',
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ username: username, password: password, email: email })
    })
        .then(data => handleAuthResponse(data, contextSetter, setWS, setSession))
        .catch(console.log)
        .finally(() => { if (after) after() })
}

export function registerGuest(
    username: string,
    contextSetter: Setter<UserContext | null>,
    setWS: Setter<WebSocket | null>,
    setSession: Setter<QuoridorSession | null>,
    after?: () => void
) {
    fetch(LOGIN_URL, {
        method: 'post',
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ username: username })
    })
        .then(data => handleAuthResponse(data, contextSetter, setWS, setSession))
        .catch(console.log)
        .finally(() => { if (after) after() })
}

function handleAuthResponse(
    response: Response,
    contextSetter: Setter<UserContext | null>,
    setWS: Setter<WebSocket | null>,
    setSession: Setter<QuoridorSession | null>
) {
    if (!response.ok) {
        if (response.status >= 500) { showMessage("Server error please try agian in few minutes!"); return }
        if (response.status == 404) { showMessage("User not found! Check your login!"); return }
        if (response.status == 403) { showMessage("Incorrect credentials!"); return }
    }
    response.json().then(user => handleAuthResult(user, contextSetter, setWS, setSession))
}

function handleAuthResult(
    data: UserResult,
    contextSetter: Setter<UserContext | null>,
    setWS: Setter<WebSocket | null>,
    setSession: Setter<QuoridorSession | null>
) {
    console.log(data);
    if (data === "AlreadyTaken") { showMessage("This username is already in use!"); return }
    if ("UnsupportedDataType" in data) { showMessage(data.UnsupportedDataType); return }
    contextSetter(data);
    setCookie(data.authToken);
    if (data.activeMatch) {
        startTransition();
        setCookie(data.authToken);
        joinQuoriodrGame(data.activeMatch, setWS, setSession, finishTransition);
    }
}

// export function updatePassword(password: string, after?: () => void) {
//     if (password.length > 72) { showMessage("Password too long!"); return }
//     fetch(UPDATE_PASS, {
//         method: 'put',
//         body: JSON.stringify({ user: getUser(), password: password, email: "" })
//     })
//         .then(response => response.json())
//         .then((data: UserResult) => handleAuthResult(data, (_) => showMessage("Password is changed!")))
//         .catch(console.log)
//         .finally(() => { if (after) after() })
// }