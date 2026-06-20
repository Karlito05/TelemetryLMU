import { Separator } from "@/components/ui/separator";
import DriverSelect from "./DriverSelect";
import LayoutSelect from "./LayoutSelect";

export default function NormalLayout() {
  return (
    <>
      <DriverSelect />
      <Separator orientation="vertical" className="mx-2 my-1" />
      <LayoutSelect />
    </>
  );
}
