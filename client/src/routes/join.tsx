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
          placeholder="Game ID"
          class="bg-crust text-subtext0 rounded-lg p-3"
          onChange={(e) => setGameId(e.currentTarget.value)}
        ></input>
        <button
          onClick={(e) => {
            e.preventDefault();
            onClick();
          }}
          class="text-text border-2 border-green hover:bg-mantle transition-all rounded-lg p-3"
        >
          Join
        </button>
      </form>
    </main>
  );
}
