import Graphs from "./graphs/Graphs.tsx";
import ControlBar from "./controlBar/ControlBar.tsx";
import { useState, createContext, useEffect } from "react";
import { getLayouts, GraphViewData, GraphViewType, Layouts } from "./store.ts";
import { invoke } from "@tauri-apps/api/core";
import { SaveData } from "./controlBar/normalLayout/ReferenceSelect.tsx";

type TelemetryContextType = {
  curDriverNum: number;
  setCurDriverNum: (value: number) => void;
  editMode: boolean;
  setEditMode: (value: boolean) => void;
  graphData: GraphViewData[];
  setGraphData: (value: GraphViewData[]) => void;
  sizes: string[];
  setSizes: (value: string[]) => void;
  layouts: Layouts[];
  setLayouts: (value: Layouts[]) => void;
  activeLayout: number;
  setActiveLayout: (value: number) => void;
  activeReference: SaveData | null;
  setActiveRefence: (value: SaveData | null) => void;
};

const defaultTelemetryContext: TelemetryContextType = {
  curDriverNum: 0,
  setCurDriverNum: () => {},
  editMode: false,
  setEditMode: () => {},
  graphData: [],
  setGraphData: () => {},
  sizes: ["50%", "50%"],
  setSizes: () => {},
  layouts: [],
  setLayouts: () => {},
  activeLayout: 0,
  setActiveLayout: () => {},
  activeReference: null,
  setActiveRefence: () => {},
};

export const TelemetryContext = createContext<TelemetryContextType>(
  defaultTelemetryContext,
);

export default function Telemetry() {
  const [curDriverNum, setCurDriverNum] = useState(0);
  const [editMode, setEditMode] = useState(false);
  const [sizes, setSizes] = useState(["50%", "50%"]);
  const [activeReference, setActiveRefence] = useState<SaveData | null>(null);
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
  const [layouts, setLayouts] = useState<Layouts[]>([
    { name: "Default", scales: sizes, graphData: graphData },
  ]);

  useEffect(() => {
    getLayouts().then((v) => {
      if (v) setLayouts(v);
    });
  }, []);

  const [activeLayout, setActiveLayout] = useState<number>(0);

  useEffect(() => {
    invoke("spawn_logger", { carNum: curDriverNum });
    return () => {
      invoke("despawn_logger");
    };
  }, [curDriverNum]);

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
        layouts,
        setLayouts,
        activeLayout,
        setActiveLayout,
        activeReference,
        setActiveRefence,
      }}
    >
      <div className="flex h-full min-h-0px w-full flex-col gap-2">
        <div className="shrink-0 min-h-10 pb-1">
          <ControlBar />
        </div>
        <div className="min-h-0 flex-1">
          <Graphs />
        </div>
      </div>
    </TelemetryContext.Provider>
  );
}
