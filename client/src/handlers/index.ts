import { Accessor, Setter } from "solid-js";
import {
  GameCreateEvent,
  ReadyEvent,
  Board,
  Piece,
  GameUpdateEvent,
  AckEvent,
  ErrorEvent,
} from "~/types";

export function handleAckEvent(
  _ws: WebSocket,
  _ev: AckEvent,
  _setToken: Setter<string>,
  _setGameId: Setter<string>,
  _setTurn: Setter<Piece>,
  _board: Board,
  _token: Accessor<string>
) {}

export function handleReady(
  ws: WebSocket,
  ev: ReadyEvent,
  setToken: Setter<string>,
  _setGameId: Setter<string>,
  _setTurn: Setter<Piece>,
  _board: Board,
  _token: Accessor<string | undefined>
) {
  setToken(ev.d.token);
  ws.send(
    JSON.stringify({
      op: 1,
      t: ev.d.token,
      d: { type: "Create", guest: "unicorn" },
    })
  );
}

export function handleGameCreate(
  ws: WebSocket,
  ev: GameCreateEvent,
  _setToken: Setter<string>,
  setGameId: Setter<string>,
  _setTurn: Setter<Piece>,
  _board: Board,
  token: Accessor<string | undefined>
) {
  setGameId(ev.d.id);
  ws.send(
    JSON.stringify({
      op: 3,
      t: token(),
      d: {
        type: "Join",
        id: ev.d.id,
      },
    })
  );
}

export function handleGameUpdate(
  _ws: WebSocket,
  ev: GameUpdateEvent,
  _setToken: Setter<string>,
  _setGameId: Setter<string>,
  setTurn: Setter<Piece>,
  board: Board,
  _token: Accessor<string | undefined>
) {
  const { board: gameBoard, turn } = ev.d.game;
  for (let i = 0; i < 64; i++) {
    const row = Math.floor(i / 8);
    const col = i % 8;
    const piece = gameBoard[i];
    if (piece !== null) {
      const color = piece === "White" ? Piece.White : Piece.Black;
      const [, setPiece] = board[row][col];
      setPiece(() => color);
    }
  }
  setTurn(turn === "White" ? Piece.White : Piece.Black);
}

export function handleErrorEvent(
  _ws: WebSocket,
  ev: ErrorEvent,
  _setToken: Setter<string>,
  _setGameId: Setter<string>,
  _setTurn: Setter<Piece>,
  _board: Board,
  _token: Accessor<string | undefined>
) {
  console.error(ev.d.message);
}
