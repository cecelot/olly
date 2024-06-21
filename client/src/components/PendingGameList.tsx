"use client";

import { TOAST_ERROR_OPTIONS, TOAST_SUCCESS_OPTIONS } from "@/lib";
import call from "@/lib/call";
import usePendingGames from "@/lib/hooks/usePendingGames";
import useUser from "@/lib/hooks/useUser";
import toast from "react-hot-toast";

export default function PendingGameList() {
  const { games, isLoading } = usePendingGames();
  const data = useUser();

  const acceptInvite = (id: string, opponent: string) => {
    return async (e: React.MouseEvent<HTMLButtonElement, MouseEvent>) => {
      e.preventDefault();
      (async () => {
        const res = await call(`/@me/games/${id}/accept`, "POST");
        if (res.status === 200) {
          window.location.href = `/play?gameId=${id}`;
        } else {
          toast.error(
            `Failed to accept ${opponent}'s invite`,
            TOAST_ERROR_OPTIONS
          );
        }
      })();
    };
  };

  const declineInvite = (id: string, opponent: string) => {
    return async (e: React.MouseEvent<HTMLButtonElement, MouseEvent>) => {
      e.preventDefault();
      (async () => {
        const res = await call(`/@me/games/${id}/decline`, "DELETE");
        if (res.status === 200) {
          toast.success(
            `Invite from ${opponent} declined`,
            TOAST_SUCCESS_OPTIONS
          );
        } else {
          toast.error(
            `Failed to decline the invite from ${opponent}`,
            TOAST_ERROR_OPTIONS
          );
        }
      })();
    };
  };

  const cancelInvite = (id: string, opponent: string) => {
    return async (e: React.MouseEvent<HTMLButtonElement, MouseEvent>) => {
      e.preventDefault();
      (async () => {
        const res = await call(`/@me/games/${id}/cancel`, "DELETE");
        if (res.status === 204) {
          toast.success(
            `Invitation to ${opponent} canceled`,
            TOAST_SUCCESS_OPTIONS
          );
        } else {
          toast.error(
            `Failed to cancel the invite to ${opponent}`,
            TOAST_ERROR_OPTIONS
          );
        }
      })();
    };
  };

  if (isLoading) return <></>;

  return (
    <section>
      <h3 className="font-bold text-text pb-2">Invites</h3>
      <div className="max-h-96 overflow-y-scroll">
        {(games?.length || 0) > 0 ? (
          <ul className="space-y-3">
            {games?.map((game) => {
              return (
                <li className="flex-col" key={game.id}>
                  <p className="text-text font-medium">{`Game against ${game.opponent}`}</p>
                  <p className="text-sm text-subtext1">{game.id}</p>
                  {game.opponent === game.host ? (
                    <div className="space-x-3">
                      <button
                        onClick={acceptInvite(game.id, game.opponent)}
                        className="text-mauve hover:text-pink transition-all"
                      >
                        {"["}Accept{"]"}
                      </button>
                      <button
                        onClick={declineInvite(game.id, game.opponent)}
                        className="text-mauve hover:text-pink transition-all"
                      >
                        {"["}Decline{"]"}
                      </button>
                    </div>
                  ) : (
                    <button
                      onClick={cancelInvite(game.id, game.opponent)}
                      className="text-mauve hover:text-pink transition-all"
                    >
                      {"["}Cancel{"]"}
                    </button>
                  )}
                </li>
              );
            })}
          </ul>
        ) : (
          <p className="text-subtext0 max-w-screen-sm mx-auto">
            {data.user?.username ? (
              <>
                No pending games! Create one using the button above, or ask a
                friend to create a game with <code>{data.user?.username}</code>{" "}
                as their opponent.
              </>
            ) : (
              <>Log in to view invites to games!</>
            )}
          </p>
        )}
      </div>
    </section>
  );
}
