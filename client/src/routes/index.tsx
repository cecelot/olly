import { A } from "@solidjs/router";

export default function Home() {
  return (
    <>
      <main class="text-center mx-auto pt-40">
        <section class="space-y-5">
          <h1 class="font-bold text-gray-100 text-6xl">Othello</h1>
          <h2 class="font-normal text-gray-300 text-xl">
            The two-player strategy board game based on Reversi
          </h2>
          <div class="flex-row space-x-5">
            <button class="bg-green-400 hover:bg-green-500 transition-all text-slate-950 p-3 rounded-lg">
              <A href="/play">New Game</A>
            </button>
            <button class=" bg-green-200 hover:bg-green-300 transition-all text-slate-950 p-3 rounded-lg">
              <A href="/join">Join Game</A>
            </button>
          </div>
        </section>
      </main>
    </>
  );
}
