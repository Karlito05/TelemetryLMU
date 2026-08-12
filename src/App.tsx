import Telemetry from "@/pages/telemetry/Telemetry";
import { createContext, useState } from "react";
import { SidebarProvider } from "@/components/ui/sidebar";
import { AppSidebar } from "./components/AppSidebar";
import { ThemeProvider } from "./components/theme-provider";
import WorkInProgress from "./components/WorkInProgress";
import { Toaster } from "sonner";
import { Page } from "./utils/Page.ts";
import Championships from "./pages/championships/Championships.tsx";

const PAGES = {
  [Page.Telemetry]: <Telemetry />,
  [Page.Championships]: <Championships />,
  [Page.LiveTimings]: <WorkInProgress />,
  [Page.Setups]: <WorkInProgress />,
};

type SettingsContextType = {
  name: string;
  gameName: string;
  setName: (v: string) => void;
  setGameName: (v: string) => void;
};

const defaultSettingsContext: SettingsContextType = {
  name: "",
  gameName: "",
  setName: () => {},
  setGameName: () => {},
};

export const SettingsContext = createContext(defaultSettingsContext);

function App() {
  const [curPage, setCurPage] = useState<Page>(0);
  const [open, setOpen] = useState(true);
  const [name, setName] = useState("Karel Lukes");
  const [gameName, setGameName] = useState("Karel Lukes");

  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <SidebarProvider open={open} onOpenChange={setOpen}>
        <SettingsContext
          value={{ name: name, gameName: gameName, setGameName: setGameName, setName: setName }}
        >
          <AppSidebar activePage={curPage} setActivePage={setCurPage} />{" "}
          <main
            className={`bg-background flex h-dvh min-h-0 w-full min-w-0 flex-1 overflow-hidden p-2 ${open ? "pl-0" : "pl-2"}`}
          >
            {PAGES[curPage]}
          </main>
        </SettingsContext>

        <Toaster theme="dark" />
      </SidebarProvider>
    </ThemeProvider>
  );
}

export default App;
