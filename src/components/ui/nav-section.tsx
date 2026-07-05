"use client";

import { type Icon } from "@tabler/icons-react";

import {
  SidebarGroup,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";

export function NavSection({
  name,
  items,
}: {
  name: string;
  items: {
    name: string;
    onClick: () => void;
    icon: Icon;
    isActive: boolean;
  }[];
}) {
  return (
    <SidebarGroup className="group-data-[collapsible=icon]:hidden">
      <SidebarGroupLabel className="font-[Days_One] text-[12px] color-[#FFFFFF80]">
        {name}
      </SidebarGroupLabel>
      <SidebarMenu>
        {items.map((item) => (
          <SidebarMenuItem key={item.name}>
            <SidebarMenuButton
              asChild
              onClick={item.onClick}
              isActive={item.isActive}
              className="rounded-[8px]"
            >
              <a>
                <item.icon color="#138DF1" className="size-8! stroke-[1.5]" />
                <span className="text-base">{item.name}</span>
              </a>
            </SidebarMenuButton>
          </SidebarMenuItem>
        ))}
      </SidebarMenu>
    </SidebarGroup>
  );
}
