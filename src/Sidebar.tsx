import { SVGProps, ComponentType } from "react";
import SettingsIcon from "./assets/gear.svg?react";
import LiveTimingsIcon from "./assets/stopwatch.svg?react";
import TelemetryIcon from "./assets/graph.svg?react";
import AnalysisIcon from "./assets/analysis-icon.svg?react";
import SetupsIcon from "./assets/setups.svg?react";
import { Page } from "./App.tsx";

function TopRow() {
  return (
    <div className="pt-2 pl-2 pr-2 flex justify-between">
      <div className="rounded-full bg-blue-500 w-12 h-12 mask-clip-content">
        <img />
      </div>
      <button className="hover:bg-[#FFFFFF18] rounded-full">
        <SettingsIcon className="w-12 h-12" />
      </button>
    </div>
  );
}

type SidebarButtonProps = {
  Icon: ComponentType<SVGProps<SVGSVGElement>>;
  text: string;
  isActive: boolean;
  id: Page;
  onClick: (id: Page) => void;
};

function SidebarButton({ Icon, text, isActive, id, onClick }: SidebarButtonProps) {
  return (
    <button
      className={`flex h-15 rounded-2xl justify-start items-center p-2 ${!isActive ? "bg-[#16171CC0] hover:bg-[#FFFFFF18] active:bg-[#3B28CC]" : "bg-[#3B28CC]"}`}
      onClick={() => {
        onClick(id);
      }}
    >
      <Icon className=" h-10 w-10 mr-4" />
      <div className="font-[Electrolize] text-white text-2xl">{text}</div>
    </button>
  );
}

type SidebarProps = {
  activePage: Page;
  onPageChange: (id: Page) => void;
};

export default function Sidebar({ activePage, onPageChange }: SidebarProps) {
  return (
    <div className="h-full w-full bg-[#FFFFFF18] rounded-4xl ">
      <TopRow />
      <div className="mt-4 pl-2 pr-2">
        <div className="font-[Days_One] text-white text-3xl mb-2">ANALYZE</div>
        <div className="flex flex-col gap-1">
          <SidebarButton
            Icon={TelemetryIcon}
            text="Telemetry"
            isActive={activePage == Page.Telemetry}
            onClick={(id) => onPageChange(id)}
            id={Page.Telemetry}
          />
          <SidebarButton
            Icon={LiveTimingsIcon}
            text="Live Timings"
            isActive={activePage == Page.LiveTimings}
            onClick={(id) => onPageChange(id)}
            id={Page.LiveTimings}
          />
          <SidebarButton
            Icon={AnalysisIcon}
            text="Analysis"
            isActive={activePage == Page.Analysis}
            onClick={(id) => onPageChange(id)}
            id={Page.Analysis}
          />
        </div>
      </div>
      <div className="mt-4 pl-2 pr-2">
        <div className="font-[Days_One] text-white text-3xl mb-2">PREPARE</div>
        <div className="flex flex-col gap-1">
          <SidebarButton
            Icon={SetupsIcon}
            text="Setups"
            isActive={activePage == Page.Setups}
            onClick={(id) => onPageChange(id)}
            id={Page.Setups}
          />
        </div>
      </div>
    </div>
  );
}
