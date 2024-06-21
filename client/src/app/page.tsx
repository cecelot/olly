import Link from "next/link";
import { Button } from "@headlessui/react";
import GameList from "@/components/GameList";
import PendingGameList from "@/components/PendingGameList";

export default async function Home() {
  return (
    <main className="text-center mx-auto pt-5">
      <section className="space-y-5 py-10">
        <h1 className="font-bold text-6xl text-text">Othello</h1>
        <h2 className="font-normal text-subtext0 text-xl">
          Play online with friends or against the computer!
        </h2>
        <div className="flex-row space-x-5 pt-5">
          <Link href="/new">
            <Button className="text-text border-2 border-green hover:bg-mantle transition-all rounded-lg p-3">
              New Game
            </Button>
          </Link>
          <Link href="/join">
            <Button className="text-text border-2 border-teal hover:bg-mantle transition-all rounded-lg p-3">
              Join Game
            </Button>
          </Link>
        </div>
      </section>
      <GameList />
      <br />
      <PendingGameList />
      <div className="mb-10"></div>
    </main>
  );
}
