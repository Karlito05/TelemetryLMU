import Graphs from "./graphs/Graphs.tsx";
import ControlBar from "./controlBar/ControlBar.tsx";
import { useState, createContext } from "react";
import { GraphViewData, GraphViewType } from "./graphs/Graphs.tsx";

type TelemetryContextType = {
  curDriverNum: number;
  setCurDriverNum: (value: number) => void;
  editMode: boolean;
  setEditMode: (value: boolean) => void;
  graphData: GraphViewData[];
  setGraphData: (value: GraphViewData[]) => void;
  sizes: number[];
  setSizes: (value: number[]) => void;
};

const defaultTelemetryContext: TelemetryContextType = {
  curDriverNum: 0,
  setCurDriverNum: () => {},
  editMode: false,
  setEditMode: () => {},
  graphData: [],
  setGraphData: () => {},
  sizes: [0.5, 0.5],
  setSizes: () => {},
};

export const TelemetryContext = createContext<TelemetryContextType>(
  defaultTelemetryContext,
);

export default function Telemetry() {
  const [curDriverNum, setCurDriverNum] = useState(0);
  const [editMode, setEditMode] = useState(false);
  const [sizes, setSizes] = useState<number[]>([0.5, 0.5]);
  const [graphData, setGraphData] = useState<GraphViewData[]>([
    {
      baseColor: "#9eff5d",
      nLines: 3,
      type: GraphViewType.Throttle,
    },
    {
      baseColor: "#ff5d5d",
      nLines: 3,
      type: GraphViewType.Brake,
    },
  ]);

  return (
    <TelemetryContext.Provider
      value={{
        curDriverNum,
        setCurDriverNum,
        editMode,
        setEditMode,
        sizes,
        setSizes,
        graphData,
        setGraphData,
      }}
    >
      <div className=" h-full w-full">
        <div className="h-6/100 pb-0.5">
          <ControlBar />
        </div>
        <div className="h-94/100">
          <Graphs />
        </div>
      </div>
    </TelemetryContext.Provider>
  );
}
