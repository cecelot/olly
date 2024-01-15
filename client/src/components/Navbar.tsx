import { Show, createEffect, createSignal } from "solid-js";
import { A } from "@solidjs/router";
import { Member } from "~/types";
import { currentUser } from "~/lib/currentUser";

export default function Navbar() {
  const logout = async () => {
    await fetch("http://localhost:3000/logout", {
      credentials: "include",
      mode: "cors",
      method: "POST",
    });
    window.location.href = "/";
  };

  const [user, setUser] = createSignal<Member | null>(null);

  createEffect(() => {
    (async () => {
      setUser(await currentUser());
    })();
  });

  return (
    <nav class="flex flex-row justify-center p-5 bg-gray-900">
      <div class="space-x-3">
        <A
          href="/about"
          class="text-green-400 hover:text-green-500 transition-all"
        >
          {"["}About{"]"}
        </A>
        <A
          href="/stats"
          class="text-green-400 hover:text-green-500 transition-all"
        >
          {"["}Stats{"]"}
        </A>
        <A
          href="/friends"
          class="text-green-400 hover:text-green-500 transition-all"
        >
          {"["}Friends{"]"}
        </A>
        <Show
          when={user()}
          fallback={
            <A
              href="/login"
              class="text-green-400 hover:text-green-500 transition-all"
            >
              {"["}Login{"]"}
            </A>
          }
        >
          <button
            onClick={() => logout()}
            class="text-green-400 hover:text-green-500 transition-all"
          >
            {"["}Logout{"]"}
          </button>
        </Show>
      </div>
    </nav>
  );
}
