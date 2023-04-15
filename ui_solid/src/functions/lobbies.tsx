import { QuoridorSession } from "./game_quoridor";
import { createSocket } from "./utils";
import { QUORIDOR_HOST, QUORIDOR_JOIN, QUORIDOR_SOLO, QUORIDOR_QUE, GAME_CHANNEL } from "./utils"
import { setQuoridorWS } from "../AppViews";
import { createSignal } from "solid-js";
import { UserContext } from "./auth";
import { showMessage } from "../Message";

function joinGame(context: UserContext) {
    if (context.activeMatch) {
        let [getter, setter] = createSignal<QuoridorSession | null>(null);
        let socket = createSocket<string>(
            GAME_CHANNEL + context.activeMatch,
            (message) => {
                try {
                    setter(JSON.parse(message as string))
                } catch (e) {
                    console.log(e);
                    alert("Failed to read message from server! Try reloading the page.")
                }
            }
        );
        setQuoridorWS([socket, getter]);
    } else {
        showMessage("Unable to create match, please try again later!")
    }
}

export function hostQuoriodrCPU(after?: () => void) {
    fetch(QUORIDOR_SOLO)
        .then((status) => {
            if (!status.ok) {
                alert("Unable to create solo game!");
                return;
            }
            status.json().then(joinGame)
        })
        .catch(alert)
        .finally(() => { if (after) after() })

}

export function joinQuoriodrGame(id: string, after?: () => void) {
    fetch(QUORIDOR_JOIN + id)
        .then((status) => {
            if (!status.ok) {
                alert("Unable to join game!");
                return
            }
            status.json().then(joinGame)
        })
        .catch(alert)
        .finally(() => { if (after) after() })
}

export function hostQuoriodrGame(after?: () => void) {
    createSocket<string>(
        QUORIDOR_HOST,
        (ev) => {
            let [getter, setter] = createSignal<QuoridorSession | null>(null);
            let game_socket = createSocket<QuoridorSession>(GAME_CHANNEL + ev, setter);
            setQuoridorWS([game_socket, getter]);
            if (after) after();
        },
        () => { if (after) after() }
    );
}

export function getLobbies(setter: (lobbies: Array<string>) => void) {

}