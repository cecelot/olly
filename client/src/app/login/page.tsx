"use client";

import ErrorableFormInput from "@/components/input/Field";
import { BASE_API_URL } from "@/lib";
import { Optional } from "@/types";
import { Button } from "@headlessui/react";
import { useState } from "react";

export default function Login() {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [usernameError, setUsernameError] = useState<Optional<string>>();
  const [passwordError, setPasswordError] = useState<Optional<string>>();

  const onClick = (_: unknown) => {
    return (async () => {
      const res = await fetch(`${BASE_API_URL}/login`, {
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
      if (res.status === 200) {
        window.location.href = to;
      } else {
        switch (res.status) {
          case 404: {
            setUsernameError("No user exists with that username.");
            setPasswordError(undefined);
            break;
          }
          case 403: {
            setUsernameError(undefined);
            setPasswordError("Incorrect password! Try again.");
            break;
          }
          default:
            setUsernameError("An unexpected error occurred.");
            setPasswordError(undefined);
        }
        // setErrorCode(res.status);
      }
    })();
  };

  return (
    <main className="text-center mx-auto m-4">
      <h1 className="text-3xl font-semibold mb-5">Login</h1>
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
          Login
        </Button>
      </form>
    </main>
  );
}
