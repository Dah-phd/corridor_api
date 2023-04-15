
import { QuoridorSession, getBorderClass, setPlayer } from "./functions/game_quoridor";
import { For, createSignal, onMount, onCleanup } from "solid-js";
const ARRAY_OF_IDS = [0, 1, 2, 3, 4, 5, 6, 7, 8];

function setTileClass(row: number, column: number, session: QuoridorSession): string {
    const cls = [getBorderClass(row, column, session.game)];
    return "tile" + cls.join(' ') + setPlayer(row, column, session.game)
}

function Tile(props: { row: number, column: number, session: QuoridorSession }) {
    return (
        <div
            class={setTileClass(props.row, props.column, props.session)}
            id={`${props.row}${props.column}`}
        >
        </div>
    )
}

function Row(props: { row: number, game: QuoridorSession }) {
    return (
        <div class="row">
            <For each={ARRAY_OF_IDS}>
                {(i) => { return <Tile column={i} row={props.row} session={props.game} /> }}
            </For>
        </div>
    )
}

export function FinishedQuoridor(props: { session: QuoridorSession }) {
    return (
        <div class="full_screen_centered">
            <div class="quoridor">
                <h1>{props.session.up_player} ({props.session.game.up_player_free_walls})</h1>
                <div>
                    <For each={ARRAY_OF_IDS}>
                        {(i) => { return <Row row={i} game={props.session} /> }}
                    </For>
                </div>
                <h1>{props.session.down_player} ({props.session.game.down_player_free_walls})</h1>
            </div>
        </div>
    )
}