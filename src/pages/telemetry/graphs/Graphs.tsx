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
  const curLap = useRef(0);
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
    // Ensure reference laps is initialized with the same structure
    referenceLaps.current = newCurrentLaps.map((l) => ({ type: l.type, data: [] }));

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
        if (vals.length === 0) {
          return;
        }

        if (vals[0].lap_num != curLap.current) {
          curLap.current = vals[0].lap_num;
          const finishedLap = currentLaps.current.map((lap) => ({
            type: lap.type,
            data: [...lap.data],
          }));
          invoke<boolean>("was_last_best", { carNum: c.curDriverNum }).then((wasLastBest) => {
            if (wasLastBest) {
              if (!c.activeReference) referenceLaps.current = finishedLap;
            }
          });
          currentLaps.current.forEach((val) => {
            val.data = [];
          });
        }
        // Add data to the right spot
        vals.map((val) => {
          // Find the right spot
          let i = currentLaps.current.findIndex((cl) => {
            return cl.type == val.graph_type;
          });

          if (i === -1) {
            return;
          }

          // Insert the data

          currentLaps.current[i].data.push({
            distance: val.distance,
            values: val.values,
          });
          setLapTick((tick) => tick + 1);
        });
        // console.log(currentLaps.current);

        if (c.activeReference) {
          c.activeReference.data.forEach((v, i) => {
            if (!referenceLaps.current[i]) {
              referenceLaps.current[i] = { type: v.data_type, data: [] };
            }
            referenceLaps.current[i].data = [...v.data];
            referenceLaps.current[i].type = v.data_type;
          });
        }
      });
    }, 1000 / c.sampleRate); //TODO: make this into a setting

    return () => {
      clearInterval(getData);
    };
  }, [activeLayout, c.curDriverNum, c.sampleRate, c.activeReference]);

  return c.editMode ? (
    <EditMode
      currentLaps={currentLaps.current}
      referenceLaps={referenceLaps.current}
      telemetryInfos={telemetryInfos}
    />
  ) : (
    <NormalMode
      currentLaps={currentLaps.current}
      referenceLaps={referenceLaps.current}
      telemetryInfos={telemetryInfos}
    />
  );
}
