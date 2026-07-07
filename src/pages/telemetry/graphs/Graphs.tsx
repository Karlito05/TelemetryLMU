import { useContext, useEffect, useRef, useState } from "react";
import { TelemetryContext } from "../Telemetry";
import { DataPoint } from "./normalMode/GraphView";
import { invoke } from "@tauri-apps/api/core";
import NormalMode from "./normalMode/NormalMode";
import EditMode from "./editMode/EditMode";

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
    }, 1000 / c.sampleRate); //TODO: make this into a setting

    return () => {
      clearInterval(getData);
    };
  }, [activeLayout]);

  return c.editMode ? (
    <EditMode />
  ) : (
    <NormalMode
      currentLaps={currentLaps.current}
      referenceLaps={referenceLaps.current}
      telemetryInfos={telemetryInfos}
    />
  );
}
