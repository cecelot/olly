import cs from "classnames";
import Circle from "@/components/Circle";
import { Piece } from "../types";

interface SquareProps {
  onClick: () => void;
  onMouseEnter: () => void;
  onMouseLeave: () => void;
  piece: Piece | null;
  preview?: boolean;
  turn: Piece;
  color: Piece;
  row: number;
  col: number;
}

export default function Square(props: SquareProps) {
  const color = {
    [Piece.White]: "white",
    [Piece.Black]: "black",
  } as const;

  const previewColor = {
    [Piece.White]: "faded-white",
    [Piece.Black]: "faded-black",
  } as const;

  return (
    <div
      onClick={props.onClick}
      onMouseEnter={props.onMouseEnter}
      onMouseLeave={props.onMouseLeave}
      className={cs(
        "bg-green-400 w-20 h-20 text-center border-slate-900 border-r-2 border-t-2",
        {
          "border-l-2": props.col === 0,
          "border-b-2": props.row === 7,
          "cursor-pointer": props.piece === null && props.color === props.turn,
          "cursor-not-allowed":
            props.piece !== null || props.color !== props.turn,
        }
      )}
    >
      {props.piece !== null && (
        <Circle
          color={
            props.preview ? previewColor[props.turn] : color[props.piece!!]
          }
        />
      )}
    </div>
  );
}
