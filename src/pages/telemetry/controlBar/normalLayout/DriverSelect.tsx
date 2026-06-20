import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "../../../../components/ui/select";
import { useContext, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { TelemetryContext } from "@/pages/telemetry/Telemetry";

type Driver = {
  index: number;
  name: string;
};

export default function DriverSelect() {
  const [drivers, setDrivers] = useState<Driver[]>([]);
  const c = useContext(TelemetryContext);

  useEffect(() => {
    invoke<Driver[]>("get_drivers").then((v) => {
      setDrivers(v);
    });
  }, []);

  const selectedDriver = drivers.find(
    (driver) => driver.index === c.curDriverNum,
  );

  return (
    <div className="flex gap-1 justify-center items-center p-1">
      <span>Driver:</span>
      <Select
        value={String(c.curDriverNum)}
        onValueChange={(value: string) => c.setCurDriverNum(Number(value))}
      >
        <SelectTrigger className="w-fit">
          <SelectValue
            placeholder={selectedDriver?.name ?? "Select a Driver"}
          />
        </SelectTrigger>
        <SelectContent position="popper">
          <SelectGroup>
            {drivers.map((driver) => (
              <SelectItem key={driver.index} value={String(driver.index)}>
                {driver.name}
              </SelectItem>
            ))}
          </SelectGroup>
        </SelectContent>
      </Select>
    </div>
  );
}
