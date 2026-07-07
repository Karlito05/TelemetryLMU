import { useState } from "react";
import { Button } from "./button";
import { Input } from "./input";
import { IconCaretDown, IconCaretUp } from "@tabler/icons-react";

export function NumberInput({
  onValueChange,
  defaultValue,
  min,
  max,
  unit,
  step,
}: {
  defaultValue?: number;
  onValueChange?: (value: number) => void;
  min?: number;
  max?: number;
  unit?: string;
  step?: number;
}) {
  const [value, setValue] = useState(defaultValue ?? 0);
  const [dispVal, setDispVal] = useState(value.toString() + (unit ?? ""));
  step = step ? step : 1;

  function handleButtonClick(val: number) {
    if (min && value + val < min) return;
    if (max && value + val > max) return;
    if (onValueChange) onValueChange(value + val);
    setValue(value + val);
    setDispVal((value + val).toString() + (unit ? unit : ""));
  }

  return (
    <div className="flex items-center justify-center h-8">
      <Input className="w-24 rounded-none rounded-l-[8px] h-full" value={dispVal} />
      <div className="flex flex-col overflow-hidden rounded-r-[8px] h-full">
        <Button
          size="icon"
          variant={"ghost"}
          className="rounded-none rounded-tr-[8px] bg-[rgba(256,256,256,0.075)] h-1/2 border-0"
          onClick={() => handleButtonClick(step)}
        >
          <IconCaretUp />
        </Button>
        <Button
          size="icon"
          variant={"ghost"}
          className="rounded-none rounded-br-[8px] bg-[rgba(256,256,256,0.075)] h-1/2 border-0"
          onClick={() => handleButtonClick(-step)}
        >
          <IconCaretDown />
        </Button>
      </div>
    </div>
  );
}
