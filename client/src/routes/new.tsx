import { createSignal } from "solid-js";

export default function New() {
  const [opponent, setOpponent] = createSignal<string>("");

  const onClick = () => {
    const main = async () => {
      const res = await fetch("http://localhost:3000/game", {
        credentials: "include",
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          guest: opponent(),
        }),
      });
      if (res.status === 201) {
        const { message } = await res.json();
        window.location.href = `/play?gameId=${message.id}`;
      } else {
        alert(`An error occurred: ${JSON.stringify(await res.json())}`);
      }
    };
    main();
  };

  return (
    <main class="text-center mx-auto p-4">
      <form class="flex flex-col mx-auto space-y-3 max-w-80">
        <input
          placeholder="Opponent"
          class="bg-crust text-subtext0 rounded-lg p-3"
          onChange={(e) => setOpponent(e.currentTarget.value)}
        ></input>
        <button
          onClick={(e) => {
            e.preventDefault();
            onClick();
          }}
          class="text-text border-2 border-green hover:bg-mantle transition-all rounded-lg p-3"
        >
          Play
        </button>
      </form>
    </main>
  );
}
