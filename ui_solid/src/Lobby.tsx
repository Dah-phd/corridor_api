import { createSignal, For, onCleanup, Setter } from "solid-js"
import { finishTransition, startTransition } from "./Transition"
import { joinQuoriodrGame, getLobbies } from "./functions/lobbies"

const LOBBY_INTERVAL = "lobbyInterval"

function createLobbyInterval(lobbySetter: Setter<Array<string>>) {
    sessionStorage.setItem(
        LOBBY_INTERVAL,
        window.setInterval(() => { getLobbies(lobbySetter); }, 10000).toString()
    )
}

function killLobbyInterval() {
    const intervalID = sessionStorage.getItem(LOBBY_INTERVAL);
    window.clearInterval(Number(intervalID));
}

export function Lobbies() {
    const [activeLobbies, setLobbies] = createSignal<Array<string>>([]);
    createLobbyInterval(setLobbies);
    onCleanup(killLobbyInterval);
    getLobbies(setLobbies);
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