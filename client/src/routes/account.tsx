import { Show, createEffect, createSignal } from "solid-js";
import FriendsList from "~/components/friends/FriendsList";
import MergedFriendRequests from "~/components/friends/MergedFriendRequests";
import { currentUser, call } from "~/lib";
import { Member } from "~/types";

export default function Account() {
  const [user, setUser] = createSignal<Member | null>(null);
  const [friendUsername, setFriendUsername] = createSignal<string>("");

  const sendFriendRequest = (
    e: MouseEvent & { currentTarget: HTMLButtonElement }
  ) => {
    e.preventDefault();
    (async () => {
      const resp = await call(`/users/${friendUsername()}/friend`, "POST");
      if (resp.status === 201) {
        alert("Friend request sent!");
      } else {
        alert(`An error occurred: ${JSON.stringify(await resp.json())}`);
      }
    })();
  };

  createEffect(() => {
    (async () => {
      setUser(await currentUser());
      if (user() === null) {
        window.location.href = `/login?to=${encodeURIComponent("/account")}`;
      }
    })();
  });

  return (
    <main class="text-center mx-auto p-5 max-w-3xl">
      <Show when={user()}>
        <div class="pb-10">
          <h1 class="text-2xl text-text font-extrabold">
            Hi, {user()?.username}!
          </h1>
          <h6 class="text-sm text-subtext1">{user()?.id}</h6>
        </div>
        <form class="flex flex-col mx-auto space-y-3 max-w-80">
          <input
            placeholder="Username"
            class="bg-crust text-subtext0 rounded-lg p-3"
            onChange={(e) => setFriendUsername(e.currentTarget.value)}
          ></input>
          <button
            onClick={sendFriendRequest}
            class="text-text border-2 border-green hover:bg-mantle transition-all rounded-lg p-3"
          >
            Send Friend Request
          </button>
        </form>
        <section class="grid grid-cols-2">
          <FriendsList />
          <MergedFriendRequests />
        </section>
      </Show>
    </main>
  );
}
