import { For, Show, createEffect, createSignal } from "solid-js";
import { call, simpleGet } from "~/lib";
import { IncomingFriendRequest } from "~/types";

export default function IncomingFriendRequests() {
  const [incomingFriendRequests, setIncomingFriendRequests] =
    createSignal<IncomingFriendRequest[]>();

  const onClick = (sender: string, accept: boolean) => {
    return async (e: MouseEvent & { currentTarget: HTMLButtonElement }) => {
      e.preventDefault();
      const end = accept ? "accept" : "deny";
      (async () => {
        const resp = await call(`/@me/friends/${sender}/${end}`, "POST");
        if (resp.status === 200) {
          alert(`Friend request ${end}ed!`);
        } else {
          alert(`An error occurred: ${JSON.stringify(await resp.json())}`);
        }
      })();
    };
  };

  createEffect(() => {
    (async () =>
      setIncomingFriendRequests(await simpleGet("/@me/friends/incoming")))();
  });

  return (
    <Show when={(incomingFriendRequests()?.length || 0) > 0}>
      <For each={incomingFriendRequests()}>
        {(request) => {
          return (
            <li class="flex flex-row space-x-3 justify-center">
              <p class="text-text">{request.sender}</p>
              <button
                onClick={onClick(request.sender, true)}
                class="text-mauve hover:text-pink transition-all"
              >
                {"["}Accept{"]"}
              </button>
              <button
                onClick={onClick(request.sender, false)}
                class="text-mauve hover:text-pink transition-all"
              >
                {"["}Deny{"]"}
              </button>
            </li>
          );
        }}
      </For>
    </Show>
  );
}
