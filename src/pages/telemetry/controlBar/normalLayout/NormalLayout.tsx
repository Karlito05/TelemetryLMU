import { Separator } from "@/components/ui/separator";
import DriverSelect from "./DriverSelect";
import LayoutSelect from "./LayoutSelect";
import { SidebarTrigger } from "@/components/ui/sidebar";
import ReferenceSelect from "./ReferenceSelect";

export default function NormalLayout() {
  return (
    <>
      <SidebarTrigger className="pl-2" />
      <Separator orientation="vertical" className="mx-2 my-1" />
      <DriverSelect />
      <Separator orientation="vertical" className="mx-2 my-1" />
      <LayoutSelect />
      {/* <Separator orientation="vertical" className="mx-2 my-1" />
      <Timing /> */}
      <Separator orientation="vertical" className="mx-2 my-1" />
      <ReferenceSelect />
    </>
  );
}
