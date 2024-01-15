import { createSignal } from "solid-js";

export default function Login() {
  const [username, setUsername] = createSignal<string>("");
  const [password, setPassword] = createSignal<string>("");

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
      if (res.status === 200) {
        window.location.href = "/";
      }
    };
    main();
  };

  return (
    <main class="text-center mx-auto p-4">
      <form class="flex flex-col mx-auto space-y-3 max-w-60">
        <input
          placeholder="Username"
          class="bg-slate-800 text-gray-100 rounded-lg p-3"
          onChange={(e) => setUsername(e.currentTarget.value)}
        ></input>
        <input
          placeholder="Password"
          class="bg-slate-800 text-gray-100 rounded-lg p-3"
          type="password"
          onChange={(e) => setPassword(e.currentTarget.value)}
        ></input>
        <button
          onClick={(e) => {
            e.preventDefault();
            onClick();
          }}
          class="bg-green-400 hover:bg-green-500 transition-all text-slate-950 rounded-lg p-3"
        >
          Login
        </button>
      </form>
    </main>
  );
}
