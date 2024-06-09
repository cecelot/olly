import { createSignal } from "solid-js";
import CustomToaster from "~/components/CustomToaster";
import { createGame } from "~/lib/createGame";

export default function New() {
  const [opponent, setOpponent] = createSignal<string>("");

  const onClick = (e: MouseEvent & { currentTarget: HTMLButtonElement }) => {
    e.preventDefault();
    createGame(opponent());
  };

  return (
    <main class="text-center mx-auto p-4">
      <CustomToaster />
      <form class="flex flex-col mx-auto space-y-3 max-w-80">
        <input
          placeholder="Opponent"
          class="bg-crust text-subtext0 rounded-lg p-3"
          onChange={(e) => setOpponent(e.currentTarget.value)}
        ></input>
        <button
          onClick={onClick}
          class="text-text border-2 border-green hover:bg-mantle transition-all rounded-lg p-3"
        >
          Play
        </button>
      </form>
    </main>
  );
}
