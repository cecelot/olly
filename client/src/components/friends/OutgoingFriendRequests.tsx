import { For, Show, createEffect, createSignal } from "solid-js";
import { call, simpleGet } from "~/lib";
import showToast from "~/lib/showToast";
import { OutgoingFriendRequest } from "~/types";

export default function OutgoingFriendRequests() {
  const [outgoingFriendRequests, setOutgoingFriendRequests] =
    createSignal<OutgoingFriendRequest[]>();

  const cancelFriendRequest = (recipient: string) => {
    return async (e: MouseEvent & { currentTarget: HTMLButtonElement }) => {
      e.preventDefault();
      (async () => {
        const res = await call(`/@me/friends/outgoing/${recipient}`, "DELETE");
        showToast(
          {
            200: {
              text: `Cancelled friend request to ${recipient}.`,
              kind: "success",
            },
          },
          res.status
        );
      })();
    };
  };

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
              <button
                class="text-mauve hover:text-pink transition-all"
                onClick={cancelFriendRequest(request.recipient)}
              >
                {"["}Cancel{"]"}
              </button>
            </li>
          );
        }}
      </For>
    </Show>
  );
}
