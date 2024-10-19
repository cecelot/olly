"use client";

import Link from "next/link";
import useGames from "@/lib/hooks/useGames";
import useUser from "@/lib/hooks/useUser";

export default function GameList() {
  const data = useUser();
  const { games, isLoading } = useGames();

  if (isLoading) return <></>;

  return (
    <section>
      <h3 className="font-bold text-text pb-2">Active Games</h3>
      <div className="max-h-96 overflow-y-scroll">
        {(games?.filter((game) => !game.ended).length || 0) > 0 ? (
          <ul className="space-y-3">
            {games
              ?.filter((game) => !game.ended)
              .map((game) => {
                return (
                  <li className="flex-col" key={game.id}>
                    <Link
                      href={`/play?gameId=${game.id}`}
                      className="text-blue hover:underline-offset-4 hover:underline hover:text-sapphire"
                    >{`Game against ${game.opponent}`}</Link>
                    <p className="text-sm text-subtext1">{game.id}</p>
                  </li>
                );
              })}
          </ul>
        ) : (
          <p className="text-subtext0">
            {data.user?.username ? (
              <>No active games! Create one using the button above.</>
            ) : (
              <>Log in to view invites to games!</>
            )}
          </p>
        )}
      </div>
    </section>
  );
}
