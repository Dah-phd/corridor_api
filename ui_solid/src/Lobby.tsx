import { For, onCleanup } from "solid-js"
import { finishTransition, startTransition } from "./Transition"
import { joinQuoriodrGame, getLobbies } from "./functions/lobbies"
import { activeLobbies } from "./App"

const LOBBY_INTERVAL = "lobbyInterval"

function createLobbyInterval() {
    sessionStorage.setItem(
        LOBBY_INTERVAL,
        window.setInterval(() => { getLobbies(); }, 10000).toString()
    )
}

function killLobbyInterval() {
    const intervalID = sessionStorage.getItem(LOBBY_INTERVAL);
    window.clearInterval(Number(intervalID));
}

export function Lobbies() {
    createLobbyInterval();
    onCleanup(killLobbyInterval);
    getLobbies();
    return (
        <div class="lobbies">
            <For each={activeLobbies()}>
                {(host) => {
                    return (
                        <div class="lobby_struct" onClick={() => {
                            startTransition();
                            joinQuoriodrGame(host, finishTransition)
                        }}>
                            {host}
                        </div>
                    )
                }}
            </For>
        </div >
    )
}