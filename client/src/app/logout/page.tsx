"use client";

import { BASE_API_URL } from "@/lib";
import useUser from "@/lib/hooks/useUser";
import Link from "next/link";
import { useEffect, useState } from "react";

export default function Logout() {
  const { mutate } = useUser();
  const [loggedOut, setLoggedOut] = useState(false);
  const [message, setMessage] = useState("You are being logged out...");

  useEffect(() => {
    fetch(`${BASE_API_URL}/logout`, {
      method: "POST",
      credentials: "include",
    }).then((res) => {
      if (res.status === 200) {
        mutate();
        setLoggedOut(true);
        setMessage("You have been logged out.");
      } else {
        setMessage("An error occurred while trying to log you out.");
      }
    });
  });

  return (
    <main className="text-center mx-auto p-5 max-w-md leading-8 space-y-5">
      <p>{message}</p>
      {loggedOut && (
        <p className="my-4">
          <Link href="/" className="text-mauve hover:text-pink transition-all">
            {"["}Home{"]"}
          </Link>
        </p>
      )}
    </main>
  );
}
