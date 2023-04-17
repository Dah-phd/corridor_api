import { QuoridorSession } from "./game_quoridor";
import { createSocket } from "./utils";
import { QUORIDOR_HOST, QUORIDOR_JOIN, QUORIDOR_SOLO, QUORIDOR_QUE, GAME_CHANNEL } from "./utils"
import { Accessor, Setter } from "solid-js";
import { UserContext } from "./auth";
import { showMessage } from "../Message";

function joinGame(context: UserContext, setWS: Setter<WebSocket>, setSession: Setter<QuoridorSession | null>) {
    if (context.activeMatch) {
        let socket = createSocket<string>(
            GAME_CHANNEL + context.activeMatch,
            (message) => {
                try {
                    const qSession = JSON.parse(message as string);
                    setSession(qSession);
                } catch (e) {
                    alert("Failed to read message from server! Try reloading the page.")
                }
            }
        );
        setWS(socket);
    } else {
        showMessage("Unable to create match, please try again later!")
    }
}

export function hostQuoriodrCPU(
    setWS: Setter<WebSocket>,
    setSession: Setter<QuoridorSession | null>,
    after?: () => void
) {
    fetch(QUORIDOR_SOLO)
        .then((status) => {
            if (!status.ok) {
                alert("Unable to create solo game!");
                return;
            }
            status.json().then(msg => joinGame(msg, setWS, setSession))
        })
        .catch((err) => { alert(err) })
        .finally(() => { if (after) after() })

}

export function joinQuoriodrGame(
    id: string,
    setWS: Setter<WebSocket>,
    setSession: Setter<QuoridorSession | null>,
    after?: () => void
) {
    fetch(QUORIDOR_JOIN + id)
        .then((status) => {
            if (!status.ok) {
                alert("Unable to join game!");
                return
            }
            status.json().then(msg => joinGame(msg, setWS, setSession))
        })
        .catch(alert)
        .finally(() => { if (after) after() })
}

export function hostQuoriodrGame(
    getWS: Accessor<WebSocket|null>,
    setWS: Setter<WebSocket|null>,
    setSession: Setter<QuoridorSession | null>,
    after?: () => void
) {
    if (getWS()?.OPEN) {showMessage("Already connected to game, refresh to reconnect!"); return};
    const builderSocket = createSocket<string>(
        QUORIDOR_HOST,
        (ev) => {
            console.log("event on builder", ev)
            builderSocket.close(); 
            let game_socket = createSocket(
                GAME_CHANNEL + ev,
                (message) => {
                    try {
                        const qSession = JSON.parse(message as string);
                        setSession(qSession);
                    } catch (e) {
                        alert("Failed to read message from server! Try reloading the page.")
                    }
                }

            );
            setWS(game_socket);
            if (after) after();
        },
        () => { if (after) after() }
    );
}

export function getLobbies(setter: Setter<Array<string>>) {
    fetch(QUORIDOR_QUE).then(
        resp => {
            console.log(resp)
            if (resp.ok) { resp.json().then(setter) }
            else showMessage("Unable to retrive QUE!")
        })
}