import { Accessor, Signal } from "solid-js";

export enum Piece {
  Black,
  White,
}

export interface Game {
  id: string;
  opponent: string;
}

export interface Member {
  id: string;
  username: string;
}

export interface Friend {
  username: string;
}

export interface IncomingFriendRequest {
  sender: string;
}

export interface OutgoingFriendRequest {
  recipient: string;
}

export type Board = Array<Array<Signal<Piece | undefined>>>;

export interface AckEvent {
  op: 1;
}

export interface ReadyEvent {
  op: 2;
  d: {
    token: string;
  };
}

export interface GameUpdateEvent {
  op: 4;
  d: {
    game: {
      board: Array<string | null>;
      turn: string;
    };
  };
}

export interface PreviewEvent {
  op: 5;
  d: {
    changed: Array<[number, number]>;
  };
}

export interface ErrorEvent {
  op: 6;
  d: {
    message: string;
    code: number;
  };
}

export type Event =
  | AckEvent
  | ReadyEvent
  | GameUpdateEvent
  | ErrorEvent
  | PreviewEvent;

export interface Context<T> {
  ws: WebSocket;
  ev: T;
  board: Board;
  token: Accessor<string | undefined>;
  gameId: Accessor<string | null>;
  setTurn: (turn: Piece) => void;
  setColor: (color: Piece) => void;
  setPreview: (preview: Array<[number, number]> | undefined) => void;
}
