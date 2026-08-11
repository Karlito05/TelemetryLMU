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
      <div></div>
    </div>
  );
}
