"use client";

import { Button, Input } from "@headlessui/react";
import { useEffect, useState } from "react";
import { BASE_API_URL } from "@/lib";
import cn from "classnames";

export default function AccountEditPassword() {
  const [currentPassword, setCurrentPassword] = useState("");
  const [newPassword, setNewPassword] = useState("");
  const [confirmedPassword, setConfirmedPassword] = useState("");
  const [statusText, setStatusText] = useState("");
  const [errored, setErrored] = useState(false);

  useEffect(() => {
    setErrored(false);
    setStatusText("");
  }, [currentPassword, newPassword, confirmedPassword]);

  const changePassword = (
    e: React.MouseEvent<HTMLButtonElement, MouseEvent>
  ) => {
    e.preventDefault();
    (async () => {
      console.log(currentPassword, newPassword, confirmedPassword);
      const res = await fetch(`${BASE_API_URL}/@me`, {
        method: "PATCH",
        body: JSON.stringify({
          password: {
            current: currentPassword,
            new: newPassword,
            confirmed: confirmedPassword,
          },
        }),
        credentials: "include",
        mode: "cors",
        headers: {
          "Content-Type": "application/json",
        },
      });
      if (res.status === 200) {
        setStatusText(`Changed your password successfully.`);
      } else {
        const { message } = await res.json();
        setErrored(true);
        switch (res.status) {
          case 403:
            setStatusText(message);
            break;
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
          type="password"
          placeholder="Current Password"
          className="bg-crust text-subtext0 rounded-lg p-3"
          onChange={(e) => setCurrentPassword(e.currentTarget.value)}
        ></Input>
        <hr />
        <Input
          type="password"
          placeholder="New Password"
          className="bg-crust text-subtext0 rounded-lg p-3"
          onChange={(e) => setNewPassword(e.currentTarget.value)}
        ></Input>
        <Input
          type="password"
          placeholder="Confirm Password"
          className="bg-crust text-subtext0 rounded-lg p-3"
          onChange={(e) => setConfirmedPassword(e.currentTarget.value)}
        ></Input>
        <Button
          onClick={changePassword}
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
