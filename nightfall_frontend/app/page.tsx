"use client";

import Header from "@/components/Header";
import { Button } from "@/components/ui/button";
import Image from "next/image";
import Link from "next/link";
// import { useRouter } from "next/router";
import { useRouter } from "next/navigation";
// 
export default function Home() {
  const router = useRouter();

  return (
    <div className="flex flex-col min-h-screen bg-zinc-50 font-sans text-black pt-3">
      <Header/>
      <div className="body w-[85%] mx-auto mt-2.5 items-center my-auto justify-center pt-52">
        <div className="hero_section max-w-md mx-auto text-center space-y-14">
          <h1 className="text-5xl">Night.fall DEMO</h1>
          <p>Hightfall demo frontend showcasing the app</p>
          <div className="flex items-center justify-center space-x-4">
            <Button className="cursor-pointer border rounded-sm
             bg-black text-white px-13 py-8 text-zinc-200 text-2xl">Sign Up</Button>
             <Button className="cursor-pointer border rounded-sm
             bg-black text-white px-13 py-8 text-zinc-200 text-2xl">Sign In</Button>
          </div> 
        </div>
      </div>

    </div>
  );
}
