import { createSignal } from "solid-js";
import CustomToaster from "~/components/CustomToaster";
import showToast from "~/lib/showToast";

export default function Register() {
  const [username, setUsername] = createSignal<string>("");
  const [password, setPassword] = createSignal<string>("");

  const onClick = () => {
    const main = async () => {
      const res = await fetch("http://localhost:3000/register", {
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
      showToast(
        {
          201: {
            text: `Account created! You will be redirected to the login page shortly.`,
            kind: "success",
            afterClose: () => {
              window.location.href = "/login";
            },
          },
          409: {
            text: "That username is already taken!",
            kind: "error",
          },
        },
        res.status
      );
    };
    main();
  };

  return (
    <main class="text-center mx-auto p-4">
      <CustomToaster />
      <h1 class="text-3xl font-semibold mb-5">Register</h1>
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
          Register
        </button>
      </form>
    </main>
  );
}
