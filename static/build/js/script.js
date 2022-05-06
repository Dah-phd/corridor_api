"use strict";
let STATE = {
    connected: false,
    player: "guest"
};
function setPlayer(id) {
    let name = document.getElementById(id);
    if (!name) {
        return;
    }
    STATE.player = name.value;
    name.value = "";
}
function subscribe(uri) {
    var retryTime = 1;
    function connect(uri) {
        const events = new EventSource(uri);
        events.addEventListener("message", (ev) => {
            console.log("raw data", JSON.stringify(ev.data));
            console.log("decoded data", JSON.stringify(JSON.parse(ev.data)));
            const msg = JSON.parse(ev.data);
            if (!("message" in msg) || !("room" in msg) || !("username" in msg))
                return;
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
function setConnectedStatus(status) {
    STATE.connected = status;
}
// subscribe('/events/')
function setButton(button_id, callback) {
    let btn = document.getElementById(button_id);
    if (!btn)
        return;
    btn.addEventListener("click", (e) => { callback(e); });
}
function injectEvents() {
    setButton("build_room", (_) => {
        fetch("/create_room", {
            method: "post",
            body: JSON.stringify({
                owner: STATE.player,
                game: "Quoridor"
            })
        }).then(response => response.json()).then((data) => { console.log(data); }).catch(console.error);
    });
    setButton("make_player", (_) => {
        setPlayer("player");
        let head = document.getElementById("head");
        if (!head)
            return;
        head.innerHTML = STATE.player;
        fetch("");
    });
}
window.addEventListener("load", (_) => {
    injectEvents();
});
