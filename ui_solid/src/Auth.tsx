import { Setter } from "solid-js";
import { login, registerUser, registerGuest, UserContext } from "./functions/auth";
import { QuoridorSession } from "./functions/game_quoridor";
import { startTransition, finishTransition } from "./Transition";


export function UserSignIn(props: {
    contextSetter: Setter<UserContext | null>,
    setWS: Setter<WebSocket | null>,
    setSession: Setter<QuoridorSession | null>,
}) {
    let username: any, password: any;
    function loginClick() {
        startTransition(true);
        login(
            username.value, password.value,
            props.contextSetter,
            props.setWS,
            props.setSession,
            finishTransition
        );
        password.value = null
    }
    return (
        <div style="flex-basis: 50%;" onKeyPress={(e) => { if (e.key == 'Enter') loginClick() }}>
            <h1>Sign In:</h1>
            <div >
                <input type="text" ref={username} placeholder="Username" />
                <input type="password" ref={password} placeholder="Password" />
                <button class="std_btn" onClick={loginClick}>Login</button>
            </div>
        </div>
    )
}

export function GuestSignIn( props:{
    contextSetter: Setter<UserContext | null>,
    setWS: Setter<WebSocket | null>,
    setSession: Setter<QuoridorSession | null>,
}) {
    let username: any;
    function newGuestClick() {
        startTransition(true);
        registerGuest(username.value, props.contextSetter, props.setWS, props.setSession, finishTransition);
        username.value = null
    }
    return (
        <div style="flex-basis: 50%;" onKeyPress={(e) => { if (e.key == 'Enter') newGuestClick() }}>
            <h1>Sign In as Guest</h1>
            <div>
                <input type="text" ref={username} placeholder="Username" />
                <button class="std_btn" onClick={newGuestClick}>Register as Guest:</button>
            </div>
        </div>
    )
}

export function UserCreation(props: {
        contextSetter: Setter<UserContext | null>,
        setWS: Setter<WebSocket | null>,
        setSession: Setter<QuoridorSession | null>,
}) {
    let username: any, password: any, password2: any, email: any;
    function newUserClick() {
        if (password.value != password2.value) return alert('Passwords do not match!');
        startTransition(true);
        registerUser(
            username.value, password.value, email.value,
            props.contextSetter, props.setWS, props.setSession,
            finishTransition
        );
        [username, password, password2, email].forEach((el) => el.value = null);
    }
    return (
        <div style="flex-basis: 50%;" onKeyPress={(e) => { if (e.key == 'Enter') newUserClick() }}>
            <h1>Create New Account:</h1>
            <div>
                <input type="text" ref={username} placeholder="Username" />
                <input type="password" ref={password} placeholder="Password" />
                <input type="password" ref={password2} placeholder="Repeat password" />
                <input type="email" ref={email} placeholder="Email" />
                <button class="std_btn" onClick={newUserClick}>Register User</button>
            </div>
        </div>
    )
}