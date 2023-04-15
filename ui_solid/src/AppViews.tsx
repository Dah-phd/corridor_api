import { Nav } from "./Nav";
import { Footer } from "./Footer";
import { createSignal, Switch, Match, onMount, onCleanup, Accessor, } from "solid-js";
import { finishTransition, startTransition } from "./Transition";

// LOGIN VIEW

import { GuestSignIn, UserCreation, UserSignIn } from "./Auth";

export function LoginView() {
    const [signInState, showSignIn] = createSignal(true);
    return (
        <>
            <Nav
                left={{ text: "Sign In", click: () => { showSignIn(true) } }}
                right={{ text: "Create New Account", click: () => { showSignIn(false) } }}
            />
            <div class='full_screen_centered'>
                <div class="form_container">
                    {signInState() ? <UserSignIn /> : <UserCreation />}
                    <GuestSignIn />
                </div >
            </div>
            <Footer />
        </>
    )
}

// LOBBIES VIEW
export const [getQuoridorWS, setQuoridorWS] = createSignal<null | [WebSocket, Accessor<QuoridorSession | null>]>(null);
export const [showSpinner, switchSpinner] = createSignal(false);

import { Lobbies } from "./Lobby"

export function LobbiesView() {
    function MatchMaking() {
        function cancelLobby(ev: KeyboardEvent) { if (ev.key === "Escape") { } }
        onMount(() => { document.addEventListener('keydown', cancelLobby) });
        onCleanup(() => { document.removeEventListener('keydown', cancelLobby) });
        return (
            <>
                <div class="covering-panel" ><div class="spin"></div></div>
                <Nav />
                <h1>Looking for opponent ...</h1><hr /><h3>Press Esc to cancel</h3>
            </>
        )
    }
    return (
        < Switch >
            <Match when={showSpinner()}>
                <MatchMaking />
            </Match>
            <Match when={!showSpinner()}>
                <Nav
                    left={{ text: "Game VS CPU", click: () => { hostQuoriodrCPU(finishTransition); startTransition() } }}
                    right={{ text: "Create Lobby", click: () => { hostQuoriodrGame(finishTransition); startTransition() } }}
                />
                <div class="full_screen_centered">
                    <Lobbies />
                </div>
                <Footer />
            </Match>
        </Switch >
    )
}

// GAME VIEW

import { QuoridorBoard } from "./Quoridor";
import { QuoridorSession } from "./functions/game_quoridor";
import { concede } from "./functions/game_quoridor";
import { MessageBoard, showMessages, switchShowMessages, unreadMessages, setUnreadMessages } from "./Chat";

export function GameView() {
    let ws = getQuoridorWS();
    if (ws === null) {
        return;
    }
    return (
        <>
            <Nav
                right={{ text: 'Concede', style: 'color:red;', click: () => { concede() } }}
                left={{
                    text: !showMessages() ? `Open Chat ${unreadMessages() ? unreadMessages() : ""}` : "Back to Game",
                    style: unreadMessages() ? "color: red;" : "",
                    click: () => { switchShowMessages(!showMessages()); if (showMessages()) setUnreadMessages(0) }
                }}
            />
            <QuoridorBoard />
            <div class="full_screen_centered">
                <MessageBoard />
            </div>
        </>
    )
}


// FINISHED GAME VIEW

import { FinishedQuoridor } from "./QuoridorEnd";
import { hostQuoriodrCPU, hostQuoriodrGame } from "./functions/lobbies";
export const [finishedGame, setFinishedGame] = createSignal<null | QuoridorSession>(null);

function FinishedGameSelector(props: { session: QuoridorSession | null }) {
    if (!props.session) return
    return (
        props.session ? <FinishedQuoridor session={props.session} /> : <></>
    )
}

export function FinishedGameView() {
    const backToLobbies = () => { setFinishedGame(null) };
    function backToLobbiesListener(ev: KeyboardEvent) { if (ev.key === "Escape") { backToLobbies() } }
    onMount(() => { document.addEventListener('keydown', backToLobbiesListener) });
    onCleanup(() => { document.removeEventListener('keydown', backToLobbiesListener) });
    return (
        <>
            <Nav right={{ text: "Back to Lobbies", click: backToLobbies }} />
            <FinishedGameSelector session={finishedGame()} />
            <div class="std_btn" onClick={backToLobbies}> Go to Lobbies </div>
        </>
    )
}