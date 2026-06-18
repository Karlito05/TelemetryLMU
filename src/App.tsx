import Sidebar from "./components/Sidebar";
import Telemetry from "./pages/telemetry/Telemetry";
import { useState } from "react";
import { ConfigProvider, theme } from "antd";
import Titlebar from "./components/Titlebar";

export enum Page {
  Telemetry,
  LiveTimings,
  Analysis,
  Setups,
}

function App() {
  const [curPage, setCurPage] = useState<Page>(0);
  const [isOpen, setIsOpen] = useState(true);

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
      <main className="w-screen h-screen overflow-hidden bg-[#16171C]  rounded-3xl">
        <Titlebar />
        <div className="flex gap-3 h-full w-full p-2 pt-7">
          <div className={isOpen ? "w-1/7 min-w-80" : "w-3/100"}>
            <Sidebar
              activePage={curPage}
              onPageChange={(id) => setCurPage(id)}
              setIsOpen={setIsOpen}
              isOpen={isOpen}
            />
          </div>
          <div className="w-full">{PAGES[curPage]}</div>
        </div>
      </main>
    </ConfigProvider>
  );
}

export default App;
