import { getChatWS, setChatWS, userContext } from "../App";
import { pushNewMsg } from "../Chat";
import { CHAT_CHANNEL, createSocket } from "./utils"

export interface Message {
    user: string,
    message: string,
    timestamp: number,
}

export function sendMsg(msg: string) {
    if (!userContext()) return;
    getChatWS()?.send(msg)
}

export function createChat(id: string) {
    console.log('Building chat socket on ' + id);
    let socket = createSocket(CHAT_CHANNEL + id,
        (message) => {
            console.log("chat message", message);
            try {
                pushNewMsg(JSON.parse(message as string));
            } catch (e) { console.log(e); }
        },
        (_) => { setChatWS(null) }
    )
    setChatWS(socket)
}

export function updateScroll(element: HTMLElement | null) {
    if (!element) return;
    element.scrollTop = element.scrollHeight;
}