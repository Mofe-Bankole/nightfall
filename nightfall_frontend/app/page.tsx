import Header from "@/components/Header";
import Image from "next/image";
import Link from "next/link";
// import { useRouter } from "next/navigation";

export default function Home() {
  // const router = useRouter();

  return (
    <div className="flex flex-col min-h-screen bg-zinc-50 font-sans text-black pt-3">
      <Header/>
      <input type="text" />

    </div>
  );
}
