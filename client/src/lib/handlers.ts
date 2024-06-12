import {
  ReadyEvent,
  Piece,
  GameUpdateEvent,
  AckEvent,
  ErrorEvent,
  PreviewEvent,
  Context,
} from "@/types";
export function handleAckEvent(_: Context<AckEvent>) {}

export function handleReady(context: Context<ReadyEvent>) {
  context.setReady(true);
}

export function handleGameUpdate(context: Context<GameUpdateEvent>) {
  const { ev, board, setTurn, setPreview, setBoard } = context;
  const { board: gameBoard, turn } = ev.d.game;
  for (let i = 0; i < 64; i++) {
    const row = Math.floor(i / 8);
    const col = i % 8;
    const piece = gameBoard[i];
    if (piece !== null) {
      const color = piece === "White" ? Piece.White : Piece.Black;
      board[row][col] = color;
    }
  }
  setBoard(board);
  setTurn(turn === "White" ? Piece.White : Piece.Black);
  setPreview(undefined);
}

export function handleErrorEvent(_: Context<ErrorEvent>) {}

export function handlePreviewEvent(context: Context<PreviewEvent>) {
  const { ev, setPreview } = context;
  setPreview(ev.d.changed);
}
