import { IconAlertTriangle } from "@tabler/icons-react";

export type DamageData = {
  damage_msg: string;
  severity: Severity;
};

export enum Severity {
  Minor,
  Moderate,
  Major,
}

export default function DamageCard({ damages }: { damages: DamageData[] }) {
  return (
    <div className="bg-[#FFFFFF0C] rounded-[24px] p-4 flex-1">
      <div className="text-[#FFFFFF60] text-[14px] font-[Barlow_Condesed] flex gap-1 items-center">
        <IconAlertTriangle size={12} /> DAMAGE REPORT
      </div>
      <div>
        {damages.map((d) => {
          return <Damage damage={d} />;
        })}
      </div>
    </div>
  );
}

function Damage({ damage }: { damage: DamageData }) {
  return (
    <div className="flex items-center gap-2 font-[Jet_Brains_Mono]">
      <IconAlertTriangle
        size={16}
        color={
          damage.severity == Severity.Minor
            ? "#FFFF00"
            : damage.severity == Severity.Moderate
              ? "#FF6B35"
              : "#FF0000"
        }
      />
      {damage.damage_msg}
    </div>
  );
}
