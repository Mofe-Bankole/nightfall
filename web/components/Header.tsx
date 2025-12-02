import { Sun } from "lucide-react";
import axios from "axios";
import { useState } from "react";
import { useEffect } from "react";

export default function Header() {
  const [zecPrice, setZecPrice] = useState<number | null>(null);

  useEffect(() => {
    axios
      .get(
        "https://api.diadata.org/v1/assetQuotation/Zcash/0x0000000000000000000000000000000000000000",
      )
      .then((res) => {
        // The price from diadata usually comes as res.data.Price
        setZecPrice(res?.data.Price ?? null);
      })
      .catch(() => setZecPrice(null));
  }, []);

  return (
    <div className="py-3.5 px-6 bg-[#020202] text-white w-[95%] mx-auto rounded-sm mt-2.5 flex justify-between">
      <h3>Night.fall</h3>
      <div className="flex items-center">
        <h3 className="text-xl mr-3.5">ZEC : ${Math.ceil(zecPrice)}</h3>
        {/*<Sun className="cursor-pointer text-2xl sun_mode" />*/}
      </div>
    </div>
  );
}
