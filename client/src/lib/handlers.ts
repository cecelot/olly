import {
  ReadyEvent,
  Piece,
  GameUpdateEvent,
  AckEvent,
  ErrorEvent,
  PreviewEvent,
  Context,
  GameAbortEvent,
  GameEndEvent,
} from "@/types";
import toast from "react-hot-toast";

export function handleAckEvent(_: Context<AckEvent>) {}

export function handleReady(context: Context<ReadyEvent>) {
  context.setReady(true);
}

export function handleGameAbort(context: Context<GameAbortEvent>) {
  if (!context.aborted) {
    context.setAborted(true);
    toast.success("Game aborted. Redirecting to home page...", {
      duration: 5000,
    });
    setTimeout(() => {
      window.location.href = "/";
    }, 5000);
  }
}

export function handleGameEnd(context: Context<GameEndEvent>) {
  const { ev } = context;
  if (!context.aborted) {
    context.setAborted(true);
    toast.success(
      `${ev.d.winner} won the game with a score of ${ev.d.points} / ${ev.d.total}!`,
      { duration: 10_000 },
    );
  }
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
