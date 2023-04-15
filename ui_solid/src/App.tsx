import { LoginView, LobbiesView, GameView, FinishedGameView, finishedGame, getQuoridorWS } from "./AppViews";
import { LoadAnimation } from "./LoadAnimation";
import { userContext, getContext } from './functions/auth';
import { Switch, Match } from "solid-js";
import { Message, message, showMessage } from "./Message";
import { Profile, showProfile } from "./Profile";
import { Transition, inTransition, isLoading } from "./Transition";
export const IS_MOBILE = navigator.userAgent.toLowerCase().match(/mobile/i);


function App() {
  getContext();
  
  if (IS_MOBILE) {
    showMessage("Quoridor is not yet optimized to be used on mobile device. Use at your own risk!")
  }

  return (
    <>
      <Switch>
        <Match when={finishedGame()}>
          <FinishedGameView />
        </Match>
        <Match when={!inTransition && !isLoading}>
          <Transition />
        </Match>
        <Match when={!userContext()}>
          <LoadAnimation />
          <LoginView />
        </Match>
        <Match when={userContext() && !getQuoridorWS()}>
          <LobbiesView />
        </Match>
        <Match when={userContext() && getQuoridorWS()}>
          <GameView />
        </Match>
      </Switch>
      {showProfile() ? <Profile /> : <></>}
      {message() ? <Message /> : <></>}
    </>
  );
}

export default App;
