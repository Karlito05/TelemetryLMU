import { NumberInput } from "@/components/ui/number-input";

export default function SampleRate() {
  return (
    <div className="flex gap-2 justify-center items-center">
      <span className="text-base text-nowrap">Sample Rate:</span>
      <NumberInput unit=" Hz" step={15} />
    </div>
  );
}
