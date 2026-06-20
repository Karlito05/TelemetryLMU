import { useContext } from "react";
import { TelemetryContext } from "../Telemetry";
import NormalLayout from "./normalLayout/NormalLayout";
import EditLayout from "./editLayout/EditLayout";

export default function ControlBar() {
  const c = useContext(TelemetryContext);
  return (
    <div className="bg-[#FFFFFF18] h-full w-full rounded-2xl items-center flex justify-baseline ">
      {c.editMode ? <EditLayout /> : <NormalLayout />}
    </div>
  );
}
