import GraphViewDummy from "./GraphViewDummy";
import { useContext, useEffect, useRef, useState } from "react";
import { TelemetryContext } from "../Telemetry";
import { ResizablePanel, ResizablePanelGroup } from "@/components/ui/resizable";
import { PanelSize } from "react-resizable-panels";
import GraphViewNew from "./GraphViewNew";
import { DataPoint } from "./GraphView";
import { invoke } from "@tauri-apps/api/core";

export default function Graphs() {
  const c = useContext(TelemetryContext);
  const activeLayout = c.layouts[c.activeLayout];
  const currentLaps = useRef<{ type: string; data: DataPoint[] }[]>([]);
  const referenceLaps = useRef<{ type: string; data: DataPoint[] }[]>([]);
  const [curLap, setCurLap] = useState(0);
  const [telemetryInfos, setTelemetryInfos] = useState<
    { max_value: number; unit: string; graph_type: string }[]
  >([]);
  const [, setLapTick] = useState(0);

  function handleResize(
    panelSize: PanelSize,
    id: string | number | undefined,
    _prevPanelSize: PanelSize | undefined,
  ) {
    let newSizes = c.sizes;
    newSizes[Number(id)] = panelSize.asPercentage.toString() + "%";
    c.setSizes(newSizes);
  }

  useEffect(() => {
    // Init curLaps and curTypes
    let curTypes: string[] = [];
    let newCurrentLaps: { type: string; data: DataPoint[] }[] = [];
    activeLayout.graphData.map((data) => {
      curTypes.push(data.type.toString());
      newCurrentLaps.push({
        type: data.type.toString(),
        data: [],
      });
    });
    currentLaps.current = newCurrentLaps;

    invoke<{ max_value: number; unit: string; graph_type: string }[]>("get_telemetry_info", {
      carNum: c.curDriverNum,
      teleTypes: curTypes,
    }).then((val) => setTelemetryInfos(val));

    // utpdate tele
    setLapTick((tick) => tick + 1);

    const getData = setInterval(() => {
      // fetch data from backend
      invoke<{ values: number[]; distance: number; lap_num: number; graph_type: string }[]>(
        "get_lap_data",
        {
          carNum: c.curDriverNum,
          teleTypes: curTypes,
        },
      ).then((vals) => {
        if (vals[0].lap_num != curLap) {
          setCurLap(vals[0].lap_num);
          let wasLastBest;
          invoke<boolean>("was_last_best", { carNum: c.curDriverNum }).then((v) => {
            wasLastBest = v;
          });
          if (wasLastBest) {
            referenceLaps.current = currentLaps.current;
          }
          currentLaps.current.map((val) => {
            val.data = [];
          });
        }
        // Add data to the right spot
        vals.map((val) => {
          // Find the right spot
          let i = currentLaps.current.findIndex((cl) => {
            return cl.type == val.graph_type;
          });

          // Insert the data

          currentLaps.current[i].data.push({
            distance: val.distance,
            values: val.values,
          });
          setLapTick((tick) => tick + 1);
        });
        // console.log(currentLaps.current);
      });
    }, 1000 / c.sampleRate);

    return () => {
      clearInterval(getData);
    };
  }, [activeLayout, c.sampleRate]);

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
              currentLap={currentLaps.current.find((v) => v.type === data.type.toString())?.data}
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
