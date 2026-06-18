import { SVGProps, ComponentType } from "react";
import HamburgerMenuIcon from "../assets/icons/hamburger-menu.svg?react";
import LiveTimingsIcon from "../assets/icons/stopwatch.svg?react";
import TelemetryIcon from "../assets/icons/graph.svg?react";
import AnalysisIcon from "../assets/icons/analysis-icon.svg?react";
import SetupsIcon from "../assets/icons/setups.svg?react";
import { Page } from "../App.tsx";
import { useState } from "react";
import { Divider } from "antd";

type TopRowProps = {
  isOpen: boolean;
  setIsOpen: (value: boolean) => void;
};
function TopRow({ setIsOpen, isOpen }: TopRowProps) {
  return (
    <div className="pt-2 pl-2 pr-2 flex justify-between items-center">
      <button
        className="hover:bg-[#FFFFFF18] rounded-full"
        onClick={() => setIsOpen(isOpen ? false : true)}
      >
        <HamburgerMenuIcon className="w-15 h-15" />
      </button>
      {isOpen ? (
        <div className="rounded-full bg-blue-500 w-12 h-12 mask-clip-content">
          <img />
        </div>
      ) : (
        <div />
      )}
    </div>
  );
}

type SidebarButtonProps = {
  Icon: ComponentType<SVGProps<SVGSVGElement>>;
  text: string;
  isActive: boolean;
  id: Page;
  onClick: (id: Page) => void;
  maximized: boolean;
};

function SidebarButton({
  Icon,
  text,
  isActive,
  id,
  onClick,
  maximized,
}: SidebarButtonProps) {
  return (
    <button
      className={`flex h-15 rounded-2xl justify-start items-center p-2 ${!isActive ? "bg-[#16171CC0] hover:bg-[#FFFFFF18] active:bg-[#3B28CC]" : "bg-[#3B28CC]"}`}
      onClick={() => {
        onClick(id);
      }}
    >
      <Icon className={maximized ? " h-10 w-10 mr-4" : "h-10 w-10"} />
      {maximized ? (
        <div className="font-[Electrolize] text-white text-2xl">{text}</div>
      ) : (
        <></>
      )}
    </button>
  );
}

type SidebarProps = {
  activePage: Page;
  onPageChange: (id: Page) => void;
  isOpen: boolean;
  setIsOpen: (value: boolean) => void;
};

export default function Sidebar({
  activePage,
  onPageChange,
  isOpen,
  setIsOpen,
}: SidebarProps) {
  return (
    <div className="h-full w-full bg-[#FFFFFF18] rounded-4xl ">
      <TopRow isOpen={isOpen} setIsOpen={setIsOpen} />
      <div className="mt-4 pl-2 pr-2">
        {isOpen ? (
          <div className="font-[Days_One] text-white text-3xl mb-2">
            ANALYZE
          </div>
        ) : (
          <Divider />
        )}
        <div className="flex flex-col gap-1">
          <SidebarButton
            Icon={TelemetryIcon}
            text="Telemetry"
            isActive={activePage == Page.Telemetry}
            onClick={(id) => onPageChange(id)}
            id={Page.Telemetry}
            maximized={isOpen}
          />
          <SidebarButton
            Icon={LiveTimingsIcon}
            text="Live Timings"
            isActive={activePage == Page.LiveTimings}
            onClick={(id) => onPageChange(id)}
            id={Page.LiveTimings}
            maximized={isOpen}
          />
          <SidebarButton
            Icon={AnalysisIcon}
            text="Analysis"
            isActive={activePage == Page.Analysis}
            onClick={(id) => onPageChange(id)}
            id={Page.Analysis}
            maximized={isOpen}
          />
        </div>
      </div>
      <div className="mt-4 pl-2 pr-2">
        {isOpen ? (
          <div className="font-[Days_One] text-white text-3xl mb-2">
            PREPARE
          </div>
        ) : (
          <Divider />
        )}
        <div className="flex flex-col gap-1">
          <SidebarButton
            Icon={SetupsIcon}
            text="Setups"
            isActive={activePage == Page.Setups}
            onClick={(id) => onPageChange(id)}
            id={Page.Setups}
            maximized={isOpen}
          />
        </div>
      </div>
    </div>
  );
}
