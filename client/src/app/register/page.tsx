"use client";

import { Button, Input } from "@headlessui/react";
import { useState } from "react";

export default function Login() {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

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
      if (res.status === 200) {
        window.location.href = to;
      } else {
        alert(JSON.stringify(await res.json()));
      }
    })();
  };

  return (
    <main className="text-center mx-auto m-4">
      <h1 className="text-3xl font-semibold mb-5">Register</h1>
      <form className="flex flex-col mx-auto space-y-3 max-w-60">
        <Input
          placeholder="Username"
          className="bg-crust text-subtext0 rounded-lg p-3"
          onChange={(e) => setUsername(e.currentTarget.value)}
        ></Input>
        <Input
          placeholder="Password"
          className="bg-crust text-subtext0 rounded-lg p-3"
          type="password"
          onChange={(e) => setPassword(e.currentTarget.value)}
        ></Input>
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
