import Link from "next/link";

export default function AccountEditLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <>
      {children}
      <p className="my-4">
        <Link
          href="/account"
          className="text-mauve hover:text-pink transition-all"
        >
          {"["}Back{"]"}
        </Link>
      </p>
    </>
  );
}
