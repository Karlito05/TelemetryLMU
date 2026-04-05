import GraphView from "./GraphView";

function App() {
    return (
        <main className="w-screen h-screen overflow-hidden bg-[#16171C]">
            <div className="w-full h-1/4">
                <GraphView baseColor={"#9eff5d"} carNum={0} graphName="Throttle" nLines={3} type="throttle" />
                <GraphView baseColor={"#ff5d5d"} carNum={0} graphName="Brake" nLines={3} type="brake" />
                <GraphView baseColor={"#5dd1ff"} carNum={0} graphName="Engine RPM" nLines={3} type="rpm" />
                <GraphView baseColor={"#e45dff"} carNum={0} graphName="Delta" nLines={3} type="delta" />
            </div>
        </main>
    );
}

export default App;
