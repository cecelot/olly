import { useState, useEffect } from "react";

interface CircleProps {
  color: "white" | "black" | "faded-white" | "faded-black";
}

const fills = {
  white: "#e4e4e7",
  black: "#09090b",
  "faded-white": "#cacacc",
  "faded-black": "#696980",
};

export default function Circle(props: CircleProps) {
  const [fill, setFill] = useState(fills[props.color]);

  useEffect(() => {
    setFill(fills[props.color]);
  }, [props.color]);

  return (
    <svg
      viewBox="0 0 100 100"
      xmlns="http://www.w3.org/2000/svg"
      className="inline-block mx-auto"
    >
      <circle cx="50" cy="50" r="50" fill={fill} stroke={fills.black} />
    </svg>
  );
}
