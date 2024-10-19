export type Optional<T> = T | undefined;

export enum Piece {
  Black,
  White,
}

export interface Game {
  id: string;
  host: string;
  opponent: string;
  ended: boolean;
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

export type Board = Array<Array<Piece | null>>;

export interface AckEvent {
  op: 1;
}

export interface ReadyEvent {
  op: 2;
  d: {
    token: string;
  };
}

export interface GameAbortEvent {
  op: 3;
  d: {
    token: string;
    id: string;
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

export interface GameEndEvent {
  op: 7;
  d: {
    winner: string;
    points: number;
    total: number;
  };
}

export type Event =
  | AckEvent
  | ReadyEvent
  | GameAbortEvent
  | GameUpdateEvent
  | ErrorEvent
  | PreviewEvent
  | GameEndEvent;

export interface Context<T> {
  ws: WebSocket;
  ev: T;
  board: Board;
  token?: string;
  gameId: string | null;
  aborted?: boolean;
  setReady: (ready: boolean) => void;
  setTurn: (turn: Piece) => void;
  setBoard: (board: Board) => void;
  setColor: (color: Piece) => void;
  setAborted: (aborted: boolean) => void;
  setPreview: (preview: Array<[number, number]> | undefined) => void;
}
