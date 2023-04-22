import { createEffect, createSignal, For } from "solid-js";
import { Message, sendMsg, updateScroll } from "./functions/chat";

export const [unreadMessages, setUnreadMessages] = createSignal(0);

export const [newMsg, pushNewMsg] = createSignal<Message | null>(null);

export const [showMessages, switchShowMessages] = createSignal(false);

const CHAT = 'chat_history'

export function messageNotification() {
    if (!showMessages()) setUnreadMessages(unreadMessages() + 1);
}

function pushMessage(msg: Message | null): Array<Message> {
    const chat = sessionStorage.getItem(CHAT);
    let chat_history = chat ? JSON.parse(chat) : [];
    if (msg) chat_history.push(msg);
    sessionStorage.setItem(CHAT, JSON.stringify(chat_history));
    return chat_history
}

const [lastSender, setLastSender] = createSignal<string>();

export function MessageBoard() {
    let scrollableEl: any;
    createEffect(() => { updateScroll(scrollableEl); newMsg(); })
    return (
        <div class="chat_wrapper" style={!showMessages() ? "display:none;" : ""}>
            <div class="chat_box" ref={scrollableEl}>
                <For each={pushMessage(newMsg())}>
                    {(msg: Message) => {
                        const Component = <MessageText msg_sender={msg.user} msg={msg.message} />
                        setLastSender(msg.user)
                        return Component
                    }}
                </For>
            </div>
            <SendMessage />
        </div>
    )
}

function MessageText(props: { msg_sender: string, msg: string }) {
    if (false) return <div class="msg_own_wrapper"><div class="msg_own">{props.msg}</div></div>

    if (lastSender() == props.msg_sender) return <div class="msg">{props.msg}</div>

    return (
        <>
            <div class="msg_user"><span class="msg_user"> <span style="font-size:small;">From: </span>{props.msg_sender}</span></div>
            <div class="msg">{props.msg}</div>
        </>
    )
}

function SendMessage() {
    let inputEl: any;
    const inputHandler = () => {
        if (inputEl.value) sendMsg(inputEl.value);
        inputEl.value = null;
    }
    return (
        <div class="send_msg" onKeyPress={(el) => { if (el.key == 'Enter') { inputHandler() } }}>
            <input type="text" ref={inputEl} class="send_msg_box" />
            <div class="send_msg_btn" onClick={inputHandler}></div>
        </div>
    )
}