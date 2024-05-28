import { For, Show, createEffect, createSignal } from "solid-js";
import { call, simpleGet } from "~/lib";
import { OutgoingFriendRequest } from "~/types";

export default function OutgoingFriendRequests() {
  const [outgoingFriendRequests, setOutgoingFriendRequests] =
    createSignal<OutgoingFriendRequest[]>();

  createEffect(() => {
    (async () =>
      setOutgoingFriendRequests(await simpleGet("/@me/friends/outgoing")))();
  });

  return (
    <Show when={(outgoingFriendRequests()?.length || 0) > 0}>
      <For each={outgoingFriendRequests()}>
        {(request) => {
          return (
            <li class="flex flex-row space-x-1 justify-center">
              <p class="text-text">{request.recipient}</p>
              <button class="text-mauve hover:text-pink transition-all">
                {"["}Cancel{"]"}
              </button>
            </li>
          );
        }}
      </For>
    </Show>
  );
}
