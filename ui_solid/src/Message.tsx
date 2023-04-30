import { createSignal } from "solid-js";

export const [message, showMessage] = createSignal<string>();
export const [messageCallback, setMessageCallback] = createSignal<() => void>()
function Ok() {
    showMessage();
    const callback = messageCallback();
    if (callback) callback();
    setMessageCallback();
}

export function Message() {

    return (
        <>
            <div class="systen_msg">
                <hr />
                <h3 >{message()}</h3>
                <button class="std_btn" onClick={Ok}>Ok</button>
                <hr />
            </div>
            <div class="covering-panel"></div>
        </>
    )
}