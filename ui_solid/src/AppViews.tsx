import { Nav } from "./Nav";
import { Footer } from "./Footer";
import { createSignal, Switch, Match, onMount, onCleanup, Accessor, Setter, createEffect, } from "solid-js";
import { finishTransition, startTransition } from "./Transition";
import { UserContext } from "./functions/auth";
import { hostQuoriodrCPU, hostQuoriodrGame } from "./functions/lobbies";

// LOGIN VIEW

import { GuestSignIn, UserCreation, UserSignIn } from "./Auth";

export function LoginView(props: {
    context: [Accessor<UserContext | null>, Setter<UserContext | null>],
    setWS: Setter<WebSocket | null>,
    setSession: Setter<QuoridorSession | null>
}) {
    const [signInState, showSignIn] = createSignal(true);
    return (
        <>
            <Nav
                context={props.context}
                left={{ text: "Sign In", click: () => { showSignIn(true) } }}
                right={{ text: "Create New Account", click: () => { showSignIn(false) } }}
            />
            <div class='full_screen_centered'>
                <div class="form_container">
                    {signInState() ? <UserSignIn
                        contextSetter={props.context[1]}
                        setWS={props.setWS}
                        setSession={props.setSession}
                    /> : <UserCreation 
                        contextSetter={props.context[1]}
                        setWS={props.setWS}
                        setSession={props.setSession}
                    />}
                    <GuestSignIn 
                        contextSetter={props.context[1]}
                        setWS={props.setWS}
                        setSession={props.setSession}
                    />
                </div >
            </div>
            <Footer />
        </>
    )
}

// LOBBIES VIEW
export const [showSpinner, switchSpinner] = createSignal(false);

import { Lobbies } from "./Lobby"

export function LobbiesView(props: {
    context: [Accessor<UserContext | null>, Setter<UserContext | null>],
    setSession: Setter<QuoridorSession | null>,
    setWS: Setter<WebSocket | null>
},
) {
    let user = props.context[0]() as UserContext;
    function MatchMaking() {
        function cancelLobby(ev: KeyboardEvent) { if (ev.key === "Escape") { } }
        onMount(() => { document.addEventListener('keydown', cancelLobby) });
        onCleanup(() => { document.removeEventListener('keydown', cancelLobby) });
        return (
            <>
                <div class="covering-panel" ><div class="spin"></div></div>
                <Nav context={props.context} />
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
                    context={props.context}
                    left={{ text: "Game VS CPU", click: () => { hostQuoriodrCPU(props.setWS, props.setSession, finishTransition); startTransition() } }}
                    right={{ text: "Create Lobby", click: () => { hostQuoriodrGame(props.setWS, props.setSession, finishTransition); startTransition() } }}
                />
                <div class="full_screen_centered">
                    <Lobbies setWS={props.setWS} setSession={props.setSession} />
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

export function GameView(props: {
    context: [Accessor<UserContext | null>, Setter<UserContext | null>],
    ws: WebSocket | null,
    session: QuoridorSession | null
}) {
    if (!props.ws) return;
    let right_btn, right_fn
    if (props.session?.winner) {
        alert(`Winner is ${props.session.winner}`);
        right_btn = "Back to Lobbies";
        right_fn = () => props.ws?.close();
    } else {
        right_btn = "Concede";
        right_fn = () => concede(props.ws as WebSocket);
    }


    return (
        <>
            <Nav
                context={props.context}
                right={{ text: right_btn , style: 'color:red;', click: right_fn }}
                left={{
                    text: !showMessages() ? `Open Chat ${unreadMessages() ? unreadMessages() : ""}` : "Back to Game",
                    style: unreadMessages() ? "color: red;" : "",
                    click: () => { switchShowMessages(!showMessages()); if (showMessages()) setUnreadMessages(0) }
                }}
            />
            <QuoridorBoard ws={props.ws} session={props.session} user={props.context[0]() as UserContext} />
            <div class="full_screen_centered">
                <MessageBoard />
            </div>
        </>
    )
}
