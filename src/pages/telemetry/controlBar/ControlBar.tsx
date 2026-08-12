import { useContext } from "react";
import { TelemetryContext } from "../Telemetry";
import NormalLayout from "./normalLayout/NormalLayout";
import EditLayout from "./editLayout/EditLayout";
import { useEffect, useRef, type ReactNode } from "react";
// import { SidebarTrigger } from "@/components/ui/sidebar";
// import { Button } from "@/components/ui/button";
// import { IconRefresh } from "@tabler/icons-react";

export default function ControlBar() {
  const c = useContext(TelemetryContext);
  return (
    <div className=" bg-(--middleground) h-full w-full rounded-[24px] items-center flex justify-baseline overflow-x-auto no-scrollbar overflow-y-hidden">
      {/* <SidebarTrigger className="pl-2 size-10 text-[#138DF1] hover:text-[#138DF1] hover:bg-[#00000000]!" /> */}
      <ScrollX>{c.editMode ? <EditLayout /> : <NormalLayout />}</ScrollX>
      {/* <Button size={"icon"} className="size-8! hover:bg-[#00000000]!" variant={"ghost"}> */}
      {/*   <IconRefresh className="size-8 stroke-[1.5]" color="#138DF1" /> */}
      {/* </Button> */}
    </div>
  );
}

interface ScrollXProps {
  children: ReactNode;
}

function ScrollX({ children }: ScrollXProps) {
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const el = ref.current;
    if (!el) return;

    const onWheel = (e: WheelEvent) => {
      if (e.deltaY === 0) return;
      e.preventDefault();
      el.scrollLeft += e.deltaY;
    };

    el.addEventListener("wheel", onWheel, { passive: false });
    return () => el.removeEventListener("wheel", onWheel);
  }, []);

  return (
    <div
      ref={ref}
      className="flex overflow-x-auto overflow-y-hidden no-scrollbar h-full w-full justify-center items-center"
    >
      {children}
    </div>
  );
}
