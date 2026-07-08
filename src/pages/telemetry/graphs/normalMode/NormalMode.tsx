import { useContext } from "react";
import { TelemetryContext } from "../../Telemetry";
import GraphViewNew, { DataPoint } from "./GraphViewNew";

export default function NormalMode({
  currentLaps,
  referenceLaps,
  telemetryInfos,
}: {
  currentLaps: { type: string; data: DataPoint[] }[];
  referenceLaps: { type: string; data: DataPoint[] }[];
  telemetryInfos: { max_value: number; unit: string; graph_type: string }[];
}) {
  const c = useContext(TelemetryContext);
  const activeLayout = c.layouts[c.activeLayout];

  return (
    <div className="h-full w-full">
      {activeLayout.graphData.map((data, i) => {
        let refData = undefined;
        if (c.activeReference) {
          refData = c.activeReference.data;
          refData = refData.filter((val) => {
            return val.data_type == data.type.toString();
          });
          console.log(refData);
        }
        let info = telemetryInfos.find((v) => v.graph_type === data.type.toString());
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
              currentLap={currentLaps.find((v) => v.type === data.type.toString())?.data}
              referenceLap={referenceLaps.find((v) => v.type === data.type.toString())?.data}
              telemetryInfo={
                info
                  ? { maxVal: info.max_value, type: info.graph_type, unit: info.unit }
                  : undefined
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
                    : {}
              }
            />
          </div>
        );
      })}
    </div>
  );
}
