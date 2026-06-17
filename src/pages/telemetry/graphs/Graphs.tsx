import GraphView from "./GraphView";
import GraphViewDummy from "./GraphViewDummy";
import { Splitter } from "antd";
import { useContext } from "react";
import { TelemetryContext } from "../Telemetry";

export default function Graphs() {
  const c = useContext(TelemetryContext);
  return c.editMode ? (
    <Splitter vertical={true} className="w-full h-full" onResize={c.setSizes}>
      {c.graphData.map((data, i) => (
        <Splitter.Panel
          key={data.type}
          resizable={c.editMode}
          size={c.sizes[i]}
          min="10%"
          style={{ paddingTop: "0.25rem", paddingBottom: "0.25rem" }}
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
        </Splitter.Panel>
      ))}
    </Splitter>
  ) : (
    <Splitter vertical={true} className="w-full h-full" onResize={c.setSizes}>
      {c.graphData.map((data, i) => (
        <Splitter.Panel
          key={data.type}
          resizable={c.editMode}
          size={c.sizes[i]}
          style={
            i == 0
              ? { paddingBottom: "0.125rem" }
              : i == c.graphData.length - 1
                ? { paddingTop: "0.125rem" }
                : { paddingTop: "0.125rem", paddingBottom: "0.125rem" }
          }
        >
          <GraphView
            {...data}
            graphName={data.type.charAt(0).toUpperCase() + data.type.slice(1)}
            carNum={c.curDriverNum}
            componentStyle={
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
        </Splitter.Panel>
      ))}
    </Splitter>
  );
}
