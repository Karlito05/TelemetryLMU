import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuItem,
  SidebarMenuButton,
} from "@/components/ui/sidebar";
import {
  IconFlask,
  IconGraph,
  IconSettingsCog,
  IconStopwatch,
} from "@tabler/icons-react";
import { NavUser } from "./ui/nav-user";
import { NavSection } from "./ui/nav-section";
import { Page } from "@/App";

export function AppSidebar({
  activePage,
  setActivePage,
}: {
  activePage: Page;
  setActivePage: (value: Page) => void;
}) {
  return (
    <Sidebar>
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton
              asChild
              className="data-[slot=sidebar-menu-button]:p-1.5!"
            >
              <a href="#">
                <IconGraph className="size-5!" />
                <span className="text-base font-semibold">Telemetry LMU</span>
              </a>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>
      <SidebarContent>
        <NavSection
          name="Analyze"
          items={[
            {
              name: "Telemetry",
              icon: IconGraph,
              isActive: activePage == Page.Telemetry,
              onClick: () => {
                setActivePage(Page.Telemetry);
              },
            },
            {
              name: "Live Timings",
              icon: IconStopwatch,
              isActive: activePage == Page.LiveTimings,
              onClick: () => {
                setActivePage(Page.LiveTimings);
              },
            },
            {
              name: "Analysis",
              icon: IconFlask,
              isActive: activePage == Page.Analysis,
              onClick: () => {
                setActivePage(Page.Analysis);
              },
            },
          ]}
        />{" "}
        <NavSection
          name="Prepare"
          items={[
            {
              name: "Setups",
              icon: IconSettingsCog,
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
          <NavUser
            user={{ name: "Karlito", avatar: "", gameName: "Karel Lukes" }}
          />
        </SidebarFooter>
      </SidebarFooter>
    </Sidebar>
  );
}
