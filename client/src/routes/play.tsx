import { Accessor, For, Show, createEffect, createSignal } from "solid-js";
import { createWS } from "@solid-primitives/websocket";
import cookie from "cookie";
import { Board, Piece, Event, Member } from "~/types";
import {
  handleAckEvent,
  handleErrorEvent,
  handleGameUpdate,
  handlePreviewEvent,
  handleReady,
} from "~/handlers";
import Square from "~/components/Square";
import { currentUser } from "~/lib/currentUser";
import { A } from "@solidjs/router";

const UUID_REGEX =
  /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/;

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

interface LiveBoardProps {
  gameId: Accessor<string | null>;
}

function LiveBoard(props: LiveBoardProps) {
  const board = createBoard();
  const [turn, setTurn] = createSignal<Piece>(Piece.Black);
  const [color, setColor] = createSignal<Piece>(Piece.Black);
  const [token, setToken] = createSignal<string>();
  const [ws, setWebSocket] = createSignal<WebSocket>();
  const [preview, setPreview] = createSignal<Array<[number, number]>>();

  const stringifyPiece = (piece: Piece) =>
    piece === Piece.Black ? "Black" : "White";

  createEffect(() => {
    const ws = createWS("ws://0.0.0.0:3000/live");
    setWebSocket(ws);
    ws.addEventListener("message", (e) => {
      const data: Event = JSON.parse(e.data);
      const handlers = {
        1: handleAckEvent,
        2: handleReady,
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
        gameId: props.gameId,
        setTurn,
        setPreview,
        setColor,
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
    <div class="flex flex-row flex-wrap-reverse">
      <div class="flex flex-col max-h-screen items-center p-5">
        <For each={board}>
          {(arr, row) => (
            <div class="flex flex-row">
              <For each={arr}>
                {([piece], col) => (
                  <Square
                    piece={piece()}
                    turn={turn()}
                    color={color()}
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
                            id: props.gameId(),
                            x: col(),
                            y: row(),
                            piece: stringifyPiece(color()),
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
                            id: props.gameId(),
                            x: col(),
                            y: row(),
                            piece: stringifyPiece(color()),
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
        <p class="text-text pt-5">Turn: {turn() === 0 ? "Black" : "White"}</p>
        <Show when={token() !== undefined}>
          <p class="text-text pt-5">Token: {token()}</p>
        </Show>
        <p class="text-text pt-5">Game ID: {props.gameId()}</p>
      </div>
    </div>
  );
}

export default function Play() {
  const [user, setUser] = createSignal<Member | null | undefined>(undefined);
  const [gameId, setGameId] = createSignal<string | null>(null);

  createEffect(() => {
    (async () => {
      setUser(await currentUser());
      setGameId(new URLSearchParams(window.location.search).get("gameId"));
    })();
  });

  return (
    <Show
      when={user() && gameId()?.match(UUID_REGEX)}
      fallback={(() => {
        if (user() === undefined || gameId() === null) {
          return (
            <main class="text-center mx-auto p-4">
              <h3 class="text-lg text-subtext0">Loading...</h3>
            </main>
          );
        } else if (user() === null) {
          window.location.href = "/login";
        } else {
          return (
            <main class="text-center mx-auto p-4">
              <h3 class="text-lg text-subtext0">Invalid game ID provided</h3>
              <p class="my-4">
                <A href="/" class="text-mauve hover:text-pink transition-all">
                  {"["}Home{"]"}
                </A>
              </p>
            </main>
          );
        }
      })()}
    >
      <LiveBoard gameId={gameId} />
    </Show>
  );
}
