import type { Metadata } from "next";
import { Inclusive_Sans } from "next/font/google";
import "./globals.css";
import Navbar from "@/components/Navbar";

const inclusiveSans = Inclusive_Sans({
  weight: "400",
  subsets: ["latin", "latin-ext"],
});

export const metadata: Metadata = {
  title: "Othello",
  description: "Play Othello online with friends or against the computer!",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className="latte bg-base">
      <body className={inclusiveSans.className}>
        <Navbar />
        {children}
      </body>
    </html>
  );
}
