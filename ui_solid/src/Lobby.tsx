import { For, onCleanup, createSignal, Setter } from "solid-js"
import { finishTransition, startTransition } from "./Transition"
import { joinQuoriodrGame, getLobbies } from "./functions/lobbies"
import { activeLobbies } from "./App"
import { LEADERBOARD, LeaderBoardStat, UserStats } from "./functions/utils"
import { message } from "./Message"

const LOBBY_INTERVAL = "lobbyInterval"

function createLobbyInterval() {
    sessionStorage.setItem(
        LOBBY_INTERVAL,
        window.setInterval(() => { getLobbies(); }, 5000).toString()
    )
}

function killLobbyInterval() {
    const intervalID = sessionStorage.getItem(LOBBY_INTERVAL);
    window.clearInterval(Number(intervalID));
}

function LeaderBoard() {
    const [leaderBoard, setLeaderBoard] = createSignal<Array<UserStats>>([]);
    fetch(LEADERBOARD)
        .then(res => { if (res.ok) res.json().then(json => { setLeaderBoard(json); console.log(json) }) })
        .catch(e => { console.log(e); })

    return (
        <div>
            <h3>LeaderBoard</h3>
            <hr />
            <For each={leaderBoard()}>{(playerStat) => <LeaderBoardStat stats={playerStat} />}</For>
        </div>
    )
}

export function Lobbies() {
    createLobbyInterval();
    onCleanup(killLobbyInterval);
    getLobbies();
    return (
        <>
            <LeaderBoard />
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
        </>
    )
}