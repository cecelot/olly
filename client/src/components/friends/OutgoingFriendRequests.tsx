import call from "@/lib/call";
import simpleGet from "@/lib/simpleGet";
import { OutgoingFriendRequest } from "@/types";
import { Button } from "@headlessui/react";
import { useEffect, useState } from "react";

export default function OutgoingFriendRequests() {
  const [outgoingFriendRequests, setOutgoingFriendRequests] =
    useState<OutgoingFriendRequest[]>();

  const cancelFriendRequest = (recipient: string) => {
    return async (e: React.MouseEvent<HTMLButtonElement, MouseEvent>) => {
      e.preventDefault();
      (async () => {
        const res = await call(`/@me/friends/outgoing/${recipient}`, "DELETE");
        if (res.status === 200) {
          alert(`Cancelled friend request to ${recipient}.`);
        } else {
          alert(`Failed to cancel friend request.`);
        }
      })();
    };
  };

  useEffect(() => {
    (async () =>
      setOutgoingFriendRequests(await simpleGet("/@me/friends/outgoing")))();
  });

  return (outgoingFriendRequests?.length || 0) > 0 ? (
    outgoingFriendRequests?.map((request) => (
      <li
        className="flex flex-row space-x-1 justify-center"
        key={`${request.recipient}-outgoing`}
      >
        <p className="text-text">{request.recipient}</p>
        <Button
          className="text-mauve hover:text-pink transition-all"
          onClick={cancelFriendRequest(request.recipient)}
        >
          {"["}Cancel{"]"}
        </Button>
      </li>
    ))
  ) : (
    <></>
  );
}
