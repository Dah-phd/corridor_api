import { Accessor, For, createSignal } from "solid-js"
import {
    QuoridorSession,
    makeQuoridorMove,
    quoridorMove,
    quoridorHWall,
    quoridorVWall,
    getQuoridorHoverPlayerMove,
    getBorderClass,
    shouldHoverHWall,
    shouldHoverVWall,
    setPlayer
} from "./functions/game_quoridor"

import { showMessages } from "./Chat";
import { UserContext } from "./functions/auth";

const ARRAY_OF_IDS = [0, 1, 2, 3, 4, 5, 6, 7, 8];

const moveDispatch: any = {
    move: quoridorMove,
    hW: quoridorHWall,
    vW: quoridorVWall
}

const [moveCallbackName, setMoveCallbackName] = createSignal('move');

const HOVER_WALLS_CLASSES = new RegExp('wall_bot|wall_top|wall_left|wall_right');

function setTileClass(row: number, column: number, session: QuoridorSession): string {
    const cls = [getBorderClass(row, column, session.game)];
    if (moveCallbackName() == 'move') { cls.push(getQuoridorHoverPlayerMove(row, column, session)) };
    return "tile" + cls.join(' ') + setPlayer(row, column, session.game)
}

function setClassToWall(row: number, col: number, cls: string) {
    const element = document.getElementById(`${row}${col}`);
    if (element) { element.className = `${element.className} ${cls}` };
}

function dispatchHoverWallBorders(row: number, col: number, session: QuoridorSession) {
    const moveType = moveCallbackName();
    if (moveType === 'hW' && shouldHoverHWall(row, col, session.game)) {
        setClassToWall(row, col, 'wall_bot');
        setClassToWall(row, col + 1, 'wall_bot');
        setClassToWall(row + 1, col, 'wall_top');
        setClassToWall(row + 1, col + 1, 'wall_top');
    }
    if (moveType === 'vW' && shouldHoverVWall(row, col, session.game)) {
        setClassToWall(row, col, 'wall_right');
        setClassToWall(row + 1, col, 'wall_right');
        setClassToWall(row, col + 1, 'wall_left');
        setClassToWall(row + 1, col + 1, 'wall_left');
    }

}

function removeHoverWallBorders(row: number, col: number) {
    if (moveCallbackName() === 'move') return;
    const positions = [[row, col], [row, col + 1], [row + 1, col], [row + 1, col + 1]];
    for (const pos in positions) {
        const elemet = document.getElementById(`${positions[pos][0]}${positions[pos][1]}`);
        if (elemet) elemet.className = elemet.className.replace(HOVER_WALLS_CLASSES, '');
    }
}

function Tile(props: { row: number, column: number, session: QuoridorSession, ws: WebSocket, user: UserContext }) {
    return (
        <div
            class={setTileClass(props.row, props.column, props.session)}
            id={`${props.row}${props.column}`}
            onClick={() => makeQuoridorMove(props.row, props.column, props.session, props.ws, props.user, moveDispatch[moveCallbackName()])}
            onMouseEnter={() => {
                if (moveCallbackName() === "move") return;
                dispatchHoverWallBorders(props.row, props.column, props.session);
            }}
            onMouseLeave={() => {
                if (moveCallbackName() === "move") return;
                removeHoverWallBorders(props.row, props.column);
            }}
        >
        </div>
    )
}

function Row(props: { row: number, session: QuoridorSession, user:UserContext, ws: WebSocket }) {
    return (
        <div class="row">
            <For each={ARRAY_OF_IDS}>
                {(i) => { return <Tile column={i} {...props} /> }}
            </For>
        </div>
    )
}

export function QuoridorBoard(props: {ws:WebSocket, session: QuoridorSession | null, user:UserContext}) {
    if (!props.session) return;

    function currentRed(player: string) {
        return props.session?.current == player ? "color:red" : ""
    }

    return (
        <div class="full_screen_centered" style={showMessages() ? "display:none;" : ""}>
            <div class="quoridor">
                <h1 style={currentRed(props.session.up_player)}>{props.session.up_player} ({props.session.game.up_player_free_walls})</h1>
                <div style={props.session.current == props.user.email ? "border: solid 10px red;" : 'border: solid 10px white;'} >
                    <For each={ARRAY_OF_IDS}>
                        {(i) => { return <Row row={i} session={props.session as QuoridorSession} ws={props.ws} user={props.user} /> }}
                    </For>
                </div>
                <h1 style={currentRed(props.session.down_player)}>{props.session.down_player} ({props.session.game.down_player_free_walls})</h1>
                <div class="box">
                    <button
                        class="std_btn"
                        style={moveCallbackName() === 'move' ? "color:red;" : ''}
                        onClick={() => setMoveCallbackName('move')}
                    >
                        PlayerMove
                    </button>
                    <button
                        class="std_btn"
                        style={moveCallbackName() === 'hW' ? "color:red;" : ''}
                        onClick={() => setMoveCallbackName('hW')}
                    >
                        Horizontal Border
                    </button>
                    <button
                        class="std_btn"
                        style={moveCallbackName() === 'vW' ? "color:red;" : ''}
                        onClick={() => setMoveCallbackName('vW')}
                    >
                        Vertical Border
                    </button>
                </div>
            </div>
        </div>
    )
}


const json_game_example = {
    "up_player": "a",
    "down_player": "b",
    "game": {
        "up_player": [1, 4],
        "down_player": [8, 4],
        "up_player_free_walls": 8,
        "down_player_free_walls": 8,
        "vertical_walls": [[5, 5]],
        "horizontal_walls": [[2, 2]]
    },
    "turn": 0,
    "current": "b",
    "winner": null,
    "only_player_moves_allowed": false
}

const json_game_example2 = {
    "up_player": "a",
    "down_player": "b",
    "game": {
        "up_player": [1, 4],
        "down_player": [7, 4],
        "up_player_free_walls": 7,
        "down_player_free_walls": 7,
        "vertical_walls": [[5, 5], [1, 3], [0, 4]],
        "horizontal_walls": [[2, 2], [1, 4],],
    },
    "turn": 0,
    "current": "b",
    "winner": null,
    "only_player_moves_allowed": false
}

export { json_game_example, json_game_example2 }