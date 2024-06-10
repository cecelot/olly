"use client";

import FriendsList from "@/components/friends/FriendsList";
import MergedFriendRequests from "@/components/friends/MergedFriendRequests";
import call from "@/lib/call";
import useUser from "@/lib/useUser";
import { Button, Input } from "@headlessui/react";
import Link from "next/link";
import { useState } from "react";

export default function Account() {
  const { user, isLoading } = useUser();
  const [friendUsername, setFriendUsername] = useState("");

  const sendFriendRequest = (
    e: React.MouseEvent<HTMLButtonElement, MouseEvent>
  ) => {
    e.preventDefault();
    (async () => {
      const res = await call(`/users/${friendUsername}/friend`, "POST");
      if (res.status === 201) {
        alert(`Sent friend request to ${friendUsername}!`);
      } else {
        alert(
          `That user doesn't exist! Make sure their username is spelled correctly.`
        );
      }
    })();
  };

  if (isLoading) {
    return <></>;
  }

  return (
    <main className="text-center mx-auto p-5 max-w-3xl">
      {user?.username ? (
        <>
          <div className="pb-10">
            <h1 className="text-2xl text-text font-extrabold">
              Hi, {user.username}!
            </h1>
            <h6 className="text-sm text-subtext1">{user.id}</h6>
          </div>
          <form className="flex flex-col mx-auto space-y-3 max-w-80">
            <Input
              placeholder="Username"
              className="bg-crust text-subtext0 rounded-lg p-3"
              onChange={(e) => setFriendUsername(e.currentTarget.value)}
            ></Input>
            <Button
              onClick={sendFriendRequest}
              className="text-text border-2 border-green hover:bg-mantle transition-all rounded-lg p-3"
            >
              Send Friend Request
            </Button>
          </form>
          <section className="grid grid-cols-2">
            <FriendsList />
            <MergedFriendRequests />
          </section>
        </>
      ) : (
        <>
          <h1 className="text-2xl text-text font-extrabold">
            You are not logged in.
          </h1>
          <p className="my-4">
            <Link
              href="/"
              className="text-mauve hover:text-pink transition-all"
            >
              {"["}Home{"]"}
            </Link>
          </p>
        </>
      )}
    </main>
  );
}
