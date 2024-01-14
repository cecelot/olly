import { createEffect, createSignal } from "solid-js";

interface CircleProps {
  color: "white" | "black";
}

export default function Circle(props: CircleProps) {
  const fills = {
    white: "#e4e4e7",
    black: "#09090b",
  };
  const [fill, setFill] = createSignal(fills[props.color]);

  createEffect(() => {
    setFill(fills[props.color]);
  });

  return (
    <svg
      viewBox="0 0 100 100"
      xmlns="http://www.w3.org/2000/svg"
      class="inline-block mx-auto"
    >
      <circle cx="50" cy="50" r="50" fill={fill()} stroke={fills.black} />
    </svg>
  );
}
