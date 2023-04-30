import { login, registerUser, registerGuest } from "./functions/auth";
import { startTransition, finishTransition } from "./Transition";
import { showMessage } from "./Message";


export function UserSignIn() {
    let username: any, password: any;
    function loginClick() {
        startTransition("Loading ...");
        login(username.value, password.value, finishTransition);
        password.value = null
    }
    return (
        <div style="flex-basis: 50%;" onKeyPress={(e) => { if (e.key == 'Enter') loginClick() }}>
            <h1>Sign In:</h1>
            <div>
                <div><input type="text" ref={username} placeholder="Email" /></div>
                <div><input type="password" ref={password} placeholder="Password" /></div>
                <br />
                <button class="std_btn" onClick={loginClick}>Login</button>
            </div>
        </div>
    )
}

export function GuestSignIn() {
    let username: any;
    function newGuestClick() {
        startTransition("Loading ...");
        registerGuest(username.value, finishTransition);
        username.value = null
    }
    return (
        <div style="flex-basis: 50%;" onKeyPress={(e) => { if (e.key == 'Enter') newGuestClick() }}>
            <h1>Sign In as Guest</h1>
            <div>
                <div><input type="text" ref={username} placeholder="Username" /></div>
                <br />
                <button class="std_btn" onClick={newGuestClick}>Register as Guest</button>
            </div>
        </div>
    )
}

export function UserCreation() {
    let username: any, password: any, password2: any, email: any;
    function newUserClick() {
        if (password.value != password2.value) return showMessage('Passwords do not match!');
        startTransition("Loading ...");
        registerUser(username.value, password.value, email.value, finishTransition);
        [username, password, password2, email].forEach((el) => el.value = null);
    }
    showMessage("Your email will be visible to other players as identifier!!!")
    return (
        <div style="flex-basis: 50%;" onKeyPress={(e) => { if (e.key == 'Enter') newUserClick() }}>
            <h1>Create New Account:</h1>
            <div>
                <div><input type="text" ref={username} placeholder="Username" /></div>
                <div><input type="password" ref={password} placeholder="Password" /></div>
                <div><input type="password" ref={password2} placeholder="Repeat password" /></div>
                <div><input type="email" ref={email} placeholder="Email" /></div>
                <br />
                <button class="std_btn" onClick={newUserClick}>Register User</button>
            </div>
        </div>
    )
}