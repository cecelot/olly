import { createEffect, createSignal } from "solid-js";
import { A } from "@solidjs/router";
import { call, currentUser } from "~/lib";
import { Member } from "~/types";
import { Dynamic } from "solid-js/web";

const logout = async () => {
  await call("/logout", "POST");
  window.location.href = "/";
};

const options = {
  logout: () => (
    <button onClick={() => logout()} class="hover:text-pink transition-all">
      {"["}Logout{"]"}
    </button>
  ),
  loginRegister: () => (
    <>
      <A href="/login" class="text-mauve hover:text-pink transition-all">
        {"["}Login{"]"}
      </A>
      <A href="/register" class="text-mauve hover:text-pink transition-all">
        {"["}Register{"]"}
      </A>
    </>
  ),
  loading: () => <></>,
} as const;

export default function Navbar() {
  const [user, setUser] = createSignal<Member | null>(null);
  const [selected, setSelected] = createSignal<keyof typeof options>("loading");

  createEffect(() => {
    (async () => {
      setUser(await currentUser());
      if (user() === null) setSelected("loginRegister");
      else setSelected("logout");
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
        <Dynamic component={options[selected()]} />
      </div>
    </nav>
  );
}
