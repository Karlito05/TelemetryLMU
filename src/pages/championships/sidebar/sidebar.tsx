import { useEffect, useState } from "react";
import PlayerCard, { CarClass } from "./playerCard";
import { invoke } from "@tauri-apps/api/core";

type ChampDriverData = {
  name: string;
  car: string;
  car_class: string;
  id: number;
};

export default function Sidebar({
  curDriverId,
  setCurDriverId,
}: {
  curDriverId: number;
  setCurDriverId: (v: number) => void;
}) {
  const [drivers, setDrivers] = useState<ChampDriverData[]>([]);

  useEffect(() => {
    invoke<ChampDriverData[]>("get_champ_drivers").then((v) => {
      setDrivers(v);
    });
  }, []);

  return (
    <div className="min-w-[350px] max-w-[400px] w-2/10 rounded-[24px] gap-1.5 flex flex-col overflow-scroll">
      {drivers.map((d) => {
        return (
          <PlayerCard
            name={d.name}
            car={d.car.split(":")[0]}
            carClass={getCarClass(d.car_class)}
            key={d.id}
            driverId={d.id}
            curDriverId={curDriverId}
            setCurDriverId={setCurDriverId}
          />
        );
      })}
    </div>
  );
}
function getCarClass(className: string): CarClass {
  switch (className) {
    case "Hyper":
      return CarClass.HY;

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
