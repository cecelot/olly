"use client";

import ErrorableFormInput from "@/components/input/Field";
import { Optional } from "@/types";
import { Button } from "@headlessui/react";
import { useState } from "react";

interface RegisterRouteReply {
  message: string;
}

export default function Register() {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [usernameError, setUsernameError] = useState<Optional<string>>();
  const [passwordError, setPasswordError] = useState<Optional<string>>();

  const onClick = (_: unknown) => {
    return (async () => {
      const res = await fetch("http://localhost:3000/register", {
        credentials: "include",
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          username: username,
          password: password,
        }),
      });
      const to = decodeURIComponent(
        new URLSearchParams(window.location.search).get("to") || "/"
      );
      if (res.status === 201) {
        window.location.href = to;
      } else {
        const { message }: RegisterRouteReply = await res.json();
        if (message.includes("Username")) {
          setUsernameError(message);
        } else setUsernameError(undefined);
        if (message.includes("Password")) {
          setPasswordError(message);
        } else setPasswordError(undefined);
      }
    })();
  };

  return (
    <main className="text-center mx-auto m-4">
      <h1 className="text-3xl font-semibold mb-5">Register</h1>
      <form className="flex flex-col mx-auto space-y-3 max-w-60">
        <ErrorableFormInput
          placeholder="Username"
          type="text"
          setText={setUsername}
          errorText={usernameError}
        />
        <ErrorableFormInput
          placeholder="Password"
          type="password"
          setText={setPassword}
          errorText={passwordError}
        />
        <Button
          onClick={onClick}
          className="text-text border-2 border-green hover:bg-mantle transition-all rounded-lg p-3"
        >
          Create Account
        </Button>
      </form>
    </main>
  );
}
