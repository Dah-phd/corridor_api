import { QuoridorSession } from "./game_quoridor";
import { createSocket } from "./utils";
import { QUORIDOR_HOST, QUORIDOR_JOIN, QUORIDOR_SOLO, QUORIDOR_QUE, GAME_CHANNEL } from "./utils"
import { Setter, createSignal } from "solid-js";
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
    context: UserContext,
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
            status.json().then(msg => joinGame(context, setWS, setSession))
        })
        .catch((err) => { alert(err) })
        .finally(() => { if (after) after() })

}

export function joinQuoriodrGame(
    id: string,
    context: UserContext,
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
            status.json().then(data => joinGame(context, setWS, setSession))
        })
        .catch(alert)
        .finally(() => { if (after) after() })
}

export function hostQuoriodrGame(
    setWS: Setter<WebSocket>,
    setSession: Setter<QuoridorSession | null>,
    after?: () => void
) {
    createSocket<string>(
        QUORIDOR_HOST,
        (ev) => {
            let game_socket = createSocket<QuoridorSession>(GAME_CHANNEL + ev, setSession);
            setWS(game_socket);
            if (after) after();
        },
        () => { if (after) after() }
    );
}

export function getLobbies(setter: (lobbies: Array<string>) => void) {

}