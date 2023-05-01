import { createSignal, onMount, onCleanup } from "solid-js"
import { showMessage, message } from "./Message";
import { UserContext } from "./functions/auth";
import { USER_STATS } from "./functions/utils";
import { LeaderBoardStat, UserStats } from "./functions/utils";

export const [showProfile, profileSwitch] = createSignal(false);
// const [isLoading, setterLoading] = createSignal(false);
// const afterFn = () => { setterLoading(false) };

// function updatePassword(a: any, b: any) {
//     alert("Password update not implemented")
//     //TODO!
// }

// function updPassClick(pass1: HTMLInputElement, pass2: HTMLInputElement) {
//     setterLoading(true);
//     const passNonEq = () => { showMessage("Passwords are not indentical!"); afterFn() }
//     pass1.value == pass2.value ? updatePassword(pass1.value, afterFn) : passNonEq();
//     [pass1, pass2].forEach(el => el.value = "")
// }

export function Profile(props: { user: UserContext }) {
    if (props.user.username == "GUEST") {
        showMessage("User profile does not exist!\nPlease login and try again!");
        profileSwitch(false);
        return;
    }

    function closeProfile(ev: KeyboardEvent) { if (ev.key === "Escape") { profileSwitch(false) } }
    onMount(() => { document.addEventListener('keydown', closeProfile) });
    onCleanup(() => { document.removeEventListener('keydown', closeProfile) });

    // let password: any, password2: any, email: any;
    const [stats, setStats] = createSignal<UserStats>();

    fetch(USER_STATS, { method: "get" })
        .then((response) => { if (response.ok) response.json().then(result => { setStats(result); console.log(stats()) }) })
        .catch((e) => { console.log(e); setStats() })

    return (
        <>
            <div class="systen_msg" style={message() ? "z-index: 0 !important;" : ""}>
                <p style="color:gray;">Username: <i style="color:red;font-size:large;">{props.user.username}</i></p>
                <hr />
                <LeaderBoardStat stats={stats()} />
                {/* <input type="email" ref={email} placeholder={"TODO!"} />
                <p>
                    <button class="std_btn" disabled={isLoading()} onClick={() => {alert("TODO!")}}>
                        Update Email
                    </button>
                </p>
                <hr />
                <input type="password" ref={password} placeholder="Password" />
                <input type="password" ref={password2} placeholder="Repeat password" />
                <p>
                    <button class="std_btn" disabled={isLoading()} onClick={() => { updPassClick(password, password2) }}>
                        Update Password
                    </button>
                </p> */}
                <p style="color:gray;" ><i>Press Esc to exit.</i></p>
            </div>
            <div class="covering-panel"></div>
        </>
    )
}