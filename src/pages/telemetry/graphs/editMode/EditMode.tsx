import { ResizablePanel, ResizablePanelGroup } from "@/components/ui/resizable";
import GraphViewDummy from "./GraphViewDummy";
import { useContext } from "react";
import { TelemetryContext } from "../../Telemetry";
import { DataPoint } from "../normalMode/GraphView";
import GraphViewNew from "../normalMode/GraphViewNew";

export default function EditMode({
  currentLaps,
  referenceLaps,
  telemetryInfos,
}: {
  currentLaps: { type: string; data: DataPoint[] }[];
  referenceLaps: { type: string; data: DataPoint[] }[];
  telemetryInfos: { max_value: number; unit: string; graph_type: string }[];
}) {
  const c = useContext(TelemetryContext);

  return (
    <ResizablePanelGroup
      orientation="vertical"
      className=" h-full w-full"
      onLayoutChanged={(layout) =>
        c.setSizes(c.graphData.map((_, i) => `${layout[i.toString()]}%`))
      }
    >
      {c.graphData.map((_, i) => (
        <ResizablePanel
          key={i}
          id={i.toString()}
          defaultSize={Number(c.sizes[i].replace("%", ""))}
          minSize={"10%"}
          className="relative w-full h-full"
        >
          <GraphViewDummy
            index={i}
            style={{
              position: "absolute",
              top: 0,
              left: 0,
              ...(i == 0
                ? {
                    borderTopLeftRadius: "24px",
                    borderTopRightRadius: "24px",
                  }
                : i == c.graphData.length - 1
                  ? {
                      borderBottomLeftRadius: "24px",
                      borderBottomRightRadius: "24px",
                    }
                  : {}),
            }}
          />
          <div className="h-full w-full blur-sm">
            <EditModeGraphViewNew
              currentLaps={currentLaps}
              referenceLaps={referenceLaps}
              i={i}
              telemetryInfos={telemetryInfos}
            />
          </div>
        </ResizablePanel>
      ))}
    </ResizablePanelGroup>
  );
}

function EditModeGraphViewNew({
  currentLaps,
  referenceLaps,
  telemetryInfos,
  i,
}: {
  currentLaps: { type: string; data: DataPoint[] }[];
  referenceLaps: { type: string; data: DataPoint[] }[];
  telemetryInfos: { max_value: number; unit: string; graph_type: string }[];
  i: number;
}) {
  const c = useContext(TelemetryContext);
  const activeLayout = c.layouts[c.activeLayout];
  let info = telemetryInfos.find((v) => v.graph_type === c.graphData[i].type.toString());

  return (
    <GraphViewNew
      style={{ color: c.graphData[i].baseColor, gridlines: c.graphData[i].nLines }}
      currentLap={currentLaps.find((v) => v.type === c.graphData[i].type.toString())?.data}
      referenceLap={referenceLaps.find((v) => v.type === c.graphData[i].toString())?.data}
      telemetryInfo={
        info ? { maxVal: info.max_value, type: info.graph_type, unit: info.unit } : undefined
      }
      componentStyle={
        i == 0
          ? {
              borderTopLeftRadius: "24px",
              borderTopRightRadius: "24px",
            }
          : i == activeLayout.graphData.length - 1
            ? {
                borderBottomLeftRadius: "24px",
                borderBottomRightRadius: "24px",
              }
            : { width: "100%", height: "100%" }
      }
    />
  );
}
