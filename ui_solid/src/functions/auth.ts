import { createSignal } from "solid-js";
import { showMessage } from "../Message";
import { LOGIN_URL, LOGOUT, REGISTER_URL, USER_CONTEXT, getCookie, setCookie } from "./utils"
import { joinQuoriodrGame } from "./lobbies";
import { finishTransition, startTransition } from "../Transition";

export interface UserContext {
    email: string,
    username: string,
    authToken: string,
    activeMatch: string | null,
}

type UserResult = UserContext
    | "AlreadyTaken"
    | { UnsupportedDataType: string };

export const [userContext, contextSetter] = createSignal<UserContext | null>();

export function getContext() {
    if (!getCookie()) { contextSetter(null); return; }
    fetch(USER_CONTEXT).then(handleAuthResponse).catch(console.log)
}

export function login(email: string, password: string, after?: () => void) {
    fetch(LOGIN_URL, { 
        method: 'post',
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ email: email, password: password }) 
    })
        .then(handleAuthResponse)
        .catch(console.log)
        .finally(() => { setTokenIfNotExists(); if (after) after() })
}

export function logout() {
    fetch(LOGOUT).then(_ => contextSetter(null)).catch(console.log);
}

export function registerUser(username: string, password: string, email: string, after?: () => void) {
    if (password.length > 72) { showMessage("Password too long!"); return }
    fetch(REGISTER_URL, {
        method: 'post',
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ username: username, password: password, email: email })
    })
        .then(handleAuthResponse)
        .catch(console.log)
        .finally(() => { setTokenIfNotExists(); if (after) after() })
}

export function registerGuest(username: string, after?: () => void) {
    fetch(LOGIN_URL, { 
        method: 'post', 
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ username: username }) 
    })
        .then(handleAuthResponse)
        .catch(console.log)
        .finally(() => { setTokenIfNotExists(); if (after) after() })
}

function handleAuthResponse(response: Response) {
    if (!response.ok) {
        if (response.status >= 500) { showMessage("Server error please try agian in few minutes!"); return }
        if (response.status == 404) { showMessage("User not found! Check your login!"); return }
        if (response.status == 403) { showMessage("Incorrect credentials!"); return }
    }
    response.json().then(handleAuthResult)
}

function handleAuthResult(data: UserResult) {
    console.log(data);
    if (data === "AlreadyTaken") { showMessage("This username is already in use!"); return }
    if ("UnsupportedDataType" in data) { showMessage(data.UnsupportedDataType); return }
    contextSetter(data);
    setCookie(data.authToken);
    if (data.activeMatch) {
        startTransition();
        setTokenIfNotExists(data.authToken);
        joinQuoriodrGame(data.activeMatch, finishTransition);
    }
}

function setTokenIfNotExists(passed_token?: string) {
    console.log(userContext());
    if (passed_token) {
        setCookie(passed_token);
        return;
    }
    if (getCookie()) return;
    const token = userContext()?.authToken;
    console.log(token);
    if (token) setCookie(token);
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