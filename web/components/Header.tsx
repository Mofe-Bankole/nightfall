import { Sun } from "lucide-react";

export default function Header(){

    return(
        <div className="py-3.5 px-6 bg-[#020202] text-white w-[95%] mx-auto rounded-sm mt-2.5 flex justify-between">
            <h3>Night.fall</h3>
            <Sun className="cursor-pointer text-2xl sun_mode"/>
        </div>
    )
}