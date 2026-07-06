import GraphViewDummy from "./GraphViewDummy";
import { useContext } from "react";
import { TelemetryContext } from "../Telemetry";
import { ResizablePanel, ResizablePanelGroup } from "@/components/ui/resizable";
import { PanelSize } from "react-resizable-panels";
import GraphViewNew from "./GraphViewNew";

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
                    borderTopLeftRadius: "1rem",
                    borderTopRightRadius: "1rem",
                  }
                : i == c.graphData.length - 1
                  ? {
                      borderBottomLeftRadius: "1rem",
                      borderBottomRightRadius: "1rem",
                    }
                  : {}
            }
          />
        </ResizablePanel>
      ))}
    </ResizablePanelGroup>
  ) : (
    <div className="h-full">
      {activeLayout.graphData.map((data, i) => {
        let refData = undefined;
        if (c.activeReference) {
          refData = c.activeReference.data;
          refData = refData.filter((val) => {
            return val.data_type == data.type.toString();
          });
          console.log(refData);
        }
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
            <GraphViewNew
              style={{ color: data.baseColor, gridlines: data.nLines }}
              componentStyle={
                i == 0
                  ? {
                      borderTopLeftRadius: "1rem",
                      borderTopRightRadius: "1rem",
                    }
                  : i == activeLayout.graphData.length - 1
                    ? {
                        borderBottomLeftRadius: "1rem",
                        borderBottomRightRadius: "1rem",
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
