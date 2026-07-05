import { NumberInput } from "@/components/ui/number-input";

export default function Display() {
  return (
    <div className="flex gap-2 justify-center items-center">
      <span>Display</span>
      <NumberInput unit=" Datapoints" step={250} />
    </div>
  );
}
