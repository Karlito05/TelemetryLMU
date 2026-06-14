import Graphs from "./Graphs.tsx";
import ControlBar from "./ControlBar.tsx";
import { useState } from "react";

export default function Telemetry() {
  let [curDriverNum, setCurDriverNum] = useState(0);

  return (
    <div className=" h-full w-full">
      <div className="h-6/100 pb-0.5">
        <ControlBar setCurDriverNum={setCurDriverNum} />
      </div>
      <div className="h-94/100">
        <Graphs curDriverNum={curDriverNum} />
      </div>
    </div>
  );
}
