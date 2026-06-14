import { useContext } from "react";
import { TelemetryContext } from "../../Telemetry";
import { GraphViewType } from "../../graphs/graphTypes";
import { Button } from "antd";

export default function EditLayout() {
  const c = useContext(TelemetryContext);
  function handleAddGraph() {
    const nGD = [
      ...c.graphData,
      {
        baseColor: "#ff5d5d",
        carNum: c.curDriverNum,
        graphName: "Brake",
        nLines: 3,
        type: GraphViewType.Brake,
      },
    ];
    c.setGraphData(nGD);
    const newSizes: number[] = Array(nGD.length).fill(1 / nGD.length);
    c.setSizes(newSizes);
  }
  return (
    <>
      <Button onClick={() => c.setEditMode(false)}>Quit Edit Mode</Button>
      <Button onClick={() => handleAddGraph()}>Add Graph</Button>
    </>
  );
}
