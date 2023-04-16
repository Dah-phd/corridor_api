import { pushNewMsg, messageNotification } from "../Chat";
import { createSignal, from } from "solid-js";
import { CHAT_CHANNEL } from "./utils";


export const ACTIVE_CHAT = "activeChat";

export interface Message {
    user: string,
    message: string,
    timestamp: number,
}

export interface ChatID {
    MatchID: string
}

export function sendMsg(msg: string) {
    //TODO!
}


export function connectChat() {
    //TODO!
}

export function updateScroll(element: HTMLElement | null) {
    if (!element) return;
    element.scrollTop = element.scrollHeight;
}