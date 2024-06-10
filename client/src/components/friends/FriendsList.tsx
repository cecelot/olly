"use client";

import { createGame } from "@/lib/createGame";
import simpleGet from "@/lib/simpleGet";
import { Friend } from "@/types";
import { Button } from "@headlessui/react";
import { useState } from "react";

export default function FriendsList() {
  const [friends, setFriends] = useState<Friend[]>();

  useState(() => {
    (async () => setFriends(await simpleGet("/@me/friends")))();
  });

  return (
    <section className="py-10 text-left">
      <h2 className="font-bold text-text text-center">Your Friends</h2>
      <ul className="max-h-80 py-2 overflow-y-scroll">
        {(friends?.length || 0) > 0 &&
          friends?.map((friend) => (
            <li
              className="flex flex-row space-x-1 justify-center"
              key={friend.username}
            >
              <p className="text-text">{friend.username}</p>
              <Button
                onClick={(e) => {
                  e.preventDefault();
                  createGame(friend.username);
                }}
                className="text-mauve hover:text-pink transition-all"
              >
                {"["}Play{"]"}
              </Button>
              <Button className="text-mauve hover:text-pink transition-all">
                {"["}Remove{"]"}
              </Button>
            </li>
          ))}
      </ul>
    </section>
  );
}
