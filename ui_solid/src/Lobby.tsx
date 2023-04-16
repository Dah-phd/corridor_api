import { createSignal, For, onCleanup, Setter } from "solid-js"
import { finishTransition, startTransition } from "./Transition"
import { joinQuoriodrGame, getLobbies } from "./functions/lobbies"
import { QuoridorSession } from "./functions/game_quoridor"

const lobbyInterval = "lobbyInterval"

function createLobbyInterval(lobbySetter: Setter<Array<string>>) {
    sessionStorage.setItem(
        lobbyInterval,
        window.setInterval(() => { getLobbies(lobbySetter); }, 10000).toString()
    )
}

function killLobbyInterval() {
    const intervalID = sessionStorage.getItem(lobbyInterval);
    window.clearInterval(Number(intervalID));
}



export function Lobbies(props: {
    setWS: Setter<WebSocket | null>,
    setSession: Setter<QuoridorSession | null>
}) {
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
                            joinQuoriodrGame(host, props.setWS, props.setSession, finishTransition)
                        }}>
                            {host}
                        </div>
                    )
                }}
            </For>
        </div >
    )
}