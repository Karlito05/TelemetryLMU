import { useState } from "react";
import { Button } from "./button";
import { Input } from "./input";
import { IconCaretDown, IconCaretUp } from "@tabler/icons-react";

export function NumberInput({
  onValueChange,
  defaultValue,
  min,
  max,
}: {
  defaultValue?: number;
  onValueChange?: (value: number) => void;
  min?: number;
  max?: number;
}) {
  const [value, setValue] = useState(defaultValue ?? 0);

  function handleButtonClick(val: number) {
    if (min && value + val < min) return;
    if (max && value + val > max) return;
    if (onValueChange) onValueChange(value + val);
    setValue(value + val);
  }

  return (
    <div className="flex items-center justify-center">
      <Input className="w-12 rounded-none rounded-l-2xl h-full" value={value} />
      <div className="flex flex-col overflow-hidden rounded-r-2xl">
        <Button
          size="icon"
          className="rounded-none h-fit border-0"
          onClick={() => handleButtonClick(1)}
        >
          <IconCaretUp />
        </Button>
        <Button
          size="icon"
          className="rounded-none h-fit border-0"
          onClick={() => handleButtonClick(-1)}
        >
          <IconCaretDown />
        </Button>
      </div>
    </div>
  );
}
