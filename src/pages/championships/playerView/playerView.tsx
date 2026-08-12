import { useCallback, useEffect, useState } from "react";
import { CarClass } from "../sidebar/playerCard";
import DamageCard, { DamageData, Severity } from "./damage/damage";
import FuelCard, { FuelData } from "./fuel/fuel";
import TiresCard, { Tires } from "./tires/tires";
import { invoke } from "@tauri-apps/api/core";

type StaleDriverInfo = {
  name: string;
  car: string;
  car_class: string;
};

type DynDriverInfo = {
  damages: DamageData[];
  tires: Tires;
  fuel: FuelData;
};

export default function PlayerView({ curDriverId }: { curDriverId: number }) {
  const [staleDriverInfo, setStaleDriverInfo] = useState<StaleDriverInfo>({
    car: "",
    name: "",
    car_class: "",
  });
  const [dynDriverInfo, setDynDriverInfo] = useState<DynDriverInfo>({
    damages: [],
    tires: {
      fl: { brake_temp: 0, health: 0, inside_temp: 0, outside_temp: 0 },
      fr: { brake_temp: 0, health: 0, inside_temp: 0, outside_temp: 0 },
      rl: { brake_temp: 0, health: 0, inside_temp: 0, outside_temp: 0 },
      rr: { brake_temp: 0, health: 0, inside_temp: 0, outside_temp: 0 },
    },
    fuel: { fuel: 0, max_fuel: 0, ve: 0 },
  });

  useEffect(() => {
    invoke<StaleDriverInfo>("get_stale_driver_info", {
      curDriverId,
    }).then((v) => {
      setStaleDriverInfo(v);
    });

    const interval = setInterval(() => {
      console.log("ran interval");

      invoke<DynDriverInfo>("get_dyn_driver_info", {
        curDriverId,
      }).then((v) => {
        setDynDriverInfo(v);
        console.log(v);
      });
    }, 1000);

    return () => {
      clearInterval(interval);
    };
  }, [curDriverId]);

  return (
    <div className="w-full h-full rounded-[24px] p-4 bg-[#16171C] flex flex-col gap-2">
      <div className="font-[Racing_Sans_One] text-[32px]">
        {staleDriverInfo.name}
        <div className="font-[Inter] text-[16px] flex items-center gap-4">
          {staleDriverInfo.car}
          <ClassBadge carClass={getCarClass(staleDriverInfo.car_class)} />
        </div>
      </div>
      <div className="flex w-full gap-2 min-h-[200px]">
        <TiresCard tires={dynDriverInfo.tires} />
        {/* <DamageCard damages={dynDriverInfo.damages} /> */}
      </div>
      <div>
        <FuelCard fuel={dynDriverInfo.fuel} />
      </div>
    </div>
  );
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

function getCarClass(className: string): CarClass {
  switch (className) {
    case "Hyper":
      return CarClass.HY;
    case "LMP2_ELMS":
    case "LMP2":
      return CarClass.P2;

    case "LMP3":
      return CarClass.P3;

    case "GTE":
      return CarClass.GTE;

    case "GT3":
      return CarClass.GT3;

    default:
      return CarClass.GTE;
  }
}
