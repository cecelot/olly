import { A } from "@solidjs/router";
import GameList from "~/components/GameList";

export default function Home() {
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
      <GameList />
    </main>
  );
}
