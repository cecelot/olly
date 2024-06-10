import Link from "next/link";

export default function NotFound() {
  return (
    <main className="text-center mx-auto p-4">
      <h1 className="text-6xl text-text uppercase my-4">{":("}</h1>
      <h2 className="text-lg text-subtext0">
        The page you are looking for does not exist
      </h2>
      <p className="my-4">
        <Link href="/" className="text-mauve hover:text-pink transition-all">
          {"["}Home{"]"}
        </Link>
      </p>
    </main>
  );
}
