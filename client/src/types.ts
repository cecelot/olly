import { Signal } from "solid-js";

export enum Piece {
  Black,
  White,
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

export interface GameCreateEvent {
  op: 3;
  d: {
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

export interface ErrorEvent {
  op: 5;
  d: {
    message: string;
    code: number;
  };
}

export type Event =
  | AckEvent
  | ReadyEvent
  | GameCreateEvent
  | GameUpdateEvent
  | ErrorEvent;
