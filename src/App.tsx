import Sidebar from "@/components/Sidebar";
import Telemetry from "@/pages/telemetry/Telemetry";
import { useState } from "react";
import Titlebar from "@/components/Titlebar";
import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar";
import { AppSidebar } from "./components/AppSidebar";
import { ThemeProvider } from "./components/theme-provider";

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
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <SidebarProvider>
        <Titlebar />
        <AppSidebar activePage={curPage} setActivePage={setCurPage} />
        <main className="bg-background h-[100% Important!] w-full">
          {PAGES[curPage]}
        </main>
      </SidebarProvider>
    </ThemeProvider>
  );
}

export default App;
