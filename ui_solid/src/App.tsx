import { LoginView, LobbiesView, GameView } from "./AppViews";
import { LoadAnimation } from "./LoadAnimation";
import { getContext, UserContext } from './functions/auth';
import { Switch, Match, createSignal, Accessor } from "solid-js";
import { Message, message, showMessage } from "./Message";
import { Profile, showProfile } from "./Profile";
import { Transition, inTransition, isLoading } from "./Transition";
import { QuoridorSession } from "./functions/game_quoridor";
export const IS_MOBILE = navigator.userAgent.toLowerCase().match(/mobile/i);


function App() {
  const [getQuoridorWS, setQuoridorWS] = createSignal<null | WebSocket>(null);
  const [userContext, contextSetter] = createSignal<UserContext | null>(null);
  const [quoridorSession, setQuoridorSession] = createSignal<QuoridorSession | null>(null);
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
            setWS={setQuoridorWS}
            setSession={setQuoridorSession}
          />
        </Match>
        <Match when={!getQuoridorWS()}>
          <LobbiesView
            context={[userContext, contextSetter]}
            setSession={setQuoridorSession}
            setWS={setQuoridorWS}
          />
        </Match>
        <Match when={getQuoridorWS() && quoridorSession()}>
          <GameView
            context={[userContext, contextSetter]}
            ws={getQuoridorWS()}
            session={quoridorSession()}
          />
        </Match>
      </Switch>
      {showProfile() ? <Profile user={userContext() as UserContext} /> : <></>}
      {message() ? <Message /> : <></>}
    </>
  );
}

export default App;
