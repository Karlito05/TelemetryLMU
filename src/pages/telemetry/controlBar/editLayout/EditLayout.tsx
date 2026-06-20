import { useContext } from "react";
import { TelemetryContext } from "../../Telemetry";
import { Button } from "@/components/ui/button";
import { GraphViewType } from "../../store";
import { setLayouts } from "../../store";

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
  function handleCancel() {
    c.setSizes(c.layouts[c.activeLayout].scales);
    c.setGraphData(c.layouts[c.activeLayout].graphData);
    c.setEditMode(false);
  }
  function handleSave() {
    let newLayouts = c.layouts;
    newLayouts[c.activeLayout].graphData = c.graphData;
    newLayouts[c.activeLayout].scales = c.sizes;
    setLayouts(newLayouts);
    c.setLayouts(newLayouts);
    c.setEditMode(false);
  }
  function handleSaveAs() {
    let newLayouts = c.layouts;
    const name = prompt("How would you like this layout to be named: ");
    newLayouts.push({
      graphData: c.graphData,
      name: name ? name : "Layout " + c.layouts.length + 1,
      scales: c.sizes,
    });
    setLayouts(newLayouts);
    c.setLayouts(newLayouts);
    c.setEditMode(false);
  }

  function handleRemove() {
    if (c.layouts.length > 1) {
      const newLayouts = [
        ...c.layouts.slice(0, c.activeLayout),
        ...c.layouts.slice(c.activeLayout + 1),
      ];
      setLayouts(newLayouts);
      c.setActiveLayout(0);
      c.setLayouts(newLayouts);
      c.setEditMode(false);
    } else {
      alert(
        "Can't remove layout. You must have at least one more layout to remove this one.",
      );
    }
  }
  return (
    <>
      <Button onClick={handleCancel}>Cancel</Button>
      <Button onClick={handleSave}>Save</Button>
      <Button onClick={handleSaveAs}>Save As</Button>
      <Button onClick={() => handleAddGraph()}>Add Graph</Button>
      <Button onClick={handleRemove}>Remove</Button>
    </>
  );
}
