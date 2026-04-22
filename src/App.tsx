import Sidebar from "./Sidebar";
import Telemetry from "./telemetry";
import { useState } from "react";

export enum Page {
  Telemetry,
  LiveTimings,
  Analysis,
  Setups,
}

function App() {
  let [curPage, setCurPage] = useState<Page>(0);

  const PAGES = {
    [Page.Telemetry]: <Telemetry />,
    [Page.Analysis]: <div />,
    [Page.LiveTimings]: <div />,
    [Page.Setups]: <div />,
  };
  return (
    <main className="w-screen h-screen overflow-hidden bg-[#16171C]">
      <div className="flex gap-3 h-full">
        <div className="w-1/5 min-w-80">
          <Sidebar activePage={curPage} onPageChange={(id) => setCurPage(id)} />
        </div>
        <div className="h-full w-full">{PAGES[curPage]}</div>
      </div>
    </main>
  );
}

export default App;
