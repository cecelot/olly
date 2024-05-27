import { createSignal } from "solid-js";

export default function Join() {
  const [gameId, setGameId] = createSignal<string>("");

  const onClick = () => {
    window.location.href = `/play?gameId=${gameId()}`;
  };

  return (
    <main class="text-center mx-auto p-4">
      <form class="flex flex-col mx-auto space-y-3 max-w-80">
        <input
          placeholder="Game ID (shown on host's game screen)"
          class="bg-slate-800 text-gray-100 rounded-lg p-3"
          onChange={(e) => setGameId(e.currentTarget.value)}
        ></input>
        <button
          onClick={(e) => {
            e.preventDefault();
            onClick();
          }}
          class="bg-green-400 hover:bg-green-500 transition-all text-slate-950 rounded-lg p-3"
        >
          Join
        </button>
      </form>
    </main>
  );
}