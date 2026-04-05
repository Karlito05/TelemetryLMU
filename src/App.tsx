import GraphView from "./GraphView";

function App() {
    return (
        <main className="w-screen h-screen overflow-hidden bg-[#16171C]">
            <div className="w-full h-1/3">
                <GraphView baseColor={"#9eff5d"} carNum={0} graphName="Throttle" nLines={3} type="throttle" />
                <GraphView baseColor={"#ff5d5d"} carNum={0} graphName="Brake" nLines={3} type="brake" />
                <GraphView baseColor={"#5dd1ff"} carNum={0} graphName="Engine RPM" nLines={3} type="rpm" />
            </div>
        </main>
    );
}

export default App;
