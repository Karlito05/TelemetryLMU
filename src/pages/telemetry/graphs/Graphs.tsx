import GraphView from "./GraphView";
import GraphViewDummy from "./GraphViewDummy";
import { Splitter } from "antd";
import { useContext } from "react";
import { TelemetryContext } from "../Telemetry";
import { GraphViewType } from "./graphTypes";

export type GraphViewData = {
  baseColor: string;
  // graphName: string;
  nLines: number;
  type: GraphViewType;
};

export default function Graphs() {
  const c = useContext(TelemetryContext);
  return c.editMode ? (
    <Splitter vertical={true} className="w-full h-full" onResize={c.setSizes}>
      {c.graphData.map((data, i) => (
        <Splitter.Panel
          key={data.type}
          resizable={c.editMode}
          size={c.sizes[i]}
          min="10%"
        >
          <GraphViewDummy index={i} />
        </Splitter.Panel>
      ))}
    </Splitter>
  ) : (
    <Splitter vertical={true} className="w-full h-full" onResize={c.setSizes}>
      {c.graphData.map((data, i) => (
        <Splitter.Panel
          key={data.type}
          resizable={c.editMode}
          size={c.sizes[i]}
        >
          <GraphView
            {...data}
            graphName={data.type.charAt(0).toUpperCase() + data.type.slice(1)}
            carNum={c.curDriverNum}
          />
        </Splitter.Panel>
      ))}
    </Splitter>
  );
}
