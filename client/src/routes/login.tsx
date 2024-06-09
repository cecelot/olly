import { createEffect, createSignal } from "solid-js";
import CustomToaster from "~/components/CustomToaster";
import { currentUser } from "~/lib";
import showToast from "~/lib/showToast";
import { Member } from "~/types";

export default function Login() {
  const [username, setUsername] = createSignal<string>("");
  const [password, setPassword] = createSignal<string>("");
  const [user, setUser] = createSignal<Member | null>(null);

  createEffect(() => {
    (async () => {
      setUser(await currentUser());
      if (user()) {
        window.location.href = "/";
      }
    })();
  });

  const onClick = () => {
    const main = async () => {
      const res = await fetch("http://localhost:3000/login", {
        credentials: "include",
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          username: username(),
          password: password(),
        }),
      });
      const to = decodeURIComponent(
        new URLSearchParams(window.location.search).get("to") || "/"
      );
      if (res.status === 200) {
        window.location.href = decodeURIComponent(
          new URLSearchParams(window.location.search).get("to") || "/"
        );
      } else {
        showToast(
          {
            403: {
              text: "The entered password is incorrect. Please try again.",
              kind: "error",
            },
            404: {
              text: "That user doesn't exist!",
              kind: "error",
            },
          },
          res.status
        );
      }
    };
    main();
  };

  return (
    <main class="text-center mx-auto m-4">
      <CustomToaster />
      <h1 class="text-3xl font-semibold mb-5">Login</h1>
      <form class="flex flex-col mx-auto space-y-3 max-w-60">
        <input
          placeholder="Username"
          class="bg-crust text-subtext0 rounded-lg p-3"
          onChange={(e) => setUsername(e.currentTarget.value)}
        ></input>
        <input
          placeholder="Password"
          class="bg-crust text-subtext0 rounded-lg p-3"
          type="password"
          onChange={(e) => setPassword(e.currentTarget.value)}
        ></input>
        <button
          onClick={(e) => {
            e.preventDefault();
            onClick();
          }}
          class="text-text border-2 border-green hover:bg-mantle transition-all rounded-lg p-3"
        >
          Login
        </button>
      </form>
    </main>
  );
}
