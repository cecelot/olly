"use client";

import { Button, Input } from "@headlessui/react";
import { useEffect, useState } from "react";
import cn from "classnames";
import { BASE_API_URL } from "@/lib";

export default function AccountEditUsername() {
  const [username, setUsername] = useState("");
  const [statusText, setStatusText] = useState("");
  const [errored, setErrored] = useState(false);

  useEffect(() => {
    setErrored(false);
    setStatusText("");
  }, [username]);

  const changeUsername = (
    e: React.MouseEvent<HTMLButtonElement, MouseEvent>
  ) => {
    e.preventDefault();
    (async () => {
      const res = await fetch(`${BASE_API_URL}/@me`, {
        method: "PATCH",
        body: JSON.stringify({ username }),
        credentials: "include",
        mode: "cors",
        headers: {
          "Content-Type": "application/json",
        },
      });
      if (res.status === 200) {
        setStatusText(`Changed your username to ${username}!`);
      } else {
        console.log(res.status);
        const { message } = await res.json();
        setErrored(true);
        switch (res.status) {
          case 400:
            setStatusText(message);
            break;
          case 409:
            setStatusText(message);
            break;
          default:
            setStatusText(
              `An unexpected error occurred. Please try again later.`
            );
        }
      }
    })();
  };

  return (
    <>
      <form className="flex flex-col mx-auto space-y-3 max-w-80">
        <Input
          placeholder="Username"
          className="bg-crust text-subtext0 rounded-lg p-3"
          onChange={(e) => setUsername(e.currentTarget.value)}
        ></Input>
        <Button
          onClick={changeUsername}
          className="text-text border-2 border-green hover:bg-mantle transition-all rounded-lg p-3"
        >
          Update
        </Button>
      </form>
      <p
        className={cn("pt-2", {
          "text-red": errored,
          "text-green": !errored,
        })}
      >
        {statusText}
      </p>
    </>
  );
}
