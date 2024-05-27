import { A } from "@solidjs/router";
import { For, Show, createEffect, createSignal } from "solid-js";

interface Game {
  id: string;
  opponent: string;
}

export default function Home() {
  const [games, setGames] = createSignal<Game[] | null>(null);

  createEffect(() => {
    const main = async () => {
      const res = await fetch("http://localhost:3000/@me/games", {
        credentials: "include",
      });
      if (res.status === 200) {
        const { message: games } = await res.json();
        console.log(games);
        setGames(games);
      }
    };
    main();
  });

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
        <section class="space-y-5 py-10">
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
        <section class="max-h-56 overflow-y-scroll">
          <h4 class="font-bold text-text pb-2">Active Games</h4>
          <Show
            when={(games()?.length || 0) > 0}
            fallback={
              <p class="text-subtext0">
                No active games! Create one using the button above.
              </p>
            }
          >
            <For each={games()}>
              {(game) => {
                return (
                  <div class="flex-row space-x-5">
                    <A
                      href={`/play?gameId=${game.id}`}
                      class="text-blue hover:underline-offset-4 hover:underline hover:text-sapphire"
                    >{`Game against ${game.opponent}`}</A>
                  </div>
                );
              }}
            </For>
          </Show>
        </section>
      </main>
    </>
  );
}
