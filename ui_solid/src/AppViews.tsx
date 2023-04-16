import { Nav } from "./Nav";
import { Footer } from "./Footer";
import { createSignal, Switch, Match, onMount, onCleanup, Accessor, Setter, createEffect, } from "solid-js";
import { finishTransition, startTransition } from "./Transition";
import { UserContext } from "./functions/auth";
import { hostQuoriodrCPU, hostQuoriodrGame } from "./functions/lobbies";
import { QuoridorBoard } from "./Quoridor";
import { QuoridorSession } from "./functions/game_quoridor";
import { concede } from "./functions/game_quoridor";
import { MessageBoard, showMessages, switchShowMessages, unreadMessages, setUnreadMessages } from "./Chat";
import { showMessage } from "./Message";

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
import { setCookie } from "./functions/utils";

export function LobbiesView(props: {
    context: [Accessor<UserContext | null>, Setter<UserContext | null>],
    setSession: Setter<QuoridorSession | null>,
    setWS: Setter<WebSocket | null>
},
) {
    let user = props.context[0]() as UserContext;
    setCookie(user.authToken)
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

export function GameView(props: {
    context: [Accessor<UserContext | null>, Setter<UserContext | null>],
    ws: WebSocket,
    session: QuoridorSession,
    setWS: Setter<WebSocket | null>
}) {
    const [rightBtn, setRightBtn] = createSignal("Concede");
    const rightFN = () => {
        if (rightBtn() == 'Concede') concede(props.ws as WebSocket)
        else { props.ws.close(); props.setWS(null); }
    }

    createEffect(() => {
        if (props.session.winner) {
            showMessage(`Winner is ${props.session.winner}`);
            setRightBtn("Back to Lobbies");
        } else setRightBtn("Concede")
    })

    return (
        <>
            <Nav
                context={props.context}
                right={{ text: rightBtn(), style: 'color:red;', click: rightFN }}
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
