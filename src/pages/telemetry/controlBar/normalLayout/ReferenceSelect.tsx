import { useContext, useState } from "react";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectSeparator,
  SelectTrigger,
  SelectValue,
} from "../../../../components/ui/select";
import { TelemetryContext } from "../../Telemetry";
import { BaseDirectory, readTextFile } from "@tauri-apps/plugin-fs";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import { DataPoint } from "../../graphs/normalMode/GraphViewNew";

export type SaveData = {
  lap_info: { lap_time: number; date: string };
  data: { data_type: string; data: DataPoint[] }[];
};

export default function ReferenceSelect() {
  const c = useContext(TelemetryContext);
  const [activeReferenceName, setActiveRefenceName] = useState("session");

  async function handleReference(value: string) {
    if (value == "session") {
      c.setActiveRefence(null);
      setActiveRefenceName("session");
    }
    if (value == "open") {
      const filePath = await openDialog({
        multiple: false,
        directory: false,
      });
      if (!filePath) return;
      const file = await readTextFile(filePath, {
        baseDir: BaseDirectory.AppData,
      });

      const saveData = JSON.parse(file) as SaveData;
      c.setActiveRefence(saveData);
      setActiveRefenceName("open");
    }
  }

  return (
    <div className="flex gap-1 justify-center items-center p-1">
      <span>Reference: </span>
      <Select value={activeReferenceName} onValueChange={handleReference}>
        <SelectTrigger className="w-fit">
          <SelectValue placeholder={"No driver avalible"} />
        </SelectTrigger>
        <SelectContent position="popper">
          <SelectGroup>
            <SelectItem value="session">Session best</SelectItem>
          </SelectGroup>
          <SelectSeparator />
          <SelectGroup>
            <SelectItem value="open">From a File</SelectItem>
          </SelectGroup>
        </SelectContent>
      </Select>
    </div>
  );
}
