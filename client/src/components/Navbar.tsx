"use client";

import Link from "next/link";
import useUser from "@/lib/useUser";

export default function Navbar() {
  const { user, isLoading, authenticated } = useUser();

  return (
    <nav className="flex flex-row justify-center p-5 bg-base text-mauve">
      <div className="space-x-3">
        <Link href="/" className="hover:text-pink transition-all">
          {"["}Home{"]"}
        </Link>
        <Link href="/about" className="hover:text-pink transition-all">
          {"["}About{"]"}
        </Link>
        <Link
          href="/stats"
          className="text-mauve hover:text-pink transition-all"
        >
          {"["}Stats{"]"}
        </Link>
        <Link
          href={!isLoading && authenticated ? "/account" : "/login"}
          className="text-mauve hover:text-pink transition-all"
        >
          {"["}
          {!isLoading && authenticated ? user?.username : "Login"}
          {"]"}
        </Link>
        <Link
          href={!isLoading && authenticated ? "/logout" : "/register"}
          className="text-mauve hover:text-pink transition-all"
        >
          {"["}
          {!isLoading && authenticated ? "Logout" : "Register"}
          {"]"}
        </Link>
      </div>
    </nav>
  );
}
