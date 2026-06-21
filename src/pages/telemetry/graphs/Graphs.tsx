import GraphView from "./GraphView";
import GraphViewDummy from "./GraphViewDummy";
import { useContext } from "react";
import { TelemetryContext } from "../Telemetry";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import { PanelSize } from "react-resizable-panels";

export default function Graphs() {
  const c = useContext(TelemetryContext);
  const activeLayout = c.layouts[c.activeLayout];

  function handleResize(
    panelSize: PanelSize,
    id: string | number | undefined,
    _prevPanelSize: PanelSize | undefined,
  ) {
    let newSizes = c.sizes;
    newSizes[Number(id)] = panelSize.asPercentage.toString() + "%";
    c.setSizes(newSizes);
  }

  return c.editMode ? (
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
                    borderTopLeftRadius: "1.5rem",
                    borderTopRightRadius: "1.5rem",
                  }
                : i == c.graphData.length - 1
                  ? {
                      borderBottomLeftRadius: "1.5rem",
                      borderBottomRightRadius: "1.5rem",
                    }
                  : {}
            }
          />
          {c.graphData.length - 1 != i ? <ResizableHandle withHandle /> : <></>}
        </ResizablePanel>
      ))}
    </ResizablePanelGroup>
  ) : (
    <div className="h-full">
      {activeLayout.graphData.map((data, i) => {
        return (
          <div
            style={{
              height: activeLayout.scales[i],
              ...(i == 0
                ? { paddingBottom: "0.125rem" }
                : i == activeLayout.graphData.length - 1
                  ? { paddingTop: "0.125rem" }
                  : { paddingTop: "0.125rem", paddingBottom: "0.125rem" }),
            }}
          >
            <GraphView
              {...data}
              graphName={data.type.charAt(0).toUpperCase() + data.type.slice(1)}
              carNum={c.curDriverNum}
              // key={`${c.activeLayout}-${data.type}-${i}`}
              componentStyle={
                i == 0
                  ? {
                      borderTopLeftRadius: "1.5rem",
                      borderTopRightRadius: "1.5rem",
                    }
                  : i == activeLayout.graphData.length - 1
                    ? {
                        borderBottomLeftRadius: "1.5rem",
                        borderBottomRightRadius: "1.5rem",
                      }
                    : {}
              }
            />
          </div>
        );
      })}
    </div>
  );
}
