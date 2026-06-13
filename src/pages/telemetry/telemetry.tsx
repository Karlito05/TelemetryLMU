import GraphView from "./GraphView.tsx";
import ControlBar from "./ControlBar.tsx"

export default function Telemetry() {
  return (
    <div className=" h-full w-full">
      <div className="h-6/100 pb-0.5">
        <ControlBar />
      </div>
      <div className="h-94/100">
        <div className="h-1/5 p-1 ">
          <GraphView
            baseColor={"#9eff5d"}
            carNum={0}
            graphName="Throttle"
            nLines={3}
            type="throttle"
          />
        </div>
        <div className="h-1/5 p-1">
          <GraphView baseColor={"#ff5d5d"} carNum={0} graphName="Brake" nLines={3} type="brake" />
        </div>
        <div className="h-1/5 p-1">
          <GraphView baseColor={"#5db1ff"} carNum={0} graphName="Engine RPM" nLines={3} type="rpm" />
        </div>
        <div className="h-1/5 p-1">
          <GraphView baseColor={"#cc5dff"} carNum={0} graphName="Delta" nLines={3} type="delta" />
        </div>
        <div className="h-1/5 p-1">
          <GraphView baseColor={"#efff5d"} carNum={0} graphName="Speed" nLines={3} type="speed" />
        </div>
      </div>
    </div >
  );
}
