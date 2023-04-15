import { UserContext } from "./auth";

export interface QuoridorGame {
    up_player: [number, number],
    down_player: [number, number],
    up_player_free_walls: number,
    down_player_free_walls: number,
    vertical_walls: Array<[number, number]>,       // (row, col)
    horizontal_walls: Array<[number, number]>,    // (row, col)
}

export interface QuoridorSession {
    up_player: string,
    down_player: string,
    game: QuoridorGame,
    turn: number,
    current: string,
    winner: string | null,
    only_player_moves_allowed: boolean
}

type Position = { row: number, col: number };


interface PlayerMove {
    QuoridorWallV?: Position
    QuoridorWallH?: Position
    QuoridorMove?: Position
}

type MoveFunc = (row: number, col: number) => PlayerMove

export function concede(ws: WebSocket) {
    if (ws.readyState !== WebSocket.OPEN) return;
    ws.send("Concede")
}

export function makeQuoridorMove(row: number, col: number, game: QuoridorSession, ws: WebSocket, user:UserContext, callback: MoveFunc) {
    if (ws.readyState !== WebSocket.OPEN) return;
    if (!user || game.current != user.email) return;
    ws.send(JSON.stringify(callback(row, col)));
}

export function quoridorMove(row: number, col: number): PlayerMove {
    console.log(row, col, 'm')
    return { QuoridorMove: { row: row, col: col } }
}

export function quoridorHWall(row: number, col: number): PlayerMove {
    console.log(row, col, 'h')
    return { QuoridorWallH: { row: row, col: col } }
}

export function quoridorVWall(row: number, col: number): PlayerMove {
    console.log(row, col, 'v')
    return { QuoridorWallV: { row: row, col: col } }
}


export function getBorderClass(row: number, column: number, game: QuoridorGame) {
    const borders = [];
    for (let idx in game.horizontal_walls) {
        let h_border = game.horizontal_walls[idx];
        if (h_border[1] == column || h_border[1] == column - 1) {
            if (h_border[0] == row) borders.push(' bottom_red');
            if (h_border[0] == row - 1) borders.push(' top_red');
        }
    }
    for (let idx in game.vertical_walls) {
        let v_boarder = game.vertical_walls[idx];
        if (v_boarder[0] == row || v_boarder[0] == row - 1) {
            if (v_boarder[1] == column) borders.push(' right_red');
            if (v_boarder[1] == column - 1) borders.push(' left_red');
        }
    }
    return borders.join(' ')
}

export function setPlayer(row: number, column: number, game: QuoridorGame): string {
    let className = " ";
    if (game.up_player[0] == row && game.up_player[1] == column) { className += '🦀' }
    if (game.down_player[0] == row && game.down_player[1] == column) { className += '🐍' }
    return className
}


export function getQuoridorHoverPlayerMove(row: number, col: number, session: QuoridorSession) {
    const player = session.up_player == session.current ? session.game.up_player : session.game.down_player;
    if (player[0] == row && player[1] == col) return 'player_move_blocked';
    if (player && tileInRange(row, col, player))
        return !isBlocked(row, col, player, session.game) ? 'player_move' : 'player_move_blocked';
    return ""
}

function tileInRange(row: number, col: number, player: Array<number>): boolean {
    if (row == player[0] && (col == player[1] - 1 || col == player[1] + 1)) return true
    if (col == player[1] && (row == player[0] - 1 || row == player[0] + 1)) return true
    return false
}

function isBlocked(row: number, col: number, player: Array<number>, game: QuoridorGame) {
    if (row == player[0]) {
        const move_col = col > player[1] ? player[1] : col;
        for (let wall_id in game.vertical_walls) {
            const wall = game.vertical_walls[wall_id];
            if (move_col == wall[1] && (row == wall[0] || row - 1 == wall[0]))
                return true
        }
    }
    if (col == player[1]) {
        const move_row = row > player[0] ? player[0] : row;
        for (let wall_id in game.horizontal_walls) {
            const wall = game.horizontal_walls[wall_id];
            if (move_row == wall[0] && (col == wall[1] || col - 1 == wall[1]))
                return true
        }
    }
    return false
}

export function shouldHoverHWall(row: number, col: number, session: QuoridorGame) {
    if (row == 8 || col == 8) return false;
    for (const position in session.vertical_walls) {
        const vwall = session.vertical_walls[position];
        if (vwall[0] == row && vwall[1] == col) return false;
    }
    for (const position in session.horizontal_walls) {
        const hwall = session.horizontal_walls[position];
        if (hwall[0] == row) {
            if (hwall[1] == col) { return false };
            if (hwall[1] == col + 1) { return false };
            if (hwall[1] == col - 1) { return false };
        };
    };
    return true
}
export function shouldHoverVWall(row: number, col: number, session: QuoridorGame) {
    if (row == 8 || col == 8) return false;
    for (const position in session.horizontal_walls) {
        const hwall = session.horizontal_walls[position];
        if (hwall[0] == row && hwall[1] == col) return false;
    }
    for (const position in session.vertical_walls) {
        const vwall = session.vertical_walls[position];
        if (vwall[1] == col) {
            if (vwall[0] == row) { return false };
            if (vwall[0] == row + 1) { return false };
            if (vwall[0] == row - 1) { return false };
        }

    }
    return true
}
