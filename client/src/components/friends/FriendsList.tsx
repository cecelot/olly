import { For, Show, createEffect, createSignal } from "solid-js";
import { createGame, simpleGet } from "~/lib";
import { Friend } from "~/types";

export default function FriendsList() {
  const [friends, setFriends] = createSignal<Friend[]>();

  createEffect(() => {
    (async () => setFriends(await simpleGet("/@me/friends")))();
  });

  return (
    <section class="py-10 text-left">
      <h2 class="font-bold text-text text-center">Your Friends</h2>
      <ul class="max-h-80 py-2 overflow-y-scroll">
        <Show when={(friends()?.length || 0) > 0}>
          <For each={friends()}>
            {(friend) => {
              return (
                <li class="flex flex-row space-x-1 justify-center">
                  <p class="text-text">{friend.username}</p>
                  <button
                    onClick={() => createGame(friend.username)}
                    class="text-mauve hover:text-pink transition-all"
                  >
                    {"["}Play{"]"}
                  </button>
                  <button class="text-mauve hover:text-pink transition-all">
                    {"["}Remove{"]"}
                  </button>
                </li>
              );
            }}
          </For>
        </Show>
      </ul>
    </section>
  );
}
