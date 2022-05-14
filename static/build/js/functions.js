"use strict";
const LOGIN_URL = "/auth/login";
const REGISTER_URL = "/auth/register";
const STATE = {};
function login(username, password) {
    fetch(LOGIN_URL, { method: 'post', body: JSON.stringify({ User: [username, password] }) });
}
function registerUser(username, password, email) {
    if (password.length > 72)
        return "password too long";
    fetch(REGISTER_URL, {
        method: 'post', body: JSON.stringify({
            User: {
                user: username,
                password: password,
                email: email,
            }
        })
    })
        .then(resp => resp.json())
        .then((json_data) => {
        if ("Ok" in json_data) {
            setToken(json_data["Some"]);
        }
        else {
            alert("Username already taken!");
        }
    })
        .catch(alert);
}
function registerGuest(username) {
    fetch(LOGIN_URL, { method: 'post', body: JSON.stringify({ Guest: username }) })
        .then(resp => resp.json())
        .then((json_data) => {
        if ("None" in json_data) {
            "Guest username already taken";
        }
        else {
            setToken(json_data["Some"]);
        }
    })
        .catch(alert);
}
function setToken(token) { setCookie('gamertag', token); }
function getToken() { return getCookie('gamertag'); }
function setCookie(name, value, days = 30) {
    let expires = "";
    if (days) {
        let date = new Date();
        date.setTime(date.getTime() + (days * 24 * 60 * 60 * 1000));
        expires = "; expires=" + date.toUTCString();
    }
    document.cookie = name + "=" + (value || "") + expires + "; path=/";
}
function getCookie(cookie_name) {
    let re = new RegExp(`(?<=${cookie_name}=)[^;]*`);
    let cookie_result = document.cookie.match(re);
    if (!cookie_result) {
        return "this-cookie-doesn't-exist";
    }
    return cookie_result[0];
}
function setPlayer(id) {
    let name = document.getElementById(id);
    if (!name) {
        return;
    }
    name.value = "";
}
function setButton(button_id, callback) {
    let btn = document.getElementById(button_id);
    if (!btn)
        return;
    btn.addEventListener("click", (e) => { callback(e); });
}
class Subscribtion {
    constructor(uri, callback, retry = 1) {
        this.uri = uri;
        this.event_src = new EventSource(uri);
        this.callback = callback;
        this.retry = retry;
        this.status = false;
        this.connect();
    }
    kill() {
        this.event_src.close();
    }
    connect() {
        this.event_src.addEventListener("message", (ev) => { this.callback(ev, this); });
        this.event_src.addEventListener("open", () => {
            this.status = true;
            console.log("connected to event stream at " + this.uri);
        });
        this.event_src.addEventListener("error", () => {
            this.status = false;
            this.event_src.close();
            let timeout = this.retry;
            this.retry = Math.min(64, this.retry * 2);
            console.log(`connection lost. attempting to reconnect in ${timeout}s`);
            setTimeout(() => { this.event_src = new EventSource(this.uri); this.connect(); }, (() => timeout * 1000)());
        });
    }
}
window.addEventListener('load', () => {
    setToken("secret");
    console.log(getToken());
});
