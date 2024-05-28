import { A } from "@solidjs/router";
import { For, Show, createEffect, createSignal } from "solid-js";
import { currentUser, simpleGet } from "~/lib";
import { Game, Member } from "~/types";

export default function GameList() {
  const [games, setGames] = createSignal<Game[] | null>(null);
  const [user, setUser] = createSignal<Member | null>(null);

  createEffect(() => {
    (async () => {
      setUser(await currentUser());
      if (user()) {
        setGames(await simpleGet("/@me/games"));
      }
    })();
  });

  return (
    <section>
      <h3 class="font-bold text-text pb-2">Active Games</h3>
      <div class="max-h-96 overflow-y-scroll">
        <Show
          when={(games()?.length || 0) > 0}
          fallback={
            <p class="text-subtext0">
              No active games! Create one using the button above.
            </p>
          }
        >
          <ul class="space-y-3">
            <For each={games()}>
              {(game) => {
                return (
                  <li class="flex-col">
                    <A
                      href={`/play?gameId=${game.id}`}
                      class="text-blue hover:underline-offset-4 hover:underline hover:text-sapphire"
                    >{`Game against ${game.opponent}`}</A>
                    <p class="text-sm text-subtext1">{game.id}</p>
                  </li>
                );
              }}
            </For>
          </ul>
        </Show>
      </div>
    </section>
  );
}
