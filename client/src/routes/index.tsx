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
        setGames(games);
      }
    };
    main();
  });

  return (
    <main class="text-center mx-auto pt-5">
      <section class="space-y-5 py-10">
        <h1 class="font-bold text-6xl text-text">Othello</h1>
        <h2 class="font-normal text-subtext0 text-xl">
          The two-player strategy board game based on Reversi
        </h2>
        <div class="flex-row space-x-5 pt-5">
          <A
            href="/new"
            class="text-text border-2 border-green hover:bg-mantle transition-all rounded-lg p-3"
          >
            New Game
          </A>
          <A
            href="/join"
            class="text-text border-2 border-teal hover:bg-mantle transition-all rounded-lg p-3"
          >
            Join Game
          </A>
        </div>
      </section>
      <h3 class="font-bold text-text pb-2">Active Games</h3>
      <section class="max-h-96 overflow-y-scroll">
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
                <ul class="flex-col space-x-5 pb-3">
                  <li>
                    <A
                      href={`/play?gameId=${game.id}`}
                      class="text-blue hover:underline-offset-4 hover:underline hover:text-sapphire"
                    >{`Game against ${game.opponent}`}</A>
                  </li>
                  <li>
                    <p class="text-sm text-subtext1">{game.id}</p>
                  </li>
                </ul>
              );
            }}
          </For>
        </Show>
      </section>
    </main>
  );
}
