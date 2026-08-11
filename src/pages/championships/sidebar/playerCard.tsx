import { useEffect, useState } from "react";

export default function PlayerCard({
  name,
  car,
  carClass,
  driverId,
  setCurDriverId,
  curDriverId,
}: {
  name: string;
  car: string;
  carClass: CarClass;
  driverId: number;
  setCurDriverId: (v: number) => void;
  curDriverId: number;
}) {
  return (
    <button
      className={`relative w-full p-3 flex flex-col items-start ${
        driverId === curDriverId ? "bg-[#2D2E32]" : "bg-[#16171C]"
      }`}
      onClick={() => setCurDriverId(driverId)}
    >
      <div className="flex flex-col items-start">
        <div className="text-[32px] text-white font-[Racing_Sans_One]">{name}</div>
        <div className="text-[16px] text-white font-[Inter]">{car}</div>
      </div>
      <div className="absolute top-5 right-5">
        <ClassBadge carClass={carClass} />
      </div>
    </button>
  );
}

export enum CarClass {
  GT3,
  GTE,
  P3,
  P2,
  HY,
}

function ClassBadge({ carClass }: { carClass: CarClass }) {
  const classes = {
    [CarClass.GT3]: { name: "GT3", color: "#0D9D00" },
    [CarClass.GTE]: { name: "GTE", color: "#FFCC00" },
    [CarClass.P3]: { name: "P3", color: "#7B00FF" },
    [CarClass.P2]: { name: "P2", color: "#006BDD" },
    [CarClass.HY]: { name: "HY", color: "#DF271C" },
  };

  const { name, color } = classes[carClass];

  return (
    <div
      style={{
        color,
        borderColor: color,
      }}
      className="font-bold border-2 rounded-[4px] w-[48px] h-[22px] flex items-center justify-center text-[16px]"
    >
      {name}
    </div>
  );
}
