import { A } from "@solidjs/router";

export default function Home() {
  const onClick = () => {
    const main = async () => {
      const res = await fetch("http://localhost:3000/game", {
        credentials: "include",
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          guest: "unicorn",
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
    <>
      <main class="text-center mx-auto pt-40">
        <section class="space-y-5">
          <h1 class="font-bold text-6xl text-text">Othello</h1>
          <h2 class="font-normal text-subtext0 text-xl">
            The two-player strategy board game based on Reversi
          </h2>
          <div class="flex-row space-x-5 pt-5">
            <button
              onClick={onClick}
              class="text-text border-2 border-green hover:bg-mantle transition-all rounded-lg p-3"
            >
              New Game
            </button>
            <button
              onClick={() => (window.location.href = "/join")}
              class="text-text border-2 border-teal hover:bg-mantle transition-all rounded-lg p-3"
            >
              Join Game
            </button>
          </div>
        </section>
      </main>
    </>
  );
}
