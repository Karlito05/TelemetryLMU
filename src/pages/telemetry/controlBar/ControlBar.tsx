import { useContext } from "react";
import { TelemetryContext } from "../Telemetry";
import NormalLayout from "./normalLayout/NormalLayout";
import EditLayout from "./editLayout/EditLayout";

export default function ControlBar() {
  const c = useContext(TelemetryContext);
  return (
    <div className="bg-(--middleground) h-full w-full rounded-[24px] items-center flex justify-baseline overflow-x-auto">
      {c.editMode ? <EditLayout /> : <NormalLayout />}
    </div>
  );
}
