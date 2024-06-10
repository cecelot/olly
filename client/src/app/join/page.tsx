"use client";

import { Button, Input } from "@headlessui/react";
import { useState } from "react";

export default function Join() {
  const [gameId, setGameId] = useState("");

  const onClick = (e: React.MouseEvent<HTMLButtonElement, MouseEvent>) => {
    e.preventDefault();
    window.location.href = `/play?gameId=${gameId}`;
  };

  return (
    <main className="text-center mx-auto p-4">
      <form className="flex flex-col mx-auto space-y-3 max-w-80">
        <Input
          placeholder="Game ID"
          className="bg-crust text-subtext0 rounded-lg p-3"
          onChange={(e) => setGameId(e.currentTarget.value)}
        ></Input>
        <Button
          onClick={onClick}
          className="text-text border-2 border-green hover:bg-mantle transition-all rounded-lg p-3"
        >
          Join
        </Button>
      </form>
    </main>
  );
}
