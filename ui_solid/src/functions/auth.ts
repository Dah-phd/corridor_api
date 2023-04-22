import { showMessage } from "../Message";
import { GUEST_URL, LOGIN_URL, LOGOUT, REGISTER_URL, USER_CONTEXT, getCookie, setCookie } from "./utils"
import { joinQuoriodrGame } from "./lobbies";
import { finishTransition, startTransition } from "../Transition";
import { setChatWS, setQuoridorSession, setQuoridorWS, userContextSetter } from "../App";

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
) {
    if (!getCookie()) { userContextSetter(null); return; }
    fetch(USER_CONTEXT)
        .then(data => handleAuthResponse(data))
        .catch(console.log)
}

export function login(
    email: string, password: string,
    after?: () => void
) {
    fetch(LOGIN_URL, {
        method: 'post',
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ email: email, password: password })
    })
        .then(data => handleAuthResponse(data))
        .catch(console.log)
        .finally(() => { if (after) after() })
}

export function logout() {
    fetch(LOGOUT).then(_ => { userContextSetter(null); setQuoridorWS(null); setQuoridorSession(null); setChatWS(null); }).catch(console.log);
}

export function registerUser(
    username: string, password: string, email: string,
    after?: () => void
) {
    if (password.length > 72) { showMessage("Password too long!"); return }
    fetch(REGISTER_URL, {
        method: 'post',
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ username: username, password: password, email: email })
    })
        .then(data => handleAuthResponse(data))
        .catch(console.log)
        .finally(() => { if (after) after() })
}

export function registerGuest(
    username: string,
    after?: () => void
) {
    fetch(GUEST_URL, {
        method: 'post',
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ username: username })
    })
        .then(data => handleAuthResponse(data))
        .catch(console.log)
        .finally(() => { if (after) after() })
}

function handleAuthResponse(response: Response) {
    if (!response.ok) {
        if (response.status >= 500) { showMessage("Server error please try agian in few minutes!"); return }
        if (response.status == 404) { showMessage("User not found! Check your login!"); return }
        if (response.status == 403) { showMessage("Incorrect credentials!"); return }
    }
    response.json().then(user => handleAuthResult(user))
}

function handleAuthResult(
    data: UserResult,
) {
    console.log(data);
    if (data === "AlreadyTaken") { showMessage("This username is already in use!"); return }
    if ("UnsupportedDataType" in data) { showMessage(data.UnsupportedDataType); return }
    userContextSetter(data);
    setCookie(data.authToken);
    if (data.activeMatch) {
        startTransition();
        setCookie(data.authToken);
        joinQuoriodrGame(data.activeMatch, finishTransition);
    }
}