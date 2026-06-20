import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectSeparator,
  SelectTrigger,
  SelectValue,
} from "../../../../components/ui/select";
import { useContext, useEffect, useState } from "react";
import { TelemetryContext } from "@/pages/telemetry/Telemetry";

export default function LayoutSelect() {
  const c = useContext(TelemetryContext);

  function handleLayout(key: string) {
    if (key == "edit") {
      c.setEditMode(true);
    } else {
      c.setActiveLayout(Number(key));
      c.setGraphData(c.layouts[Number(key)].graphData);
      c.setSizes(c.layouts[Number(key)].scales);
    }
  }

  return (
    <div className="flex gap-1 justify-center items-center p-1">
      <span>Layout:</span>
      <Select value={String(c.activeLayout)} onValueChange={handleLayout}>
        <SelectTrigger className="w-fit">
          <SelectValue placeholder={"No driver avalible"} />
        </SelectTrigger>
        <SelectContent position="popper">
          <SelectGroup>
            {c.layouts.map((layout, i) => (
              <SelectItem value={String(i)}>{layout.name}</SelectItem>
            ))}
          </SelectGroup>
          <SelectSeparator />
          <SelectGroup>
            <SelectItem value="edit">Edit</SelectItem>
          </SelectGroup>
        </SelectContent>
      </Select>
    </div>
  );
}
