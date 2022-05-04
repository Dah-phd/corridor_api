"use strict";
let STATE = {
    connected: false
};
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
window.addEventListener("load", (_) => {
    let btn = document.getElementById('sub');
    console.log(btn);
    if (!btn)
        return;
    btn.addEventListener("click", (_) => {
        let room = document.getElementById('room');
        subscribe("events/" + room.value);
    });
});
