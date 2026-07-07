import { NumberInput } from "@/components/ui/number-input";
import { useContext } from "react";
import { TelemetryContext } from "../../Telemetry";

export default function SampleRate() {
  const c = useContext(TelemetryContext);
  return (
    <div className="flex gap-2 justify-center items-center">
      <span className="text-base text-nowrap">Sample Rate:</span>
      <NumberInput
        unit=" Hz"
        step={15}
        onValueChange={c.setSampleRate}
        defaultValue={c.sampleRate}
      />
    </div>
  );
}
