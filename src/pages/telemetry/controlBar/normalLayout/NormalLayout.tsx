import { Separator } from "@/components/ui/separator";
import DriverSelect from "./DriverSelect";
import LayoutSelect from "./LayoutSelect";

export default function NormalLayout() {
  return (
    <>
      <DriverSelect />
      <Separator />
      <LayoutSelect />
    </>
  );
}
