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
import StatusText from "@/components/StatusText";
import useWebSocket from "react-use-websocket";
import cookie from "cookie";
import cn from "classnames";
import simpleGet from "@/lib/simpleGet";
import call from "@/lib/call";

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
        // This actually makes it look smoother, IMO, because the flash isn't as abrupt and disorienting.
        setTimeout(() => {
          setSetup(true);
        }, 500);
      })();
    }
  }, [ready, gameId, token, color, setup, sendJsonMessage]);

  if (!setup) return <StatusText text="Loading..." />;

  const stringifyPiece = (piece: Piece) =>
    piece === Piece.Black ? "Black" : "White";

  return (
    <main className="flex flex-col">
      <p className="mx-auto">
        You are playing with the {stringifyPiece(color)} pieces
      </p>
      <div className="flex flex-row">
        <div className="mx-auto">
          <div
            className={cn(`border-2 w-12 h-[642px] ml-4 mt-5`, {
              "bg-mantle": color === Piece.White,
              "bg-[#09090b]": color === Piece.Black,
              "border-mauve": turn === color,
              "border-crust": turn !== color,
            })}
          />
        </div>
        <section className="flex flex-col  items-center p-5">
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
        <div className="mx-auto">
          <div
            className={cn(`border-2 w-12 h-[642px] ml-4 mt-5`, {
              "bg-mantle": color === Piece.Black,
              "bg-[#09090b]": color === Piece.White,
              "border-mauve": turn !== color,
              "border-crust": turn === color,
            })}
          />
        </div>
      </div>
    </main>
  );
}

export default function Play() {
  const [gameId, setGameId] = useState<string | null>(null);
  const [verifying, setVerifying] = useState(true);
  const [isValidGame, setIsValidGame] = useState(true);

  useEffect(() => {
    const id = new URLSearchParams(window.location.search).get("gameId") || "";
    setGameId(id);
    (async () => {
      const game = await call(`/game/${id}`, "GET");
      setVerifying(false);
      const { message } = await game.json();
      if (game.status !== 200 || message.pending) {
        setIsValidGame(false);
      }
    })();
  }, [gameId]);

  if (verifying) {
    return <StatusText text="Loading..." />;
  }

  return gameId?.match(UUID_REGEX) && isValidGame ? (
    <LiveBoard gameId={gameId} />
  ) : (
    <StatusText text="Invalid Game ID provided" />
  );
}
