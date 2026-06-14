import GraphView from "./GraphView";
import { Splitter } from "antd";
import { useState } from "react";

type GraphsProps = {
  curDriverNum: number;
};

enum GraphViewType {
  Throttle = "throttle",
  Brake = "brake",
  Rpm = "rpm",
  Delta = "delta",
  Speed = "speed",
}

type GraphViewData = {
  baseColor: string;
  carNum: number;
  graphName: string;
  nLines: number;
  type: GraphViewType;
};

export default function Graphs({ curDriverNum }: GraphsProps) {
  const [sizes, setSizes] = useState<(number | string)[]>(["25%", "75%"]);
  const [enabled, setEnabled] = useState(true);
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
    <Splitter vertical={true} className="w-full h-full" onResize={setSizes}>
      {graphData.map((data, i) => (
        <Splitter.Panel key={data.type} resizable={enabled} size={sizes[i]}>
          <GraphView {...data} />
        </Splitter.Panel>
      ))}
    </Splitter>
  );
}
