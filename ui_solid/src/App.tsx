import { LoginView, LobbiesView, GameView } from "./AppViews";
import { LoadAnimation } from "./LoadAnimation";
import { getContext, UserContext } from './functions/auth';
import { Switch, Match, createSignal } from "solid-js";
import { Message, message, showMessage } from "./Message";
import { Profile, showProfile } from "./Profile";
import { Transition, inTransition, isLoading } from "./Transition";
import { QuoridorSession } from "./functions/game_quoridor";
export const IS_MOBILE = navigator.userAgent.toLowerCase().match(/mobile/i);

export const [quoridorSession, setQuoridorSession] = createSignal<QuoridorSession | null>(null);
export const [getQuoridorWS, setQuoridorWS] = createSignal<null | WebSocket>(null);


function App() {
  const [userContext, contextSetter] = createSignal<UserContext | null>(null);
  getContext(contextSetter, setQuoridorWS, setQuoridorSession);

  if (IS_MOBILE) {
    showMessage("Quoridor is not yet optimized to be used on mobile device. Use at your own risk!")
  }

  return (
    <>
      <Switch>
        <Match when={!inTransition && !isLoading}>
          <Transition />
        </Match>
        <Match when={!userContext()}>
          <LoadAnimation />
          <LoginView
            context={[userContext, contextSetter]}
          />
        </Match>
        <Match when={!getQuoridorWS()}>
          <LobbiesView
            context={[userContext, contextSetter]}
          />
        </Match>
        <Match when={getQuoridorWS() && quoridorSession()}>
          <GameView
            context={[userContext, contextSetter]}
            ws={getQuoridorWS() as WebSocket}
            session={quoridorSession() as QuoridorSession}
          />
        </Match>
      </Switch>
      {showProfile() ? <Profile user={userContext() as UserContext} /> : <></>}
      {message() ? <Message /> : <></>}
    </>
  );
}

export default App;
