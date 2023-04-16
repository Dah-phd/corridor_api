import { createEffect, createSignal, For } from "solid-js";
import { Message, sendMsg, updateScroll } from "./functions/chat";

export const [unreadMessages, setUnreadMessages] = createSignal(0);

export const [newMsg, pushNewMsg] = createSignal<Message | null>(null);

export const [showMessages, switchShowMessages] = createSignal(false);

function getOldMessages(board_id: string): Array<Message> {
    const chat = sessionStorage.getItem(board_id)
    return chat ? JSON.parse(chat) : []
}

export function messageNotification() {
    if (!showMessages()) setUnreadMessages(unreadMessages() + 1);
}

function pushMessage(msg: Message | null): Array<Message> | undefined {
    const gameID = null;
    if (!gameID) return;
    const board_id = `chat_${JSON.stringify(gameID)}`
    const chat = getOldMessages(board_id);
    if (msg) chat.push(msg);
    sessionStorage.setItem(board_id, JSON.stringify(chat));
    return chat
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