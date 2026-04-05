import litFlameIcon from "./assets/flame-lit.svg";
import unlitFlameIcon from "./assets/flame-unlit.svg";
import gearIcon from "./assets/gear.svg";
import graphIcon from "./assets/graph.svg";
import magGlassIcon from "./assets/magnifying-glass.svg";
import reloadIcon from "./assets/reload.svg";
import stopwatchIcon from "./assets/stopwatch.svg";
import wheelIcon from "./assets/wheel.svg";

type MenuButtonProps = {
    text: string;
    src: string;
    onClick?: (event: React.MouseEvent<HTMLButtonElement, MouseEvent>) => void;
    active?: boolean;
};

function MenuButton({ text, src, onClick, active }: MenuButtonProps) {
    return (
        <button
            onClick={onClick}
            className={` rounded-2xl w-full max-h-65 flex items-center active:bg-[#3B28CC] ${active ? " bg-[#3B28CC]" : "bg-[#16171cbf] hover:bg-[#16171c80]"}`}
        >
            <img src={src} className="w-15 h-15" />
            <text className="font-[Electrolize] text-white text-2xl">{text}</text>
        </button>
    );
}

type TopPillProps = {
    isLit: boolean;
};

function TopPill({ isLit }: TopPillProps) {
    return (
        <div className="bg-[#16171cbf] rounded-full p-1 h-full flex items-center">
            <button className="hover:bg-[#16171c80] active:bg-[#3B28CC] rounded-md h-full flex items-center justify-center ">
                <img src={gearIcon} className="h-8 w-8" />
            </button>
            <div className="flex flex items-center ml-auto">
                <div className="flex items-center justify-center mr-1">
                    <img src={isLit ? litFlameIcon : unlitFlameIcon} className="h-7 w-7" />
                    <text className="font-[Electrolize] text-white text-lg">1 Day</text>
                </div>
                <div className="flex items-center justify-center mr-1">
                    <img src={reloadIcon} className="h-8 w-8" />
                    <text className="font-[Electrolize] text-white text-lg">12 Laps</text>
                </div>
                <div className="flex items-center justify-center mr-1">
                    <img src={wheelIcon} className="h-7 w-7" />
                    <text className="font-[Electrolize] text-white text-lg">1h 50min</text>
                </div>
            </div>
        </div>
    );
}

export default function Sidebar() {
    return (
        <div className="bg-[#FFFFFF1a] h-full w-full rounded-2xl ml-2 min-w-90">
            <div className="p-2">
                <div className="h-12 mb-5">
                    <TopPill isLit={true} />
                </div>
                <div>
                    <text className="text-white text-2xl font-[Days_One]">ANALYZE</text>
                    <div>
                        <div className="mb-1">
                            <MenuButton src={graphIcon} text="Telemetry" active={false} />
                        </div>
                        <div className="mb-1">
                            <MenuButton src={stopwatchIcon} text="Live Timing" active={false} />
                        </div>
                        <div className="mb-1">
                            <MenuButton src={magGlassIcon} text="Analysis" active={false} />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}
