import Telemetry from "@/pages/telemetry/Telemetry";
import { useState } from "react";
import { SidebarProvider } from "@/components/ui/sidebar";
import { AppSidebar } from "./components/AppSidebar";
import { ThemeProvider } from "./components/theme-provider";
import WorkInProgress from "./components/WorkInProgress";
import { Toaster } from "sonner";

export enum Page {
  Telemetry,
  LiveTimings,
  Analysis,
  Setups,
}

function App() {
  const [curPage, setCurPage] = useState<Page>(0);
  const [open, setOpen] = useState(true);
  const PAGES = {
    [Page.Telemetry]: <Telemetry />,
    [Page.Analysis]: <WorkInProgress />,
    [Page.LiveTimings]: <WorkInProgress />,
    [Page.Setups]: <WorkInProgress />,
  };
  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <SidebarProvider open={open} onOpenChange={setOpen}>
        <AppSidebar activePage={curPage} setActivePage={setCurPage} />

        <main className="bg-background flex h-dvh min-h-0 w-full min-w-0 flex-1 overflow-hidden p-2 ">
          {PAGES[curPage]}
        </main>
        <Toaster theme="dark" />
      </SidebarProvider>
    </ThemeProvider>
  );
}

export default App;
