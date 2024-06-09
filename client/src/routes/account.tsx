import { Show, createEffect, createSignal } from "solid-js";
import CustomToaster from "~/components/CustomToaster";
import FriendsList from "~/components/friends/FriendsList";
import MergedFriendRequests from "~/components/friends/MergedFriendRequests";
import { currentUser, call } from "~/lib";
import showToast from "~/lib/showToast";
import { Member } from "~/types";

export default function Account() {
  const [user, setUser] = createSignal<Member | null>(null);
  const [friendUsername, setFriendUsername] = createSignal<string>("");

  const sendFriendRequest = (
    e: MouseEvent & { currentTarget: HTMLButtonElement }
  ) => {
    e.preventDefault();
    (async () => {
      const res = await call(`/users/${friendUsername()}/friend`, "POST");
      showToast(
        {
          201: {
            text: `Sent friend request to ${friendUsername()}!`,
            kind: "success",
          },
          404: {
            text: `That user doesn't exist! Make sure their username is spelled correctly.`,
            kind: "error",
          },
        },
        res.status
      );
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
      <CustomToaster />
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
