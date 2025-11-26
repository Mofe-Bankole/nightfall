'use client';

import axios from "axios"
import { Button } from "@/components/ui/button";
import { Sun, Send, ArrowDownToLine, CreditCard, ArrowLeftRight, X, Copy, Check, Loader2 } from "lucide-react"
import React, { useState  , useEffect} from "react";

export default function DashBoard(){
    const [zecPrice , setZecPrice] = useState<number |null>(null)
    useEffect(() => {
        axios
            .get("https://api.diadata.org/v1/assetQuotation/Zcash/0x0000000000000000000000000000000000000000")
            .then((res) => {
                // The price from diadata usually comes as res.data.Price
                setZecPrice(res?.data.Price ?? null);
            })
            .catch(() => setZecPrice(null));
    }, []);

    const [username , setUserName] = useState<string | null >(null);
    const actions = ["Send" , "Receive" , "Buy" , "Send"]
    
    return(
        <React.Fragment>
        <div className="py-3.5 cursor-pointer px-6 border border-transparent hover:border-white/20 bg-[#020202] text-white w-[95%] mx-auto rounded-sm mt-3 flex justify-between transition-colors duration-700">
            <h3>Night.fall</h3>
            <p>setUserName</p>
            <div className="items-center flex">
            <h3 className="text-xl mr-3.5"> ZEC : ${Math.ceil(zecPrice)}</h3>
            <Sun className="cursor-pointer text-2xl sun_mode"/>
            </div>
        </div>
        <div className="text-white w-[87%] mx-auto mt-10 px-5">
        <div className="text-center xl:text-right">
            <h2 className="text-5xl">$ 133</h2>
        </div>
        <div className="mt-5 pt-3 border-t-2">
            <div className="actions space-x-2 pt-1.5">
                {/* {for i in actions} */}
                <Button className="px-11 py-7 border border-white/20 bg-black text-base font-semibold text-white transition hover:bg-white hover:text-black disabled:cursor-not-allowed disabled:border-white/10 disabled:text-white/50 cursor-pointer">Send <Send/></Button>
                <Button className="px-10 py-7 border border-white/20 bg-black text-base font-semibold text-white transition hover:bg-white hover:text-black disabled:cursor-not-allowed disabled:border-white/10 disabled:text-white/50 cursor-pointer">Receive <ArrowDownToLine/></Button>
                <Button className="px-10 py-7 border border-white/20 bg-black text-base font-semibold text-white transition hover:bg-white hover:text-black disabled:cursor-not-allowed disabled:border-white/10 disabled:text-white/50 cursor-pointer">Pay<CreditCard/></Button>
                <Button className="px-10 py-7 border border-white/20 bg-black text-base font-semibold text-white transition hover:bg-white hover:text-black disabled:cursor-not-allowed disabled:border-white/10 disabled:text-white/50 cursor-pointer">Swap<ArrowLeftRight/></Button>

            </div>
            <div className="w-auto tpx-11 py-7 border border-white/20 bg-black text-base font-semibold text-white transition hover:bg-white hover:text-black disabled:cursor-not-allowed disabled:border-white/10 disabled:text-white/50 cursor-pointer">
                <h3 className="text-xl mr-3.5"> ZEC : ${Math.ceil(zecPrice)}</h3>
            </div>
        </div>
        </div>
        </React.Fragment>
    )

}