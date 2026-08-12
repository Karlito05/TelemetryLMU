import { IconGasStation } from "@tabler/icons-react";

export type FuelData = {
  fuel: number;
  max_fuel: number;
  ve: number;
};

export default function FuelCard({ fuel }: { fuel: FuelData }) {
  return (
    <div className="bg-[#FFFFFF0C] rounded-[24px] p-4 flex-1">
      <div className="text-[#FFFFFF60] text-[14px] font-[Barlow_Condesed] flex gap-1 items-center">
        <IconGasStation size={12} /> {"FUEL & VE"}
      </div>
      <div className="flex flex-col gap-2">
        <div className="flex gap-2">
          <div className="bg-amber-600/25  size-10 rounded-[8px] flex items-center justify-center">
            <IconGasStation className="stroke-amber-600" />
          </div>
          <div className="flex flex-col flex-1">
            <div className="flex gap-1 items-end font-[Jet_Brains_Mono]">
              <div>{Math.round((fuel.fuel / fuel.max_fuel) * 100)}%</div>
              <div className="text-[14px] text-white/50">{fuel.fuel}L</div>
            </div>
            <div className="flex rounded-full w-full h-3 bg-white/10">
              <div
                className="h-full rounded-full bg-amber-600"
                style={{ width: `${(fuel.fuel / fuel.max_fuel) * 100}%` }}
              />
            </div>
          </div>
        </div>
        <div className="flex gap-2">
          <div className="bg-sky-600/25  size-10 rounded-[8px] flex items-center justify-center">
            <IconGasStation className="stroke-sky-600" />
          </div>
          <div className="flex flex-col flex-1">
            <div className="flex gap-1 items-end font-[Jet_Brains_Mono]">
              <div>{Math.round(fuel.ve * 100)}%</div>
            </div>
            <div className="flex rounded-full w-full h-3 bg-white/10">
              <div
                className="h-full rounded-full bg-sky-600"
                style={{ width: `${fuel.ve * 100}%` }}
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
