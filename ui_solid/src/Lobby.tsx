import { createSignal, For, onCleanup } from "solid-js"
import { finishTransition, startTransition } from "./Transition"
import { joinQuoriodrGame, getLobbies } from "./functions/lobbies"

const lobbyInterval = "lobbyInterval"

function createLobbyInterval(lobbySetter: (lobbies: Array<string>) => void) {
    sessionStorage.setItem(
        lobbyInterval,
        window.setInterval(() => { getLobbies(lobbySetter); }, 10000).toString()
    )
}

function killLobbyInterval() {
    const intervalID = sessionStorage.getItem(lobbyInterval);
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
                        <div class="lobby_struct" onClick={() => { startTransition(); joinQuoriodrGame(host, finishTransition) }}>
                            {host}
                        </div>
                    )
                }}
            </For>
        </div >
    )
}