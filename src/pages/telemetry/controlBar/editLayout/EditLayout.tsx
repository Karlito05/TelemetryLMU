"use client";

import { useContext } from "react";
import { TelemetryContext } from "../../Telemetry";
import { Button } from "@/components/ui/button";
import { GraphViewType } from "../../store";
import { setLayouts } from "../../store";
import { toast } from "sonner";
import { Separator } from "@/components/ui/separator";

// Todo: Refactor this :)
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
      toast("Can't have more than 10 Graphs");
    }

    c.setGraphData(nGD);
    const newSizes: string[] = Array(nGD.length).fill((100 / nGD.length).toString() + "%");
    c.setSizes(newSizes);
  }
  function handleCancel() {
    c.setSizes([...c.layouts[c.activeLayout].scales]);
    c.setGraphData([...c.layouts[c.activeLayout].graphData]);
    c.setEditMode(false);
  }
  function handleSave() {
    const newLayouts = [...c.layouts];
    newLayouts[c.activeLayout] = {
      ...newLayouts[c.activeLayout],
      graphData: [...c.graphData],
      scales: [...c.sizes],
    };
    setLayouts(newLayouts);
    c.setLayouts(newLayouts);
    c.setEditMode(false);
  }
  function handleSaveAs() {
    const newLayouts = [...c.layouts];
    const name = prompt("How would you like this layout to be named: ");
    newLayouts.push({
      graphData: [...c.graphData],
      name: name ? name : "Layout " + c.layouts.length + 1,
      scales: [...c.sizes],
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
      toast("Can't remove layout.", {
        description: "You must have at least one more layout to remove this one.",
        action: {
          label: "Ok",
          onClick: () => {},
        },
      });
    }
  }
  return (
    <>
      <Button className="bg-[#138DF1]" onClick={handleSave}>
        Save
      </Button>
      <Separator orientation="vertical" className="mx-2 my-1" />
      <Button className="bg-[#138DF1]" onClick={handleSaveAs}>
        Save As
      </Button>
      <Separator orientation="vertical" className="mx-2 my-1" />
      <Button className="bg-[rgba(255,255,255,0.075)]" onClick={handleCancel}>
        Discard
      </Button>
      <Separator orientation="vertical" className="mx-2 my-1" />
      <Button className="bg-[#FF0000]" onClick={handleRemove}>
        Delete Layout
      </Button>
      <Separator orientation="vertical" className="mx-2 my-1" />
      <Button className="bg-green-600" onClick={() => handleAddGraph()}>
        Add Graph
      </Button>
      <Separator orientation="vertical" className="mx-2 my-1" />
    </>
  );
}
