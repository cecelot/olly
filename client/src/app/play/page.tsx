"use client";

import {
  handleAckEvent,
  handleReady,
  handleGameUpdate,
  handlePreviewEvent,
  handleErrorEvent,
} from "@/lib/handlers";
import { Board, Piece, Event } from "@/types";
import { useEffect, useState } from "react";
import Square from "@/components/board/Square";
import useWebSocket from "react-use-websocket";
import cookie from "cookie";
import simpleGet from "@/lib/simpleGet";

const UUID_REGEX =
  /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/;

function createBoard(): Board {
  let board: Board = [];
  for (let i = 0; i < 8; i++) {
    board[i] = [];
    for (let j = 0; j < 8; j++) {
      board[i][j] = null;
    }
  }
  return board;
}

interface LiveBoardProps {
  gameId: string | null;
}

function LiveBoard({ gameId }: LiveBoardProps) {
  const [ready, setReady] = useState(false);
  const [setup, setSetup] = useState(false);
  const [board, setBoard] = useState<Board>(createBoard());
  const [turn, setTurn] = useState<Piece>(Piece.Black);
  const [color, setColor] = useState<Piece>(Piece.Black);
  const [token, setToken] = useState<string>();
  const [preview, setPreview] = useState<Array<[number, number]>>();

  const { sendJsonMessage } = useWebSocket("ws://0.0.0.0:3000/live", {
    onMessage: (msg) => {
      const data: Event = JSON.parse(msg.data);
      console.log(data);
      const handlers = {
        1: handleAckEvent,
        2: handleReady,
        4: handleGameUpdate,
        5: handlePreviewEvent,
        6: handleErrorEvent,
      } as const;
      handlers[data.op]({
        //@ts-expect-error
        ev: data as any,
        board,
        token,
        gameId,
        setReady,
        setTurn,
        setBoard,
        setPreview,
        setColor,
      });
    },
  });

  useEffect(() => {
    setToken(cookie.parse(document.cookie).sid);
    if (token && !setup) {
      sendJsonMessage({
        op: 6,
        t: token,
        d: {
          type: "Identify",
        },
      });
      sendJsonMessage({
        op: 3,
        t: token,
        d: {
          type: "Join",
          id: gameId,
        },
      });
      (async () => {
        const { host } = await simpleGet(`/game/${gameId}`);
        const { id } = await simpleGet("/@me");
        setColor(host === id ? Piece.Black : Piece.White);
      })();
      setSetup(true);
    }
  }, [ready, gameId, token, setup, sendJsonMessage]);

  const stringifyPiece = (piece: Piece) =>
    piece === Piece.Black ? "Black" : "White";

  return (
    <main className="flex flex-row flex-wrap-reverse">
      <section className="flex flex-col max-h-screen items-center p-5">
        {board.map((arr, row) => (
          <div className="flex flex-row" key={row}>
            {arr.map((piece, col) => (
              <Square
                key={`${row}-${col}`}
                piece={piece}
                turn={turn}
                color={color}
                preview={preview?.some(([x, y]) => x === col && y === row)}
                row={row}
                col={col}
                onMouseEnter={() => {
                  sendJsonMessage({
                    op: 7,
                    t: token,
                    d: {
                      type: "Place",
                      id: gameId,
                      x: col,
                      y: row,
                      piece: stringifyPiece(color),
                    },
                  });
                }}
                onMouseLeave={() => {
                  setPreview(undefined);
                }}
                onClick={() => {
                  sendJsonMessage({
                    op: 2,
                    t: token,
                    d: {
                      type: "Place",
                      id: gameId,
                      x: col,
                      y: row,
                      piece: stringifyPiece(color),
                    },
                  });
                }}
              />
            ))}
          </div>
        ))}
      </section>
      <section>
        <p className="text-text pt-5">Turn: {turn === 0 ? "Black" : "White"}</p>
        <p className="text-text pt-5">Game ID: {gameId}</p>
      </section>
    </main>
  );
}

export default function Play() {
  const [gameId, setGameId] = useState<string | null>(null);

  useEffect(() => {
    const id = new URLSearchParams(window.location.search).get("gameId") || "";
    setGameId(id);
  }, [gameId]);

  return gameId?.match(UUID_REGEX) ? <LiveBoard gameId={gameId} /> : <></>;
}
