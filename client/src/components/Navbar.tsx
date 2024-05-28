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
    <nav class="flex flex-row justify-center p-5 bg-base text-mauve">
      <div class="space-x-3">
        <A href="/" class="hover:text-pink transition-all">
          {"["}Home{"]"}
        </A>
        <A href="/about" class="hover:text-pink transition-all">
          {"["}About{"]"}
        </A>
        <A href="/stats" class="text-mauve hover:text-pink transition-all">
          {"["}Stats{"]"}
        </A>
        <A href="/account" class="text-mauve hover:text-pink transition-all">
          {"["}Account{"]"}
        </A>
        <Show
          when={user()}
          fallback={
            <>
              <A
                href="/login"
                class="text-mauve hover:text-pink transition-all"
              >
                {"["}Login{"]"}
              </A>
              <A
                href="/register"
                class="text-mauve hover:text-pink transition-all"
              >
                {"["}Register{"]"}
              </A>
            </>
          }
        >
          <button
            onClick={() => logout()}
            class="hover:text-pink transition-all"
          >
            {"["}Logout{"]"}
          </button>
        </Show>
      </div>
    </nav>
  );
}
