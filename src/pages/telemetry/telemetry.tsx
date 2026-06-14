import Graphs from "./Graphs.tsx";
import ControlBar from "./ControlBar.tsx";
import { useState } from "react";
import { GraphViewData, GraphViewType } from "./Graphs.tsx";

export default function Telemetry() {
  let [curDriverNum, setCurDriverNum] = useState(0);
  const [editMode, setEditMode] = useState(false);
  const [sizes, setSizes] = useState<number[]>([25, 75]);
  const [graphData, setGraphData] = useState<GraphViewData[]>([
    {
      baseColor: "#9eff5d",
      carNum: curDriverNum,
      graphName: "Throttle",
      nLines: 3,
      type: GraphViewType.Throttle,
    },
    {
      baseColor: "#ff5d5d",
      carNum: curDriverNum,
      graphName: "Brake",
      nLines: 3,
      type: GraphViewType.Brake,
    },
  ]);

  return (
    <div className=" h-full w-full">
      <div className="h-6/100 pb-0.5">
        <ControlBar
          curDriverNum={curDriverNum}
          setCurDriverNum={setCurDriverNum}
          editMode={editMode}
          setEditMode={setEditMode}
          graphData={graphData}
          setGraphData={setGraphData}
          sizes={sizes}
          setSizes={setSizes}
        />
      </div>
      <div className="h-94/100">
        <Graphs
          editMode={editMode}
          graphData={graphData}
          setGraphData={setGraphData}
          sizes={sizes}
          setSizes={setSizes}
        />
      </div>
    </div>
  );
}
