import { Accessor, Setter } from "solid-js";
import { UserContext, logout } from "./functions/auth";
import { profileSwitch } from "./Profile";
import { QuoridorSession } from "./functions/game_quoridor";

type btn = {
    click: CallableFunction,
    text: string,
    class?: string,
    style?: string,
    disabled?: boolean,
}

function ButtonBase(props: btn) {
    return (
        <button
            style={props.style}
            class={props.class ? props.class : "std_btn"}
            onClick={() => props.click()}
            disabled={props.disabled}
        >
            {props.text}
        </button>
    )
}

export function Nav(props: { 
    left?: btn, 
    right?: btn, 
    showLogo?: boolean,
    setWS: Setter<WebSocket|null>,
    setSession: Setter<QuoridorSession|null> 
    context:[Accessor<UserContext|null>, Setter<UserContext|null>] }) {
    const username = props.context[0]();
    return (
        <nav>
            <div class="nav_block">
                <div class="box">{!username ? <></> : <ButtonBase text={username.username} click={() => profileSwitch(true)} />}</div>
                <div class="box">{!props.left ? <></> : <ButtonBase {...props.left} />}</div>
            </div>
            <div class="image_nav"></div>
            <div class="nav_block">
                <div class="box">{!username ? <></> : <ButtonBase text="Logout" click={() => logout(props.context[1], props.setWS, props.setSession)} />}</div>
                <div class="box">{!props.right ? <></> : <ButtonBase {...props.right} />}</div>
            </div>
        </nav>
    )
}
