import { createSocket } from "./utils";
import { QUORIDOR_HOST, QUORIDOR_JOIN, QUORIDOR_SOLO, QUORIDOR_QUE, GAME_CHANNEL } from "./utils"
import { UserContext } from "./auth";
import { showMessage } from "../Message";
import { getQuoridorWS, setQuoridorSession, setQuoridorWS } from "../App";
import { createChat } from "./chat";
import { setLobbies } from "../App";
import { finishTransition, startTransition } from "../Transition";

function joinGame(context: UserContext) {
    if (context.activeMatch) {
        let socket = createSocket<string>(
            GAME_CHANNEL + context.activeMatch,
            (message) => {
                try {
                    const qSession = JSON.parse(message as string);
                    setQuoridorSession(qSession);
                } catch (e) {
                    showMessage("Failed to read message from server! Try reloading the page.");
                    console.log(e);
                }
            },
        );
        setQuoridorWS(socket);
        createChat(context.activeMatch);
    } else {
        showMessage("Unable to create match, please try again later!")
    }
}

export function hostQuoriodrCPU() {
    fetch(QUORIDOR_SOLO)
        .then((response) => {
            if (!response.ok) {
                showMessage("Unable to create solo game!");
                return;
            }
            response.json().then(joinGame)
        })
        .catch((err) => { alert(err) })
        .finally(finishTransition)
}

export function joinQuoriodrGame(id: string, after?: () => void) {
    fetch(QUORIDOR_JOIN + id)
        .then((response) => {
            if (!response.ok) {
                showMessage("Unable to join game!");
                return
            }
            response.json().then(joinGame)
        })
        .catch(alert)
        .finally(() => { if (after) after() })
}

export function hostQuoriodrGame() {
    if (getQuoridorWS()?.OPEN) { showMessage("Already connected to game, refresh to reconnect!"); return };
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
                        setQuoridorSession(qSession);
                    } catch (e) {
                        showMessage("Failed to read message from server! Try reloading the page.")
                    }
                }

            );
            setQuoridorWS(game_socket);
            createChat(ev as string);
        },
        (_) => { finishTransition() }
    );
    console.log("setting cancel!!");
    startTransition("Looking for game ...", { cb: () =>  builderSocket.close() });
}

export function getLobbies() {
    fetch(QUORIDOR_QUE).then(
        (resp) => {
            if (resp.ok) { resp.json().then(setLobbies) }
            else showMessage("Unable to retrive QUE!")
        })
}