"use client";
import { Audiowide } from "next/font/google";
import Header from "@/components/Header";
import { Button } from "@/components/ui/button";
import { useRouter } from "next/navigation";
import { Lock } from "lucide-react";

const audioWide = Audiowide({
  weight: "400",
});

export default function Home() {
  const router = useRouter();
  const handleCreateWallet = () => {
    router.push("/wallet/create");
  };
  const handleUseExistingWallet = () => {
    router.push("/wallet/find");
  };
  return (
    <div
      className={`flex flex-col min-h-screen bg-black text-white pt-3 ${audioWide.className}`}
    >
      <Header />
      <div className="body w-[85%] mx-auto mt-2.5 items-center my-auto justify-center pt-20">
        <div className="max-w-6xl mx-auto">
          <div className="flex items-center justify-between gap-16">
            {/* Left side - Text content */}
            <div className="flex-1 space-y-10">
              <h1 className="text-5xl">Night.fall</h1>
              <div className="space-y-4">
                <p className="mb-2">
                  Night.fall is a secure wallet built on zcash
                </p>
                <p>
                  Night.fall operates with an
                  <span className="underline cursor-pointer mx-1 ">
                    off-the-chain
                  </span>
                  protocol, what this means is that your data is never stored on
                  chain, rather on DISK as an encrypted database
                </p>
                <p>
                  Safe and Secure, night.fall offers transparent and shielded
                  addresses
                </p>
                <p>Your data never leaves your device</p>
              </div>
              <div className="flex items-center space-x-4">
                <Button
                  className="cursor-pointer border rounded-sm bg-black text-white px-8 py-8 text-zinc-200 text-2xl"
                  onClick={handleCreateWallet}
                >
                  Create New Wallet
                </Button>
                <Button
                  className="cursor-pointer border rounded-sm bg-black text-white px-8 py-8 text-zinc-200 text-2xl"
                  onClick={handleUseExistingWallet}
                >
                  Use Existing Wallet
                </Button>
              </div>
            </div>

            {/* Right side - Padlock icon */}
            <div className="flex-1 flex items-center justify-center">
              <Lock className="w-96 h-96 text-zinc-700" strokeWidth={1} />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
