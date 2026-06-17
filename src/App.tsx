import Sidebar from "./components/Sidebar";
import Telemetry from "./pages/telemetry/Telemetry";
import { useState } from "react";
import { ConfigProvider, theme } from "antd";

export enum Page {
  Telemetry,
  LiveTimings,
  Analysis,
  Setups,
}

function App() {
  const [curPage, setCurPage] = useState<Page>(0);

  const PAGES = {
    [Page.Telemetry]: <Telemetry />,
    [Page.Analysis]: <div />,
    [Page.LiveTimings]: <div />,
    [Page.Setups]: <div />,
  };
  return (
    <ConfigProvider
      theme={{
        algorithm: theme.darkAlgorithm,
      }}
    >
      <main className="w-screen h-screen overflow-hidden bg-[#16171C] p-2">
        <div className="flex gap-3 h-full">
          <div className="w-1/5 min-w-80">
            <Sidebar
              activePage={curPage}
              onPageChange={(id) => setCurPage(id)}
            />
          </div>
          <div className="h-full w-full">{PAGES[curPage]}</div>
        </div>
      </main>
    </ConfigProvider>
  );
}

export default App;
