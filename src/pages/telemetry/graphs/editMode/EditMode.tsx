import { ResizablePanel, ResizablePanelGroup } from "@/components/ui/resizable";
import GraphViewDummy from "./GraphViewDummy";
import { useContext } from "react";
import { TelemetryContext } from "../../Telemetry";
import { PanelSize } from "react-resizable-panels";

export default function EditMode() {
  const c = useContext(TelemetryContext);

  function handleResize(
    panelSize: PanelSize,
    id: string | number | undefined,
    _prevPanelSize: PanelSize | undefined,
  ) {
    let newSizes = c.sizes;
    newSizes[Number(id)] = panelSize.asPercentage.toString() + "%";
    c.setSizes(newSizes);
  }

  return (
    <ResizablePanelGroup orientation="vertical">
      {c.graphData.map((_, i) => (
        <ResizablePanel
          id={i.toString()}
          defaultSize={c.sizes[i]}
          minSize={"10%"}
          onResize={handleResize}
        >
          <GraphViewDummy
            index={i}
            style={
              i == 0
                ? {
                    borderTopLeftRadius: "24px",
                    borderTopRightRadius: "24px",
                  }
                : i == c.graphData.length - 1
                  ? {
                      borderBottomLeftRadius: "24px",
                      borderBottomRightRadius: "24px",
                    }
                  : {}
            }
          />
        </ResizablePanel>
      ))}
    </ResizablePanelGroup>
  );
}
