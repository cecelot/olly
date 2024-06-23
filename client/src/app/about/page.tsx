import Link from "next/link";

export default function About() {
  return (
    <main className="text-center text-text mx-auto p-5 max-w-md leading-8 space-y-5">
      <p>
        Olly is a free and open source game client and server for{" "}
        <a
          className="text-blue hover:underline-offset-4 hover:underline hover:text-sapphire"
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
        board to capture their opponent&apos;s pieces. Captured pieces are
        flipped to the capturing player&apos;s color. The game ends when there
        are no more legal moves, and the player with the most pieces of their
        color on the board wins.
      </p>
      <p className="my-4">
        <Link href="/" className="text-mauve hover:text-pink transition-all">
          {"["}Home{"]"}
        </Link>
      </p>
    </main>
  );
}
