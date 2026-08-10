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

export function AppSidebar({
  activePage,
  setActivePage,
}: {
  activePage: Page;
  setActivePage: (value: Page) => void;
}) {
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
              name: "Graph View",
              icon: IconTimeline,
              isActive: activePage == Page.Telemetry,
              onClick: () => {
                setActivePage(Page.Telemetry);
              },
            },
            {
              name: "Live Timings",
              icon: IconTrophy,
              isActive: activePage == Page.LiveTimings,
              onClick: () => {
                setActivePage(Page.LiveTimings);
              },
            },
          ]}
        />{" "}
        <NavSection
          name="Reflect"
          items={[
            {
              name: "Map View",
              icon: IconMap,
              isActive: activePage == Page.Setups,
              onClick: () => {
                setActivePage(Page.Setups);
              },
            },
          ]}
        />
      </SidebarContent>
      <SidebarFooter>
        <SidebarFooter>
          <NavUser user={{ name: "Karel Lukeš", avatar: "../../public/pfp-white.png" }} />
        </SidebarFooter>
      </SidebarFooter>
    </Sidebar>
  );
}
