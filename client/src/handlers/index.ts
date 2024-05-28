import { simpleGet } from "~/lib";
import {
  ReadyEvent,
  Piece,
  GameUpdateEvent,
  AckEvent,
  ErrorEvent,
  PreviewEvent,
  Context,
} from "~/types";

export function handleAckEvent(_: Context<AckEvent>) {}

export function handleReady(context: Context<ReadyEvent>) {
  const { token, gameId, ws, setColor } = context;
  ws.send(
    JSON.stringify({
      op: 3,
      t: token(),
      d: {
        type: "Join",
        id: gameId(),
      },
    })
  );
  (async () => {
    const { host } = await simpleGet(`/game/${gameId()}`);
    const { id } = await simpleGet("/@me");
    setColor(host === id ? Piece.Black : Piece.White);
  })();
}

export function handleGameUpdate(context: Context<GameUpdateEvent>) {
  const { ev, board, setTurn, setPreview } = context;
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
  setPreview(undefined);
}

export function handleErrorEvent(_: Context<ErrorEvent>) {}

export function handlePreviewEvent(context: Context<PreviewEvent>) {
  const { ev, setPreview } = context;
  setPreview(ev.d.changed);
}
