import call from "@/lib/call";
import simpleGet from "@/lib/simpleGet";
import { IncomingFriendRequest } from "@/types";
import { useEffect, useState } from "react";

export default function IncomingFriendRequests() {
  const [incomingFriendRequests, setIncomingFriendRequests] =
    useState<IncomingFriendRequest[]>();

  const onClick = (sender: string, accept: boolean) => {
    return async (e: React.MouseEvent<HTMLButtonElement, MouseEvent>) => {
      e.preventDefault();
      const end = accept ? "accept" : "deny";
      (async () => {
        const res = await call(`/@me/friends/${sender}/${end}`, "POST");
        if (res.status === 200) {
          alert(`Friend request ${end}ed!`);
        } else {
          alert(`Failed to ${end} friend request.`);
        }
      })();
    };
  };

  useEffect(() => {
    (async () =>
      setIncomingFriendRequests(await simpleGet("/@me/friends/incoming")))();
  });

  return (incomingFriendRequests?.length || 0) > 0 ? (
    incomingFriendRequests?.map((request) => (
      <li
        className="flex flex-row space-x-3 justify-center"
        key={`${request.sender}-incoming`}
      >
        <p className="text-text">{request.sender}</p>
        <button
          onClick={onClick(request.sender, true)}
          className="text-mauve hover:text-pink transition-all"
        >
          {"["}Accept{"]"}
        </button>
        <button
          onClick={onClick(request.sender, false)}
          className="text-mauve hover:text-pink transition-all"
        >
          {"["}Deny{"]"}
        </button>
      </li>
    ))
  ) : (
    <></>
  );
}
