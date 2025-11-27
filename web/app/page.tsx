"use client";

import { Audiowide } from "next/font/google";
import Header from "@/components/Header";
import { Button } from "@/components/ui/button";
import { useRouter } from "next/navigation";


const audioWide = Audiowide({
  weight : "400"
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
    <div className={`flex flex-col min-h-screen bg-black text-white pt-3 ${audioWide.className}`}>
      <Header/>
      <div className="body w-[85%] mx-auto mt-2.5 items-center my-auto justify-center pt-50">
        <div className="hero_section max-w-md mx-auto text-center space-y-10">
          <h1 className="text-5xl">Night.fall</h1>
          <p>Nightfall is a secure wallet built on zcash</p>
          <p>And the best part its easy to use even if youve never used a Z-Cash Supported Wallet </p>
          <div className="flex items-center justify-center space-x-4">
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
      </div>
    </div>
  );
}
