import { CarClass } from "../sidebar/playerCard";
import DamageCard, { DamageData, Severity } from "./damage/damage";
import FuelCard from "./fuel/fuel";
import TiresCard, { Tires } from "./tires/tires";

type StaleDriverInfo = {
  name: string;
  car: string;
  car_class: string;
};

type DynDriverInfo = {
  damages: DamageData[];
  tires: Tires;
};

export default function PlayerView({ curDriverId }: { curDriverId: number }) {
  return (
    <div className="w-full h-full rounded-[24px] p-4 bg-[#16171C] flex flex-col gap-2">
      <div className="font-[Racing_Sans_One] text-[32px]">
        name
        <div className="font-[Inter] text-[16px] flex items-center gap-4">
          car
          <ClassBadge carClass={CarClass.GT3} />
        </div>
      </div>
      <div className="flex w-full gap-2 min-h-[200px]">
        <TiresCard
          tires={{
            fl: { outside_temp: 71, inside_temp: 67, brake_temp: 56, health: 0.91 },
            fr: { outside_temp: 74, inside_temp: 65, brake_temp: 45, health: 0.92 },
            rl: { outside_temp: 84, inside_temp: 59, health: 0.87, brake_temp: 30 },
            rr: { inside_temp: 66, outside_temp: 90, health: 0.85, brake_temp: 44 },
          }}
        />
        <DamageCard
          damages={[
            { severity: Severity.Minor, damageMsg: "Damaged Front Left Suspension" },
            { severity: Severity.Moderate, damageMsg: "Difuser Heavily Damaged" },
            { severity: Severity.Major, damageMsg: "No Rear Wing" },
          ]}
        />
      </div>
      <div>
        <FuelCard />
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
