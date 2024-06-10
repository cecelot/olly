"use client";

import { useState } from "react";
import { createGame } from "@/lib/createGame";
import { Button, Input } from "@headlessui/react";

export default function New() {
  const [opponent, setOpponent] = useState("");

  const onClick = (e: React.MouseEvent<HTMLButtonElement, MouseEvent>) => {
    e.preventDefault();
    createGame(opponent);
  };

  return (
    <main className="text-center mx-auto p-4">
      <form className="flex flex-col mx-auto space-y-3 max-w-80">
        <Input
          placeholder="Opponent"
          className="bg-crust text-subtext0 rounded-lg p-3"
          onChange={(e) => setOpponent(e.currentTarget.value)}
        ></Input>
        <Button
          onClick={onClick}
          className="text-text border-2 border-green hover:bg-mantle transition-all rounded-lg p-3"
        >
          Play
        </Button>
      </form>
    </main>
  );
}
