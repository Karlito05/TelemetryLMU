import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuItem,
} from "@/components/ui/sidebar";
import { IconTimeline, IconMap, IconTrophy } from "@tabler/icons-react";
import { NavUser } from "./ui/nav-user";
import { NavSection } from "./ui/nav-section";
import { Page } from "../utils/Page.ts";
import { useContext } from "react";
import { SettingsContext } from "@/App.tsx";

export function AppSidebar({
  activePage,
  setActivePage,
}: {
  activePage: Page;
  setActivePage: (value: Page) => void;
}) {
  const s = useContext(SettingsContext);

  return (
    <Sidebar variant="floating" className="pl-2 py-2 ">
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <div className="flex p-2 justify-baseline items-center gap-4 text-2xl font-[Racing_Sans_One]">
              <img src="../../Logo.svg" alt="Logo" width={48} height={48} />
              <span>Telemetry LMU</span>
            </div>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>
      <SidebarContent>
        <NavSection
          name="Live Analysis"
          items={[
            {
              name: "Overview",
              icon: IconTrophy,
              isActive: activePage == Page.Championships,
              onClick: () => {
                setActivePage(Page.Championships);
              },
            },
            {
              name: "Live Graphs",
              icon: IconTimeline,
              isActive: activePage == Page.Telemetry,
              onClick: () => {
                setActivePage(Page.Telemetry);
              },
            },
          ]}
        />{" "}
        {/* <NavSection */}
        {/*   name="Reflect" */}
        {/*   items={[ */}
        {/*     { */}
        {/*       name: "Map View", */}
        {/*       icon: IconMap, */}
        {/*       isActive: activePage == Page.Setups, */}
        {/*       onClick: () => { */}
        {/*         setActivePage(Page.Setups); */}
        {/*       }, */}
        {/*     }, */}
        {/*   ]} */}
        {/* /> */}
      </SidebarContent>
      <SidebarFooter>
        <SidebarFooter>
          <NavUser user={{ name: s.name, avatar: "../../pfp-white.png" }} />
        </SidebarFooter>
      </SidebarFooter>
    </Sidebar>
  );
}
