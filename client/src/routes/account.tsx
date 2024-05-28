import { For, Show, createEffect, createSignal } from "solid-js";
import { currentUser } from "~/lib/currentUser";
import { Member } from "~/types";

export default function Friends() {
  const [friendUsername, setFriendUsername] = createSignal<string>("");
  const [user, setUser] = createSignal<Member | null | undefined>(undefined);
  const [incomingFriendRequests, setIncomingFriendRequests] = createSignal<
    { sender: string }[]
  >([]);
  const [outgoingFriendRequests, setOutgoingFriendRequests] =
    createSignal<{ recipient: string }[]>();
  const [friends, setFriends] = createSignal<{ username: string }[]>();

  const sendFriendRequest = () => {
    (async () => {
      const resp = await fetch(
        `http://localhost:3000/users/${friendUsername()}/friend`,
        {
          credentials: "include",
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
        }
      );
      if (resp.status === 201) {
        alert("Friend request sent!");
      } else {
        alert(`An error occurred: ${JSON.stringify(await resp.json())}`);
      }
    })();
  };

  const replyToFriendRequest = (sender: string, accept: boolean) => {
    const end = accept ? "accept" : "deny";
    (async () => {
      const resp = await fetch(
        `http://localhost:3000/@me/friends/${sender}/${end}`,
        {
          credentials: "include",
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
        }
      );
      if (resp.status === 200) {
        alert(`Friend request ${end}ed!`);
      } else {
        alert(`An error occurred: ${JSON.stringify(await resp.json())}`);
      }
    })();
  };

  createEffect(() => {
    (async () => {
      setUser(await currentUser());
      const incomingFriendRequests = await fetch(
        "http://localhost:3000/@me/friends/incoming",
        {
          credentials: "include",
          method: "GET",
        }
      );
      const { message: incoming } = await incomingFriendRequests.json();
      const outgoingFriendRequests = await fetch(
        "http://localhost:3000/@me/friends/outgoing",
        {
          credentials: "include",
          method: "GET",
        }
      );
      const { message: outgoing } = await outgoingFriendRequests.json();
      const currentFriends = await fetch("http://localhost:3000/@me/friends", {
        credentials: "include",
        method: "GET",
      });
      const { message: friends } = await currentFriends.json();
      setFriends(friends);
      setIncomingFriendRequests(incoming);
      setOutgoingFriendRequests(outgoing);
      if (user() === null) {
        window.location.href = `/login?to=${encodeURIComponent("/account")}`;
      }
    })();
  });

  return (
    <main class="text-center mx-auto p-5 max-w-md">
      <Show when={user() !== undefined}>
        <div class="pb-10">
          <h3 class="text-2xl text-text font-extrabold">
            Hi, {user()?.username}!
          </h3>
          <h6 class="text-sm text-subtext1">{user()?.id}</h6>
        </div>
        <form class="flex flex-col mx-auto space-y-3 max-w-80">
          <input
            placeholder="Username"
            class="bg-crust text-subtext0 rounded-lg p-3"
            onChange={(e) => setFriendUsername(e.currentTarget.value)}
          ></input>
          <button
            onClick={(e) => {
              e.preventDefault();
              sendFriendRequest();
            }}
            class="text-text border-2 border-green hover:bg-mantle transition-all rounded-lg p-3"
          >
            Send Friend Request
          </button>
        </form>
        <section class="p-10">
          <h4 class="font-bold text-text">Your Friends</h4>
          <div class="max-h-56 p-2 overflow-y-scroll">
            <Show when={(friends()?.length || 0) > 0}>
              <For each={friends()}>
                {(friend) => {
                  return (
                    <div class="flex flex-col space-y-1">
                      <p class="text-text">{friend.username}</p>
                      <button class="text-mauve hover:text-pink transition-all">
                        {"["}Remove{"]"}
                      </button>
                    </div>
                  );
                }}
              </For>
            </Show>
          </div>
        </section>
        <section class="p-10">
          <h4 class="font-bold text-text">Friend Requests</h4>
          <div class="max-h-56 p-2 overflow-y-scroll space-y-3">
            <Show when={(outgoingFriendRequests()?.length || 0) > 0}>
              <For each={outgoingFriendRequests()}>
                {(request) => {
                  return (
                    <div class="flex flex-col space-y-1">
                      <p class="text-text">{request.recipient}</p>
                      <button class="text-mauve hover:text-pink transition-all">
                        {"["}Cancel{"]"}
                      </button>
                    </div>
                  );
                }}
              </For>
            </Show>
            <Show when={(incomingFriendRequests()?.length || 0) > 0}>
              <For each={incomingFriendRequests()}>
                {(request) => {
                  return (
                    <div class="flex flex-col space-y-3">
                      <p class="text-text">{request.sender}</p>
                      <div class="flex flex-row space-x-3 mx-auto items-center">
                        <button
                          onClick={(e) => {
                            e.preventDefault();
                            replyToFriendRequest(request.sender, true);
                          }}
                          class="text-mauve hover:text-pink transition-all"
                        >
                          {"["}Accept{"]"}
                        </button>
                        <button
                          onClick={(e) => {
                            e.preventDefault();
                            replyToFriendRequest(request.sender, false);
                          }}
                          class="text-mauve hover:text-pink transition-all"
                        >
                          {"["}Deny{"]"}
                        </button>
                      </div>
                    </div>
                  );
                }}
              </For>
            </Show>
          </div>
        </section>
      </Show>
    </main>
  );
}
