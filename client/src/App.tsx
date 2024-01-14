import { For, Show, createSignal, createEffect } from "solid-js";
import { createWS } from "@solid-primitives/websocket";
import { Board, Piece, Event } from "./types";
import {
  handleAckEvent,
  handleErrorEvent,
  handleGameCreate,
  handleGameUpdate,
  handleReady,
} from "./handlers";
import Square from "./components/Square";

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

function App() {
  const board = createBoard();
  const [turn, setTurn] = createSignal<Piece>(Piece.Black);
  const [token, setToken] = createSignal<string>();
  const [gameId, setGameId] = createSignal<string>();
  const ws = createWS("ws://0.0.0.0:3000/live");

  createEffect(() => {
    ws.addEventListener("message", (e) => {
      const data: Event = JSON.parse(e.data);
      const handlers = {
        1: handleAckEvent,
        2: handleReady,
        3: handleGameCreate,
        4: handleGameUpdate,
        5: handleErrorEvent,
      } as const;
      console.log(data);
      handlers[data.op](
        ws,
        data as any,
        setToken,
        setGameId,
        setTurn,
        board,
        token
      );
    });
    ws.send(
      JSON.stringify({
        op: 6,
        d: {
          type: "Identify",
          username: "alaidriel",
          password: "meow",
        },
      })
    );
  });

  return (
    <div class="flex flex-col min-h-screen items-center pt-5 bg-gray-800">
      <For each={board}>
        {(arr, row) => (
          <div class="flex flex-row">
            <For each={arr}>
              {([piece], col) => (
                <Square
                  piece={piece()}
                  row={row()}
                  col={col()}
                  onClick={() => {
                    ws.send(
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
      <Show when={gameId() !== undefined}>
        <p class="text-white pt-5">Turn: {turn() === 0 ? "Black" : "White"}</p>
      </Show>
      <Show when={token() !== undefined}>
        <p class="text-white pt-5">Token: {token()}</p>
      </Show>
      <Show when={gameId() !== undefined}>
        <p class="text-white pt-5">Game ID: {gameId()}</p>
      </Show>
    </div>
  );
}

export default App;
