import {
  GameCreateEvent,
  ReadyEvent,
  Board,
  Piece,
  GameUpdateEvent,
  AckEvent,
  ErrorEvent,
  PreviewEvent,
  Context,
} from "~/types";

export function handleAckEvent(_: Context<AckEvent>) {}

export function handleReady(context: Context<ReadyEvent>) {
  const { ws, ev, setToken } = context;
  setToken(ev.d.token);
  ws.send(
    JSON.stringify({
      op: 1,
      t: ev.d.token,
      d: { type: "Create", guest: "unicorn" },
    })
  );
}

export function handleGameCreate(context: Context<GameCreateEvent>) {
  const { ws, ev, setGameId, token } = context;
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
