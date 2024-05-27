import { A } from "@solidjs/router";

export default function About() {
  return (
    <main class="text-center mx-auto p-5 max-w-md">
      <div class="leading-8 space-y-5">
        <p>
          Olly is a free and open source game client and server for{" "}
          <a
            class="text-blue hover:underline-offset-4 hover:underline hover:text-sapphire"
            href="https://en.wikipedia.org/wiki/Reversi"
          >
            Othello
          </a>
          , a two-player strategy board game. The game is played on an 8x8 board
          with pieces that are black on one side and white on the other.
        </p>

        <p>
          The goal is to have the majority of pieces of your color when the last
          playable square is filled. Players take turns placing pieces on the
          board to capture their opponent's pieces. Captured pieces are flipped
          to the capturing player's color. The game ends when there are no more
          legal moves, and the player with the most pieces of their color on the
          board wins.
        </p>

        <p class="my-4">
          <A href="/" class="text-mauve hover:text-pink transition-all">
            {"["}Home{"]"}
          </A>
        </p>
      </div>
    </main>
  );
}
