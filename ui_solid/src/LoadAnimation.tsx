import { createSignal, Match } from "solid-js";

export function LoadAnimation() {
    return (
        <div class='tetrominos'>
            <div class='tetromino box1'></div>
            <div class='tetromino box2'></div>
            <div class='tetromino box3'></div>
            <div class='tetromino box4'></div>
        </div>
    )
}

export const [inTransition, startTransition] = createSignal(false);
export function transition() {
    startTransition(true);
    setTimeout(() => { startTransition(false) }, 500);
}

export function Transition() {
    return (
        <Match when={inTransition()}>
            <LoadAnimation />
        </Match>
    )
}