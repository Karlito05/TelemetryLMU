import { Switch } from "@/components/ui/switch";
import { invoke } from "@tauri-apps/api/core";
import { useContext, useEffect, useState } from "react";
import { TelemetryContext } from "../../Telemetry";

export default function Record() {
  const c = useContext(TelemetryContext);
  const [checked, setChecked] = useState(true);

  useEffect(() => {
    if (checked) {
      invoke("spawn_logger", { carNum: c.curDriverNum }).catch((e) =>
        console.error("spawn_logger failed:", e),
      );
    } else {
      invoke("despawn_logger").catch((e) => console.error("despawn_logger failed:", e));
    }
    return () => {
      invoke("despawn_logger").catch((e) => console.error("despawn_logger cleanup failed:", e));
    };
  }, [checked, c.curDriverNum]);

  return (
    <div className="flex gap-2 justify-center items-center">
      <span>Record:</span>
      <Switch onCheckedChange={setChecked} checked={checked} />
    </div>
  );
}
