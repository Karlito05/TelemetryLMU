import Telemetry from "@/pages/telemetry/Telemetry";
import { useState } from "react";
import { SidebarProvider } from "@/components/ui/sidebar";
import { AppSidebar } from "./components/AppSidebar";
import { ThemeProvider } from "./components/theme-provider";
import WorkInProgress from "./components/WorkInProgress";
import Titlebar from "./components/Titlebar";

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
    [Page.Analysis]: <WorkInProgress />,
    [Page.LiveTimings]: <WorkInProgress />,
    [Page.Setups]: <WorkInProgress />,
  };
  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <SidebarProvider>
        <AppSidebar activePage={curPage} setActivePage={setCurPage} />
        <main className="bg-background h-[100% Important!] w-full">
          {PAGES[curPage]}
        </main>
      </SidebarProvider>
    </ThemeProvider>
  );
}

export default App;
