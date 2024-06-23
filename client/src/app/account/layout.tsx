"use client";

import useUser from "@/lib/hooks/useUser";
import Link from "next/link";

export default function AccountLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const { user, isLoading } = useUser();

  if (isLoading) return <></>;

  return (
    <main className="text-text text-center mx-auto p-5 max-w-3xl">
      {user?.username ? (
        <>
          <div className="pb-10">
            <h1 className="text-2xl text-text font-extrabold">
              Hi, {user.username}!
            </h1>
            <h6 className="text-sm text-subtext1">{user.id}</h6>
            <div className="flex flex-row justify-center space-x-3">
              <Link
                href="/account/edit/username"
                className="text-mauve hover:text-pink transition-all"
              >
                {"["}Change Username{"]"}
              </Link>
              <Link
                href="/account/edit/password"
                className="text-mauve hover:text-pink transition-all"
              >
                {"["}Change Password{"]"}
              </Link>
            </div>
          </div>
          {children}
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
