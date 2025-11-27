'use client';

import axios from "axios"
import { Button } from "@/components/ui/button";
import { Sun, Send, ArrowDownToLine, CreditCard, ArrowLeftRight, X, Copy, Check, Loader2 } from "lucide-react"
import React, { useState  , useEffect} from "react";

export default function DashBoard(){
    const [zecPrice , setZecPrice] = useState<number |null>(null);
    const [action , setAction] = useState<string |null>("send");
    const tokens = [
    {
        "token" : "BTC",
        "name" : "Bitcoin",
        "dataapi" : "https://api.diadata.org/v1/assetQuotation/Bitcoin/0x0000000000000000000000000000000000000000"
    },
    {
        "token" : "SOL",
        "name" : "Solana",
        "dataapi" : "https://api.diadata.org/v1/assetQuotation/Solana/0x0000000000000000000000000000000000000000"
    },
    {
        "token" : "SUI",
        "name" : "Solana",
        "dataapi" : ""
    },
    {
        "token" : "ETH",
        "name" : "Ethereum",
        "dataapi" : "https://api.diadata.org/app/price/asset/Ethereum/0x0000000000000000000000000000000000000000/"
    },
    {
        "token" : "ZEC",
        "name" : "Zcash",
        "dataapi" : "https://api.diadata.org/v1/assetQuotation/Zcash/0x0000000000000000000000000000000000000000"
    },
    {
        "token" : "USDT",
        "name" : "Tether",
        "dataapi" : "https://api.diadata.org/v1/assetQuotation/Tether/0x0000000000000000000000000000000000000000"
    },
    {
        "token" : "USDC",
        "name" : "USD Coin",
        "dataapi" : "https://www.diadata.org/app/price/asset/Ethereum/0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48/"
    },
    {
        "token" : "SUI",
        "name" : "Sui",
        "dataapi" : "https://www.diadata.org/app/price/asset/Sui/0x2::sui::SUI/"
    }
    ]

    useEffect(() => {
        axios
            .get("https://api.diadata.org/v1/assetQuotation/Zcash/0x0000000000000000000000000000000000000000")
            .then((res) => {
                // The price from diadata usually comes as res.data.Price
                setZecPrice(res?.data.Price ?? null);
            })
            .catch(() => setZecPrice(null));
    }, []);

    const getTokenPrice = (url: string) => {
        return axios.get(url)
            .then(res => Math.ceil(res?.data?.Price))
            .catch(() => null);
    };
    const [username , setUserName] = useState<string | null >(null);
    const actions = ["Send" , "Receive" , "Buy" , "Send"]
    
    return(
        <React.Fragment>
        <div className="py-3.5 cursor-pointer px-6 border border-transparent hover:border-white/20 bg-[#020202] text-white w-[95%] mx-auto rounded-sm mt-3 flex justify-between transition-colors duration-700">
            <h3>Night.fall</h3>
            <div className="items-center flex">
            <h3 className="text-xl mr-3.5"> ZEC : ${Math.ceil(zecPrice)}</h3>
            <Sun className="cursor-pointer text-2xl sun_mode"/>
            </div>
        </div>
        <div className="text-white w-[87%] mx-auto mt-10 px-5">
        <div className="text-center xl:text-right">
            <h2 className="text-5xl">${Math.ceil(zecPrice)}</h2>
        </div>
        <div className="mt-5 pt-3 border-t-2">
            <div className="actions space-x-2 pt-1.5">
                {/* {for i in actions} */}
                <Button className="px-11 py-7 border border-white/20 bg-black text-base font-semibold text-white transition hover:bg-white hover:text-black disabled:cursor-not-allowed disabled:border-white/10 disabled:text-white/50 cursor-pointer">Send <Send/></Button>
                <Button className="px-10 py-7 border border-white/20 bg-black text-base font-semibold text-white transition hover:bg-white hover:text-black disabled:cursor-not-allowed disabled:border-white/10 disabled:text-white/50 cursor-pointer">Receive <ArrowDownToLine/></Button>
                <Button className="px-10 py-7 border border-white/20 bg-black text-base font-semibold text-white transition hover:bg-white hover:text-black disabled:cursor-not-allowed disabled:border-white/10 disabled:text-white/50 cursor-pointer">Pay<CreditCard/></Button>
                <Button className="px-10 py-7 border border-white/20 bg-black text-base font-semibold text-white transition hover:bg-white hover:text-black disabled:cursor-not-allowed disabled:border-white/10 disabled:text-white/50 cursor-pointer">Swap<ArrowLeftRight/></Button>
            </div>
        <div>
        <div className="flex flex-col px-4 w-[50%] border-white/10 mb-2 justify-between border border-2 rounded-md mt-3.5">
            <div className="flex justify-between w-full">
                <div className="w-1/3 text-left text-sm font-semibold text-white/70 py-2">
                    Token
                </div>
                <div className="text-right text-sm font-semibold text-white/70 py-2">
                    Price
                </div>
            </div>
            {
                Array.from(tokens).map((token , index) => {
                    return(
                        <div className="flex justify-between w-full border-b-2 border-b-white py-3" key={index}>
                            <div className="w-1/3 text-left text-sm font-semibold text-white/70 py-2">
                                {token.name}
                            </div>
                            <div className="text-right text-sm font-semibold text-white/70 py-2">
                                {getTokenPrice(token.dataapi)}
                            </div>
                        </div>
                    )
                })
            }
        </div>
        </div>
        </div>
        </div>
        </React.Fragment>
    )

}