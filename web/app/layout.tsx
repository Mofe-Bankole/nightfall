import type { Metadata } from "next";
import { Geist, Geist_Mono , Audiowide } from "next/font/google";
import { Orbitron } from "next/font/google";
import "./globals.css";
import { WalletProvider } from "@/context/WalletContext";

const audioWide = Audiowide({
  weight : "400"
});

export const metadata: Metadata = {
  title: "Nigthfall",
  description: "The private payment system for web3 lords",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body
        className={`${audioWide.className} antialiased bg-black`}
      >
        {/* <WalletProvider children={children} /> */}
        {children}
      </body>
    </html>
  );
}
