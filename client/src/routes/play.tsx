import { For, Show, createEffect, createSignal } from "solid-js";
import { createWS } from "@solid-primitives/websocket";
import cookie from "cookie";
import { Board, Piece, Event, Member } from "~/types";
import {
  handleAckEvent,
  handleErrorEvent,
  handleGameCreate,
  handleGameUpdate,
  handlePreviewEvent,
  handleReady,
} from "~/handlers";
import Square from "~/components/Square";
import { currentUser } from "~/lib/currentUser";

function createBoard(): Board {
  let board: Board = [];
  for (let i = 0; i < 8; i++) {
    board[i] = [];
    for (let j = 0; j < 8; j++) {
      board[i][j] = createSignal<Piece>();
    }
  }
  return board;
}

function LiveBoard() {
  const board = createBoard();
  const [turn, setTurn] = createSignal<Piece>(Piece.Black);
  const [token, setToken] = createSignal<string>();
  const [gameId, setGameId] = createSignal<string>("...");
  const [ws, setWebSocket] = createSignal<WebSocket>();
  const [preview, setPreview] = createSignal<Array<[number, number]>>();

  createEffect(() => {
    const ws = createWS("ws://0.0.0.0:3000/live");
    setWebSocket(ws);
    ws.addEventListener("message", (e) => {
      const data: Event = JSON.parse(e.data);
      const handlers = {
        1: handleAckEvent,
        2: handleReady,
        3: handleGameCreate,
        4: handleGameUpdate,
        5: handlePreviewEvent,
        6: handleErrorEvent,
      } as const;
      handlers[data.op]({
        ws,
        //@ts-expect-error
        ev: data as any,
        board,
        token,
        setGameId,
        setTurn,
        setPreview,
      });
    });
    setToken(cookie.parse(document.cookie).sid);
    ws.send(
      JSON.stringify({
        op: 6,
        t: token(),
        d: {
          type: "Identify",
        },
      })
    );
  });

  return (
    <div class="flex flex-row flex-wrap-reverse bg-slate-900">
      <div class="flex flex-col max-h-screen items-center p-5">
        <For each={board}>
          {(arr, row) => (
            <div class="flex flex-row">
              <For each={arr}>
                {([piece], col) => (
                  <Square
                    piece={piece()}
                    turn={turn()}
                    preview={preview()?.some(
                      ([x, y]) => x === col() && y === row()
                    )}
                    row={row()}
                    col={col()}
                    onMouseEnter={() => {
                      ws()!!.send(
                        JSON.stringify({
                          op: 7,
                          t: token(),
                          d: {
                            type: "Place",
                            id: gameId(),
                            x: col(),
                            y: row(),
                            piece: turn() === Piece.Black ? "Black" : "White",
                          },
                        })
                      );
                    }}
                    onMouseLeave={() => {
                      setPreview(undefined);
                    }}
                    onClick={() => {
                      ws()!!.send(
                        JSON.stringify({
                          op: 2,
                          t: token(),
                          d: {
                            type: "Place",
                            id: gameId(),
                            x: col(),
                            y: row(),
                            piece: turn() === Piece.Black ? "Black" : "White",
                          },
                        })
                      );
                    }}
                  />
                )}
              </For>
            </div>
          )}
        </For>
      </div>
      <div>
        <Show when={gameId() !== undefined}>
          <p class="text-white pt-5">
            Turn: {turn() === 0 ? "Black" : "White"}
          </p>
        </Show>
        <Show when={token() !== undefined}>
          <p class="text-white pt-5">Token: {token()}</p>
        </Show>
        <Show when={gameId() !== undefined}>
          <p class="text-white pt-5">Game ID: {gameId()}</p>
        </Show>
      </div>
    </div>
  );
}

export default function Play() {
  const [user, setUser] = createSignal<Member | null | undefined>(undefined);

  createEffect(() => {
    (async () => {
      setUser(await currentUser());
    })();
  });

  return (
    <Show
      when={user()}
      fallback={(() => {
        if (user() === null) {
          window.location.href = "/login";
        }
        return <></>;
      })()}
    >
      <LiveBoard />
    </Show>
  );
}
