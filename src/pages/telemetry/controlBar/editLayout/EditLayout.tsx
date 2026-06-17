import { useContext } from "react";
import { TelemetryContext } from "../../Telemetry";
import { Button } from "antd";
import { GraphViewType } from "../../store";

export default function EditLayout() {
  const c = useContext(TelemetryContext);
  function handleAddGraph() {
    let nGD = [...c.graphData];
    if (c.graphData.length < 10) {
      nGD = [
        ...c.graphData,
        {
          baseColor: "#ff5d5d",
          nLines: 3,
          type: GraphViewType.Brake,
        },
      ];
    } else {
      alert("Can't have more than 10 Graphs");
    }

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
