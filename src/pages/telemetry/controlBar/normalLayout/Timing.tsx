import { IconMedal, IconStopwatch } from "@tabler/icons-react";
export default function Timing() {
  return (
    <div>
      <div className="flex font-[Electrolize] text-md gap-1">
        <IconMedal stroke={1.5} />
        {"2.27.151"}
      </div>
      <div className="flex font-[Electrolize] text-md gap-1">
        <IconStopwatch stroke={1.5} />
        {"2.26.623"}
        <span className="text-green-400">{"-0.125"}</span>
      </div>
    </div>
  );
}
